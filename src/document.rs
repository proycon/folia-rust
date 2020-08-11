use std::path::{Path};
use std::io::BufRead;
use std::io::BufReader;
use std::io::Cursor;
use std::fs::File;
use std::str;
use std::str::FromStr;
use std::borrow::Cow;
use std::string::ToString;
use std::collections::HashMap;

use quick_xml::{Reader,Writer};
use quick_xml::events::{Event,BytesStart,BytesEnd,BytesText};

use crate::common::*;
use crate::types::*;
use crate::error::*;
use crate::element::*;
use crate::attrib::*;
use crate::store::*;
use crate::elementstore::*;
use crate::metadata::*;
use crate::select::*;
use crate::query::*;
use crate::serialiser::*;
use crate::parser::*;
use crate::specification::*;

///Represents a FoLiA document, owns all data
pub struct Document {
    ///The ID of the document
    pub id: String,
    ///The FoLiA version of the document
    pub version: String,
    pub filename: Option<String>,
    ///The element store holds and owns all elements in a document
    pub elementstore: ElementStore,
    ///The provenance store holds and owns all processors and a representation of the  provenance chain
    pub provenancestore: ProvenanceStore,
    ///The declaration store holds all annotation declarations
    pub declarationstore: DeclarationStore,
    ///Metadata consists of a simple key/value store (or a reference to external metadata)
    pub metadata: Metadata,
    ///Submetadata
    pub submetadata: HashMap<String,Metadata>,

    pub autodeclare: bool,
}


#[derive(Clone)]
///Properties for document parsing/instantiation/serialisation/handling.
pub struct DocumentProperties {
    pub bodytype: BodyType,
    pub autodeclare: bool,
    pub declare: Vec<(AnnotationType,Option<String>)>
}

impl Default for DocumentProperties {
    fn default() -> Self {
        Self {
            bodytype: BodyType::Text,
            autodeclare: true,
            declare: vec![(AnnotationType::TEXT, Some(DEFAULT_TEXT_SET.to_string()) )]
        }
    }
}

impl Document {
    ///Create a new FoLiA document from scratch
    pub fn new(id: &str, properties: DocumentProperties) -> Result<Self, FoliaError> {
        let mut document = Self {
            id: id.to_string(),
            filename: None,
            version: FOLIAVERSION.to_string(),
            elementstore: ElementStore::default(),
            provenancestore:  ProvenanceStore::default(),
            declarationstore: DeclarationStore::default(),
            metadata: Metadata::default(),
            submetadata: HashMap::default(),
            autodeclare: properties.autodeclare,
        };
        let mut body = match properties.bodytype {
            BodyType::Text => ElementData::new(ElementType::Text),
            BodyType::Speech => ElementData::new(ElementType::Speech),
        };
        body = document.encode(body, None)?;
        debug_assert!(!body.encodable());
        document.add(body, None)?;
        document.apply_properties(properties)?;
        Ok(document)
    }

    pub fn apply_properties(&mut self, properties: DocumentProperties) -> Result<(),FoliaError> {
        for (annotationtype, set) in properties.declare.iter() {
            let dec_key = self.declare(*annotationtype, &set, &None,&None)?;
            if set.is_some() {
                if set.as_ref().unwrap() == DEFAULT_TEXT_SET {
                     self.add_class(dec_key, &"current".to_string())?;
                }
            }
        }
        Ok(())
    }

    ///Load a FoliA document from file. Invokes the XML parser and loads it all into memory.
    pub fn from_file(filename: &str, properties: DocumentProperties) -> Result<Self, FoliaError> {
        let mut reader = Reader::from_file(Path::new(filename))?;
        reader.trim_text(false);
        let mut doc = Self::parse(&mut reader, properties)?;
        //associate the filename with the document
        doc.filename = Some(filename.to_string());
        Ok(doc)
    }

    ///Load a FoliA document from XML string representation, loading it all into memory.
    pub fn from_str(data: &str, properties: DocumentProperties) -> Result<Self, FoliaError> {
        let mut reader = Reader::from_str(data);
        reader.trim_text(false);
        Self::parse(&mut reader, properties)
    }





    ///Returns the ID of the document
    pub fn id(&self) -> &str { &self.id }

