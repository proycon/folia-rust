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
use crate::serialiser::*;
use crate::parser::*;
use crate::specification::*;

pub struct Document {
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
}



impl Document {
    ///Create a new FoLiA document from scratch
    pub fn new(id: &str, bodytype: BodyType) -> Result<Self, FoliaError> {
        let mut document = Self {
            id: id.to_string(),
            filename: None,
            version: FOLIAVERSION.to_string(),
            elementstore: ElementStore::default(),
            provenancestore:  ProvenanceStore::default(),
            declarationstore: DeclarationStore::default(),
            metadata: Metadata::default(),
        };
        let mut body = match bodytype {
            BodyType::Text => ElementData::new(ElementType::Text),
            BodyType::Speech => ElementData::new(ElementType::Speech),
        };
        body = document.encode(body)?;
        assert!(body.is_encoded());
        document.add(body)?;
        Ok(document)
    }

    ///Load a FoliA document from file. Invokes the XML parser and loads it all into memory.
    pub fn from_file(filename: &str) -> Result<Self, FoliaError> {
        let mut reader = Reader::from_file(Path::new(filename))?;
        reader.trim_text(true);
        let mut doc = Self::parse(&mut reader)?;
        //associate the filename with the document
        doc.filename = Some(filename.to_string());
        Ok(doc)
    }

    ///Load a FoliA document from XML string representation, loading it all into memory.
    pub fn from_str(data: &str) -> Result<Self, FoliaError> {
        let mut reader = Reader::from_str(data);
        reader.trim_text(true);
        Self::parse(&mut reader)
    }





    ///Returns the ID of the document
    pub fn id(&self) -> &str { &self.id }

    ///Returns the filename associated with this document (i.e. the file from which it was loaded)
    pub fn filename(&self) -> Option<&str> { self.filename.as_ref().map(String::as_str) } //String::as_str equals  |x| &**x


    pub fn textelement_encode(&self, element_key: ElementKey, set: Option<&str>, textclass: Option<&str>) -> Option<&ElementData> {
        let set: &str = if let Some(set) = set {
            set
        } else {
            DEFAULT_TEXT_SET
        };
        let textclass: &str = if let Some(textclass) = textclass {
            textclass
        } else {
            "current"
        };
        for element in self.select_elements(element_key, Selector::new_encode(&self, ElementType::TextContent, SelectorValue::Some(set), SelectorValue::Some(textclass)), false)  {
            return Some(element.element);
        }
        None
    }




    ///Get properties from the specification (a shortcut)
    pub fn props(&self, elementtype: ElementType) -> &Properties {
        self.elementstore.specification.get(elementtype)
    }

    //************** Methods providing easy write adccess into the underlying Stores ********************

    ///Add an element to the document (but the element will be an orphan unless it is the very
    ///first one, you may want to use ``add_element_to`` instead)
    pub fn add_element(&mut self, element: ElementData) -> Result<ElementKey, FoliaError> {
        <Self as Store<ElementData,ElementKey>>::add(self, element)
    }

    ///Add a declaration. It is strongly recommended to use ``declare()`` instead
    ///because this one adds a declaration without any checks.
    ///Returns the key.
    pub fn add_declaration(&mut self, declaration: Declaration) -> Result<DecKey, FoliaError> {
        <Self as Store<Declaration,DecKey>>::add(self, declaration)
    }

    ///Add an processor the document (but the processor will be an orphan and not in the processor
    ///chain!). You may want to use ``add_processor()`` instead to add to the provenance chain or
    ///``add_subprocessor()`` to add a processor as a subprocessor.
    pub fn add_provenance(&mut self, processor: Processor) -> Result<ProcKey, FoliaError> {
        <Self as Store<Processor,ProcKey>>::add(self, processor)
    }

    //************** Higher-order methods for adding things ********************

    ///Adds a new element as a child of another, this is a higher-level function that/
    ///takes care of adding and attaching for you.
    pub fn add_element_to(&mut self, parent_key: ElementKey, mut element: ElementData) -> Result<ElementKey, FoliaError> {
        match self.add_element(element) {
            Ok(child_key) => {
                self.attach_element(parent_key, child_key)?;
                Ok(child_key)
            },
            Err(err) => {
                Err(FoliaError::InternalError(format!("Unable to add element to parent: {}", err)))
            }
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
        let child_key = self.add(processor);
        if let Ok(child_key) = child_key {
            self.provenancestore.chain.push(child_key);
        }
        child_key
    }

    ///Add a processor as a subprocessor
    ///Returns the key
    pub fn add_subprocessor(&mut self, parent_key: ProcKey, processor: Processor) -> Result<ProcKey, FoliaError> {
        let child_key = self.add(processor);
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
    pub fn declare(&mut self, annotationtype: AnnotationType, set: &Option<String>, alias: &Option<String>) -> Result<DecKey,FoliaError> {
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
        let added_key = self.add_declaration(Declaration::new(annotationtype, set.clone(), alias.clone()))?;
        Ok(added_key)
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

    pub(crate) fn get_element_by_id(&self, id: &str) -> Option<Element> {
        if let Some(elementdata) = self.get_elementdata_by_id(id) {
            Some(Element { document: Some(self), data: elementdata })
        } else {
            None
        }
    }
    pub fn get_element(&self, key: ElementKey) -> Option<mut MutElement> {
        if let Some(elementdata) = self.get_mut_elementdata(key) {
            Some(MutElement { document: Some(self), data: elementdata })
        } else {
            None
        }
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
    fn encode(&mut self, mut element: ElementData) -> Result<ElementData, FoliaError> {
        if element.is_encoded() {
            //already encoded, nothing to do
            return Ok(element);
        }

        let mut enc_attribs: EncodedAttributes = EncodedAttributes::default();

        //encode the element for storage
        let set = element.attrib(AttribType::SET);

        if let Some(annotationtype) = element.elementtype.annotationtype() {
            //Declare the element (either declares anew or just resolves the to the right
            //declaration.
            let deckey = self.declare(annotationtype, &set.map(|x| x.value().into_owned() ), &None)?;
            enc_attribs.declaration = Some(deckey);

            if let Some(class) = element.attrib(AttribType::CLASS) {
                if let Attribute::Class(class) = class {
                    if let Ok(class_key) = self.add_class(deckey, class) {
                        enc_attribs.class = Some(class_key);
                    }
                }
            }

            if let Some(declaration) = self.get_declaration(deckey) {
                enc_attribs.processor = declaration.default_processor() //returns an Option, may be overriden later if a specific processor is et
            }
        }

        if let Some(processor) = element.attrib(AttribType::PROCESSOR) {
            let processor_id: &str  = &processor.value();

            if let Some(processor_key) = <Self as Store<Processor,ProcKey>>::id_to_key(self, processor_id) {
                enc_attribs.processor = Some(processor_key); //overrides the earlier-set default (if any)
            }
        }

        //remove encoded attributes
        element.attribs.retain(|a| match a {
            Attribute::Set(_) | Attribute::Class(_) | Attribute::Processor(_) => false,
            _ => true
        });

        element.set_enc_attribs(Some(enc_attribs));

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