    ///Returns the filename associated with this document (i.e. the file from which it was loaded)
    pub fn filename(&self) -> Option<&str> { self.filename.as_ref().map(String::as_str) } //String::as_str equals  |x| &**x




    ///Get properties from the specification (a shortcut)
    pub fn props(&self, elementtype: ElementType) -> &Properties {
        self.elementstore.specification.get(elementtype)
    }

    //************** Low-level methods providing easy write access into the underlying Stores ********************


    ///Add an element to the document (but the element will be an orphan unless it is the very
    ///first one, you may want to use ``add_element_to`` or ``annotate`` instead)
    pub fn add_element(&mut self, element: ElementData) -> Result<ElementKey, FoliaError> {
        let (element,_) = self.add_children(element)?;
        <Self as Store<ElementData,ElementKey>>::add(self, element, None)
    }

    ///Add a declaration. It is strongly recommended to use ``declare()`` instead
    ///because this one adds a declaration without any checks.
    ///Returns the key.
    pub fn add_declaration(&mut self, declaration: Declaration) -> Result<DecKey, FoliaError> {
        <Self as Store<Declaration,DecKey>>::add(self, declaration, None)
    }

    ///Add an processor the document (but the processor will be an orphan and not in the processor
    ///chain!). You may want to use ``add_processor()`` instead to add to the provenance chain or
    ///``add_subprocessor()`` to add a processor as a subprocessor.
    pub fn add_provenance(&mut self, processor: Processor) -> Result<ProcKey, FoliaError> {
        <Self as Store<Processor,ProcKey>>::add(self, processor, None)
    }

    //************** Mid-level methods for adding things ********************

    ///Adds a new element as a child of another, this is a higher-level function that/
    ///takes care of adding and attaching for you. You may want to use ``annotate`` instead
    ///as that is an even higher-level function.
    pub fn add_element_to(&mut self, parent_key: ElementKey, element: ElementData) -> Result<ElementKey, FoliaError> {
        let (element,added_subelements) = self.add_children(element)?;
        match <Self as Store<ElementData,ElementKey>>::add(self, element, Some(parent_key)) {
            Ok(child_key) => {
                if let Some(added_subelements) = added_subelements {
                    for subchild_key in added_subelements.iter() {
                        if let Some(subchilddata) = self.get_mut_elementdata(*subchild_key) {
                            subchilddata.set_parent_key(Some(child_key));
                        }
                        self.post_add(*subchild_key, None)?;
                    }
                }
                self.attach_element(parent_key, child_key)?;
                self.post_add(child_key, None)?;
                Ok(child_key)
            },
            Err(err) => {
                Err(FoliaError::InternalError(format!("Unable to add element to parent: {}", err)))
            }
        }
    }

    ///Before we can add an element, we need to create and add its hitherto 'unborn' children.
    pub(crate) fn add_children(&mut self, mut element: ElementData) -> Result<(ElementData, Option<Vec<ElementKey>>),FoliaError> {
        let mut has_unborn_children = false;
        for child in element.data.iter() {
            if let DataType::AddElement(_) = child {
                has_unborn_children = true;
            }
        }
        if has_unborn_children {
            let mut added_elements: Vec<ElementKey> = Vec::new();
            let mut new_data: Vec<DataType> = Vec::new();
            for child in element.data {
                if let DataType::AddElement(child_elementdata) = child {
                    //first we do a recursion step to add the grandchildren, if any
                    let child_key = match self.add_children(child_elementdata) {
                        Ok((child_elementdata_new, Some(added_grandchildren))) => {
                            let child_key = <Self as Store<ElementData,ElementKey>>::add(self, child_elementdata_new, None)?;
                            for grandchild_key in added_grandchildren {
                                if let Some(subchilddata) = self.get_mut_elementdata(grandchild_key) {
                                    subchilddata.set_parent_key(Some(child_key));
                                }
                                self.post_add(grandchild_key, None)?;
                            }
                            child_key
                        },
                        Ok((child_elementdata_new, None)) => {
                            let child_key = <Self as Store<ElementData,ElementKey>>::add(self, child_elementdata_new, None)?;
                            child_key
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    };
                    new_data.push(DataType::Element(child_key));
                    added_elements.push(child_key);
                } else {
                    new_data.push(child);
                }
            }
            element.data = new_data;
            Ok((element, Some(added_elements)))
        } else {
            Ok((element, None))
        }
    }

    ///Adds the child element to the parent element, automatically takes care
    ///of removing the old parent (if any).
    pub fn attach_element(&mut self, parent_key: ElementKey, child_key: ElementKey) -> Result<(),FoliaError> {
        //ensure the parent exists
        if !self.get(parent_key).is_some() {
            return Err(FoliaError::InternalError(format!("Parent element does not exist: {}", parent_key)));
        };

        let oldparent_key = if let Some(child) = self.get_mut(child_key) {
            //add the new parent and return the old parent
            let tmp = child.parent_key();
            child.set_parent_key(Some(parent_key));
            tmp
        } else {
            //child does not exist
            return Err(FoliaError::InternalError(format!("Child does not exist: {}", child_key)));
        };

        if let Some(parent) = self.get_mut(parent_key) {
            parent.push(DataType::Element(child_key));
        }

        if let Some(oldparent_key) = oldparent_key {
            //detach child from the old parent
            if let Some(oldparent) = self.get_mut(oldparent_key) {
                if let Some(index) = oldparent.index(&DataType::Element(child_key)) {
                    oldparent.remove(index);
                }
            }
        }
        Ok(())
    }

    ///Performs postprocessing after adding an element
    pub(crate) fn post_add(&mut self, element_key: ElementKey, stack: Option<&Vec<ElementKey>>) -> Result<(),FoliaError> {
        let mut add_attributes: Option<Vec<Attribute>> = None;
        if let Some(element) = self.get_elementdata(element_key) {
            match element.elementtype {
                ElementType::WordReference => {
                    //We have a wref element, add SpanReference backpointers in the element
                    //that is being pointed at

                    //Find the element that is being pointed at by ID
                    let mut idref: Option<String> = None;
                    for attrib in element.attribs.iter() {
                        if let Attribute::Idref(id) = attrib {
                            idref = Some(id.clone())
                        }
                    }
                    if let Some(idref) = idref {
                        let mut span_key: Option<ElementKey> = None;
                        let selector = Selector::elements().elementgroup(Cmp::Is(ElementGroup::Span));
                        //we get our ancestors the normal way
                        if stack.is_some() {
                            for key in stack.unwrap().iter().rev() {
                                if selector.matches(&self, &DataType::Element(*key)) {
                                    span_key = Some(*key);
                                    break;
                                }
                            }
                        } else {
                            for ancestor in self.ancestors_by_key(element_key, selector) {
                                span_key = Some(ancestor.element.key().expect("unwrapping ancestor key"));
                                break;
                            }
                        }
                        if let Some(span_key) = span_key {
                            if let Some(target_element) = self.get_mut_elementdata_by_id(&idref) {
                                target_element.data.push(DataType::SpanReference(span_key));
                            } else {
                                return Err(FoliaError::ParseError("Wref span parent not found! (element gone missing)".to_string()));
                            }
                        } else {
                            return Err(FoliaError::ParseError(format!("Wref span parent not found! (idref={})",idref)));
                        }
                    } else {
                        return Err(FoliaError::ParseError("Wref element does not reference anything!".to_string()));
                    }
                },
                elementtype => {
                    if ElementGroup::Layer.contains(elementtype) {
                        //Do we have a set already?
                        if !element.has_attrib(AttribType::SET) {
                            //Find out the set from the child elements
                            let query = Query::select().elementgroup(Cmp::Is(ElementGroup::Span)).set(Cmp::Some);
                            let selector = Selector::from_query(self, &query)?;
                            for spanelement in self.get_element(element_key).expect("getting element").select(selector, Recursion::Always) {
                                if spanelement.elementtype().annotationtype() == elementtype.annotationtype() {
                                    if let Some(dec_key) = spanelement.declaration_key() {
                                        add_attributes = Some(vec!(Attribute::DeclarationRef(dec_key)));
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            return Err(FoliaError::KeyError("Element not found".to_string()));
        }
        if let Some(add_attributes) = add_attributes {
            if let Some(element) = self.get_mut_elementdata(element_key) {
                element.set_attribs(add_attributes);
            }
        }
        Ok(())
    }

    ///Removes the child from the parent, orphaning it, does NOT remove the element entirely
    pub fn detach_element(&mut self, child_key: ElementKey) -> Result<(),FoliaError> {
        let oldparent_key = if let Some(child) = self.get_mut(child_key) {
            //add the new parent and return the old parent
            let tmp = child.parent_key();
            child.set_parent_key(None);
            tmp
        } else {
            //child does not exist
            return Err(FoliaError::InternalError(format!("Child does not exist: {}", child_key)));
        };

        if let Some(oldparent_key) = oldparent_key {
            //detach child from the old parent
            if let Some(oldparent) = self.get_mut(oldparent_key) {
                if let Some(index) = oldparent.index(&DataType::Element(child_key)) {
                    oldparent.remove(index);
                }
            }
        }
        Ok(())
    }

    ///Add an element to the provenance chain
    ///Returns the key
    pub fn add_processor(&mut self, processor: Processor) -> Result<ProcKey, FoliaError> {
        let child_key = self.add(processor, None);
        if let Ok(child_key) = child_key {
            self.provenancestore.chain.push(child_key);
        }
        child_key
    }

    ///Add a processor as a subprocessor
    ///Returns the key
    pub fn add_subprocessor(&mut self, parent_key: ProcKey, processor: Processor) -> Result<ProcKey, FoliaError> {
        let child_key = self.add(processor, None);
        if let Ok(child_key) = child_key {
            self.attach_processor(parent_key, child_key)?;
        }
        child_key
    }

    ///Adds the processor element to the parent element, automatically takes care
    ///of removing the old parent (if any).
    pub fn attach_processor(&mut self, parent_key: ProcKey, child_key: ProcKey) -> Result<(),FoliaError> {
        //ensure the parent exists
        if !self.get_processor(parent_key).is_some() {
            return Err(FoliaError::InternalError(format!("Parent does not exist: {}", parent_key)));
        };

        if let Some(child) = self.get_mut_processor(child_key) {
            //add the new parent and return the old parent
            child.parent = Some(parent_key);
        } else {
            //child does not exist
            return Err(FoliaError::InternalError(format!("Child does not exist: {}", child_key)));
        };

        if let Some(parent) = self.get_mut_processor(parent_key) {
            parent.processors.push(child_key);
        }

        Ok(())
    }


    ///Add a declaration. Returns the key. If the declaration already exists it simply returns the
    ///key of the existing one.
    pub fn declare(&mut self, annotationtype: AnnotationType, set: &Option<String>, alias: &Option<String>, format: &Option<String>) -> Result<DecKey,FoliaError> {
        //first we simply check the index
        if let Some(found_key) = <Self as Store<Declaration,DecKey>>::id_to_key(self,Declaration::index_id(annotationtype, &set.as_ref().map(String::as_str)  ).as_str()) {
            return Ok(found_key);
        }

        //If not found, we search for a default
        if let Some(default_key) = self.declarationstore.get_default_key(annotationtype) {
            if let Some(declaration) = self.get_declaration(default_key) {
                if set.is_some() {
                    //there is an explicit set defined, only return the default if the sets are not
                    //in conflict
                    if let Some(declared_set) = &declaration.set {
                        if *declared_set == *set.as_ref().unwrap() {
                            return Ok(default_key);
                        }
                    }
                } else {
                    //no set defined, that means we inherit the default set
                    return Ok(default_key);
                }
            }
        }

        //if we reach this point we have no defaults and add a new declaration
        let added_key = self.add_declaration(Declaration::new(annotationtype, set.clone(), alias.clone(), format.clone()))?;
        Ok(added_key)
    }

    //************** High-level method for adding annotations ********************

    ///This is a high-level function that adds an annotation to an element, and does all necessary validation. It will simply call `add_element_to` for token annotation elements that fit within the scope and validate. For span annotation, it will create and find or create the proper annotation layer and insert the element there.
    pub fn annotate(&mut self, parent_key: ElementKey, element: ElementData) -> Result<ElementKey, FoliaError> {
        let parent = self.get_element(parent_key).ok_or(
            FoliaError::InternalError(format!("Specified element key not found: {:?}", parent_key))
        )?;
        if ElementGroup::Span.contains(element.elementtype) {
            let mut addspanfromspanned = false;
            let mut addspanfromstructure = false;
            let layertype = element.elementtype.annotationtype().expect("annotation type").layertype().ok_or(
                FoliaError::InternalError(format!("No layer type found for specified span type {:?}",element.elementtype))
            )?;
            let props = self.props(parent.elementtype());
            if props.wrefable {
                addspanfromspanned = true
            } else if ElementGroup::Structure.contains(parent.elementtype()) {
                addspanfromstructure = true
            }

            let mut set: Option<String> = None;
            if addspanfromspanned || addspanfromstructure {
                //get the set
                let set_ref = &element.set()?;
                if let Some(s) = set_ref {
                    set = Some(s.to_string());
                } else {
                    if let Some(s) = self.get_default_set(element.elementtype.annotationtype().expect("annotation type")) {
                        set = Some(s.to_string());
                    }
                    if set.is_none() {
                        return Err(FoliaError::IncompleteError(format!("No set defined when adding span annotation and none could be inferred")));
                    }
                }
            }

            if addspanfromspanned {
                //invoked from the spanned element (the parent_key/parent plays no role anymore)
                let span_keys = self.get_span_keys(&element);
                if span_keys.is_empty() {
                    Err(FoliaError::IncompleteError(format!("Span is empty, can not be added from a wrefable parent")))
                } else {
                    let query = Query::select().elementgroup(Cmp::Is(ElementGroup::Structure));
                    let common_ancestors = self.common_ancestors(Selector::from_query(&self,&query).expect("selector"), &span_keys);
                    for ancestor_key in common_ancestors.iter() {
                        let mut suitable = false; //is the ancestor suitable to hold a layer according to the specification?
                        if let Some(ancestor) = self.get_element(*ancestor_key) {
                            let props = self.props(ancestor.elementtype());
                            suitable = props.accepted_data.contains(&AcceptedData::AcceptElementGroup(ElementGroup::Layer)) || props.accepted_data.contains(&AcceptedData::AcceptElementType(layertype));
                        };
                        if suitable {
                            let mut layer_key: Option<ElementKey> = self.get_layer_key(parent_key, element.elementtype.annotationtype().expect("annotation type"), set.as_ref().map(|s| s.as_str()) )?;
                            if layer_key.is_none() {
                                //no layer found yet, add a new one
                                let layerdata = match set {
                                    Some(set) => ElementData::new(layertype).with_attrib(Attribute::Set(set.clone())),
                                    None => ElementData::new(layertype)
                                };
                                self.check_element_addable(*ancestor_key, &layerdata)?;
                                match self.add_element_to(*ancestor_key, layerdata) {
                                    Ok(key) => layer_key = Some(key),
                                    Err(e) => return Err(e)
                                }
                            };
                            self.check_element_addable(layer_key.unwrap() , &element)?;
                            //we only did one iteration, taking the closest common ancestor
                            return self.add_element_to(layer_key.unwrap(), element);
                        }
                    }
                    Err(FoliaError::IncompleteError(format!("Unable to find suitable common ancestor to create annotation layer")))
                }
            } else if addspanfromstructure {
                //invoked from the parent structure element that holds the layer (usually a sentence)
                let mut layer_key: Option<ElementKey> = self.get_layer_key(parent_key, element.elementtype.annotationtype().expect("annotation type"), set.as_ref().map(|s| s.as_str()) )?;
                if layer_key.is_none() {
                    //no layer found yet, add a new one
                    let layerdata = match set {
                        Some(set) => ElementData::new(layertype).with_attrib(Attribute::Set(set.clone())),
                        None => ElementData::new(layertype)
                    };
                    self.check_element_addable(parent_key, &layerdata)?;
                    match self.add_element_to(parent_key, layerdata) {
                        Ok(key) => layer_key = Some(key),
                        Err(e) => return Err(e)
                    }
                };
                self.check_element_addable(layer_key.unwrap(), &element)?;
                self.add_element_to(layer_key.unwrap(), element)
            } else {
                //normal behaviour
                self.check_element_addable(parent_key, &element)?;
                self.add_element_to(parent_key, element)
            }
        } else {
            //normal behaviour
            self.check_element_addable(parent_key, &element)?;
            self.add_element_to(parent_key, element)
        }
    }

    pub fn annotate_span(&mut self, element: ElementData) -> Result<ElementKey, FoliaError> {
        if !ElementGroup::Span.contains(element.elementtype) {
            return Err(FoliaError::TypeError(format!("Element passed to annotate_span is not a span element")));
        }
        let span_keys = self.get_span_keys(&element);
        if span_keys.is_empty() {
            return Err(FoliaError::IncompleteError(format!("Span is empty, can not be added from a wrefable parent")));
        }
        self.annotate(span_keys[0], element) //use the root parent key because it doesn't matter, will be extracted from the element itself
    }

    //************** Methods providing easy access to Store ****************

    pub(crate) fn get_elementdata(&self, key: ElementKey) -> Option<&ElementData> {
        <Self as Store<ElementData,ElementKey>>::get(self, key)
    }
    pub(crate) fn get_elementdata_by_id(&self, id: &str) -> Option<&ElementData> {
        <Self as Store<ElementData,ElementKey>>::get_by_id(self, id)
    }
    pub(crate) fn get_mut_elementdata(&mut self, key: ElementKey) -> Option<&mut ElementData> {
        <Self as Store<ElementData,ElementKey>>::get_mut(self, key)
    }
    pub(crate) fn get_mut_elementdata_by_id(&mut self, id: &str) -> Option<&mut ElementData> {
        <Self as Store<ElementData,ElementKey>>::get_mut_by_id(self, id)
    }

    pub fn get_element_key_by_id(&self, id: &str) -> Option<ElementKey> {
        <Self as Store<ElementData,ElementKey>>::id_to_key(self, id)
    }
    pub fn get_declaration(&self, key: DecKey) -> Option<&Declaration> {
        <Self as Store<Declaration,DecKey>>::get(self, key)
    }
    pub fn get_declaration_by_id(&self, id: &str) -> Option<&Declaration> {
        <Self as Store<Declaration,DecKey>>::get_by_id(self, id)
    }
    pub fn get_declaration_key_by_id(&self, id: &str) -> Option<DecKey> {
        <Self as Store<Declaration,DecKey>>::id_to_key(self, id)
    }
    pub fn get_mut_declaration(&mut self, key: DecKey) -> Option<&mut Declaration> {
        <Self as Store<Declaration,DecKey>>::get_mut(self, key)
    }
    pub fn get_mut_declaration_by_id(&mut self, id: &str) -> Option<&mut Declaration> {
        <Self as Store<Declaration,DecKey>>::get_mut_by_id(self, id)
    }
    pub fn declarations(&self) -> std::slice::Iter<Option<Box<Declaration>>>  { //TODO: simplify output type
        <Self as Store<Declaration,DecKey>>::iter(self)
    }
    pub fn get_processor(&self, key: ProcKey) -> Option<&Processor> {
        <Self as Store<Processor,ProcKey>>::get(self, key)
    }
    pub fn get_processor_by_id(&self, id: &str) -> Option<&Processor> {
        <Self as Store<Processor,ProcKey>>::get_by_id(self, id)
    }
    pub fn get_processor_key_by_id(&self, id: &str) -> Option<ProcKey> {
        <Self as Store<Processor,ProcKey>>::id_to_key(self, id)
    }
    pub fn get_mut_processor(&mut self, key: ProcKey) -> Option<&mut Processor> {
        <Self as Store<Processor,ProcKey>>::get_mut(self, key)
    }
    pub fn get_mut_processor_by_id(&mut self, id: &str) -> Option<&mut Processor> {
        <Self as Store<Processor,ProcKey>>::get_mut_by_id(self, id)
    }

    //************** Higher level element retrieval methods ****************
    //
    pub fn get_element(&self, key: ElementKey) -> Option<Element> {
        if let Some(elementdata) = self.get_elementdata(key) {
            Some(Element { document: Some(self), data: elementdata })
        } else {
            None
        }
    }

    pub fn get_element_by_id(&self, id: &str) -> Option<Element> {
        if let Some(elementdata) = self.get_elementdata_by_id(id) {
            Some(Element { document: Some(self), data: elementdata })
        } else {
            None
        }
    }

    /*
    pub fn get_mut_element(&mut self, key: ElementKey) -> Option<MutElement> {
        if let Some(elementdata) = self.get_mut_elementdata(key) {
            Some(MutElement { document: Some(self), data: elementdata })
        } else {
            None
        }
    }
    */

    ///Get the layer under the specified element, for the given annotation type and set.
    pub fn get_layer_key(&self, key: ElementKey, annotationtype: AnnotationType, set: Option<&str>) -> Result<Option<ElementKey>,FoliaError> {
        let layertype = annotationtype.layertype().ok_or(
            FoliaError::InternalError(format!("No layer type found for specified span type {:?}",annotationtype))
        )?;
        let query = Query::select().element(Cmp::Is(layertype)).set(match set {
            Some(set) => Cmp::Is(set.to_string()),
            None => Cmp::None,
        });
        let selector = Selector::from_query(self, &query)?;
        if let Some(element) = self.get_element(key) {
            for layer in element.select(selector, Recursion::No) {
                return Ok(layer.key());
            }
        }
        Ok(None)
    }

    pub fn get_span_keys(&self, elementdata: &ElementData) -> Vec<ElementKey> {
        let mut span_keys: Vec<ElementKey> = Vec::new();
        for child in elementdata.data.iter() {
            if let DataType::Element(k) = child {
                span_keys.push(*k);
            } else if let DataType::AddElement(ed) = child {
                if ed.elementtype == ElementType::WordReference {
                    if let Some(a) = ed.attrib(AttribType::IDREF) {
                        if let Some(k) = self.get_element_key_by_id(a.as_str().expect("str")) {
                            span_keys.push(k);
                        }
                    }
                }
            }
        }
        span_keys
    }

    //************** Other high-level retrieval methods ****************
    //

    pub fn get_default_set(&self, annotationtype: AnnotationType) -> Option<&str> {
        if let Some(default_key) = self.declarationstore.get_default_key(annotationtype) {
            if let Some(declaration) = self.get_declaration(default_key) {
                return declaration.set.as_ref().map(|s| s.as_str());
            }
        }
        None
    }


    //************** Validation methods ****************
    //
    pub fn check_element_addable(&self, parent_key: ElementKey, element: &ElementData) -> Result<(), FoliaError> {
        let parent = self.get_element(parent_key).ok_or(
            FoliaError::InternalError(format!("Specified parent element key not found"))
        )?;
        let props = self.props(parent.elementtype());
        for accepted_data in props.accepted_data.iter() {
            match accepted_data {
                AcceptedData::AcceptElementType(et) => if *et == element.elementtype {
                    return Ok(())
                },
                AcceptedData::AcceptElementGroup(g) => if g.contains(element.elementtype) {
                    return Ok(())
                }
            }
        }
        Err(FoliaError::ValidationError(format!("Can't add element type {:?} to {:?}", element.elementtype, parent.elementtype())))
    }

}


impl Store<ElementData,ElementKey> for Document {

    fn items_mut(&mut self) -> &mut Vec<Option<Box<ElementData>>> {
        &mut self.elementstore.items
    }
    fn index_mut(&mut self) -> &mut HashMap<String,ElementKey> {
        &mut self.elementstore.index
    }

    fn items(&self) -> &Vec<Option<Box<ElementData>>> {
        &self.elementstore.items
    }
    fn index(&self) -> &HashMap<String,ElementKey> {
        &self.elementstore.index
    }

    fn iter(&self) -> std::slice::Iter<Option<Box<ElementData>>> {
        self.elementstore.items.iter()
    }

    ///Actively encode element for storage, this encodes attributes that need to be encoded (such as set,class,processor), and adds them to their respective stores.
    ///It does not handle relations between elements (data/children and parent)
    ///nor does it add the element itself to the store
    ///to the store).
    fn encode(&mut self, mut element: ElementData, context: Option<ElementKey>) -> Result<ElementData, FoliaError> {
        if !element.encodable() {
            //already encoded, nothing to do
            return Ok(element);
        }

        let mut declaration_key: Option<DecKey> = None;
        let mut class_key: Option<ClassKey> = None;
        let mut processor_key: Option<ProcKey> = None;
        let mut subset_key: Option<SubsetKey> = None;

        //encode the element for storage
        if let Some(annotationtype) = element.elementtype.annotationtype() {
            //Declare the element (either declares anew or just resolves the to the right
            //declaration.
            let deckey = self.declare(annotationtype, &element.set().unwrap().map(|s| s.to_string()),  &None,&None)?;
            declaration_key  = Some(deckey);

            if let Ok(Some(class)) = element.class() {
                if let Some(declaration) = self.get_mut_declaration(deckey) {
                    if let Ok(clskey) = declaration.add_class(Cow::Borrowed(class)) {
                        class_key = Some(clskey);
                    }
                }
            }

            if let Some(declaration) = self.get_declaration(deckey) {
                processor_key = declaration.default_processor() //returns an Option, may be overriden later if a specific processor is et
            }
        } else {
            match element.elementtype {
                ElementType::Feature => {
                    if let Some(parent_key) = context {
                        //get the declaration key from the parent context:
                        let parent = self.get_elementdata(parent_key).ok_or( FoliaError::InternalError("Context for feature does not exist!".to_string()))?;

                        let annotationtype = parent.elementtype.annotationtype().expect(format!("Unwrapping annotation type of parent {}", element.elementtype).as_str() );
                        let deckey = self.declare(annotationtype, &element.set().unwrap().map(|s| s.to_string()),  &None, &None)?;
                        declaration_key  = Some(deckey);;

                        if let Some(declaration) = self.get_mut_declaration(deckey) {
                            if let Ok(Some(class)) = element.class() {
                                if let Ok(clskey) = declaration.add_subclass(Cow::Borrowed(class)) {
                                    class_key = Some(clskey);
                                }
                            }
                            if let Ok(Some(subset)) = element.subset() {
                                if let Ok(subsetkey) = declaration.add_subset(Cow::Borrowed(subset)) {
                                    subset_key = Some(subsetkey);
                                }
                            }
                        }


                    } else {
                        return Err(FoliaError::InternalError("No context provided for feature".to_string()));
                    }
                }
                _ => {
                }
            }
        }

        if let Ok(Some(processor_id)) = element.processor() {
            if let Some(prockey) = <Self as Store<Processor,ProcKey>>::id_to_key(self, processor_id) {
                processor_key = Some(prockey); //overrides the earlier-set default (if any)
            }
        }

        //remove encodable attributes
        element.attribs.retain(|attrib| !attrib.encodable());
        //add encoded attributes
        if let Some(declaration_key) = declaration_key {
            element.attribs.push(Attribute::DeclarationRef(declaration_key));
        }
        if let Some(class_key) = class_key {
            element.attribs.push(Attribute::ClassRef(class_key));
        }
        if let Some(processor_key) = processor_key {
            element.attribs.push(Attribute::ProcessorRef(processor_key));
        }
        if let Some(subset_key) = subset_key {
            element.attribs.push(Attribute::SubsetRef(subset_key));
        }

        Ok(element)
    }
}

impl Store<Declaration,DecKey> for Document {

    fn items_mut(&mut self) -> &mut Vec<Option<Box<Declaration>>> {
        &mut self.declarationstore.items
    }
    fn index_mut(&mut self) -> &mut HashMap<String,DecKey> {
        &mut self.declarationstore.index
    }

    fn items(&self) -> &Vec<Option<Box<Declaration>>> {
        &self.declarationstore.items
    }
    fn index(&self) -> &HashMap<String,DecKey> {
        &self.declarationstore.index
    }

    fn iter(&self) -> std::slice::Iter<Option<Box<Declaration>>> {
        self.declarationstore.items.iter()
    }
}

impl Store<Processor,ProcKey> for Document {
    fn items_mut(&mut self) -> &mut Vec<Option<Box<Processor>>> {
        &mut self.provenancestore.items
    }
    fn index_mut(&mut self) -> &mut HashMap<String,ProcKey> {
        &mut self.provenancestore.index
    }

    fn items(&self) -> &Vec<Option<Box<Processor>>> {
        &self.provenancestore.items
    }
    fn index(&self) -> &HashMap<String,ProcKey> {
        &self.provenancestore.index
    }

    fn iter(&self) -> std::slice::Iter<Option<Box<Processor>>> {
        self.provenancestore.items.iter()
    }
}
