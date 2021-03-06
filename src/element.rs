use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::borrow::Cow;
use std::str::{FromStr,from_utf8};
use std::string::ToString;
use std::fmt;
use std::iter::ExactSizeIterator;
use std::convert::Into;
use std::clone::Clone;

use std::io::Write;
use std::io::BufWriter;
use std::io::Cursor;
use quick_xml::{Reader,Writer};
use quick_xml::events::{Event,BytesStart,BytesEnd,BytesText};

use crate::common::*;
use crate::types::*;
use crate::error::*;
use crate::attrib::*;
use crate::elementstore::*;
use crate::store::*;
use crate::metadata::*;
use crate::query::*;
use crate::select::*;
use crate::text::*;
use crate::document::{Document};




pub enum ValidationStrategy {
    NoValidation,
    ShallowValidation,
    DeepValidation
}


#[derive(Clone,PartialEq,Debug)]
///This is the structure that represents any instance of a FoLiA element. The type of the structure
///is represented by ``elementtype``. An elements holds and owns attributes, encoded attributes (if
///it is encoded already),  data items (which may be text, comments or child elements (by key)), and a link
///to its parent (by key).
pub struct ElementData {
    pub elementtype: ElementType,
    pub attribs: Vec<Attribute>,

    //encoded (inter-element)
    pub(crate) data: Vec<DataType>,
    pub(crate) key: Option<ElementKey>,
    pub(crate) parent: Option<ElementKey>,
}

#[derive(Clone,Copy)]
///Interface to a FoLiA element
pub struct Element<'a> {
    pub(crate) document: Option<&'a Document>,
    pub(crate) data: &'a ElementData
}

///Mutable interface to a FoLiA element
pub struct MutElement<'a> {
    pub(crate) document: Option<&'a Document>,
    pub(crate) data: &'a mut ElementData
}

impl<'a> Element<'a> {

    pub(crate) fn elementdata(&self) -> &'a ElementData {
        self.data
    }


}


impl Storable<ElementKey> for ElementData {
    fn maybe_id(&self) -> Option<Cow<str>> {
        if let Some(attrib) = self.attrib(AttribType::ID) {
            Some(Cow::Borrowed(attrib.as_str().expect("unwrapping ID result for element")))
        } else {
            None
        }
    }

    ///Returns ``true`` if this element is encodable, i.e. if it needs to be encoded
    ///prior to being stored. An element is encodable if it has one or more encodable elements.
    fn encodable(&self) -> bool {
        for attrib in self.attribs.iter() {
            if attrib.encodable() {
                    return true
            }
        }
        false
    }

    ///Returns the key of the current element
    fn key(&self) -> Option<ElementKey> {
        self.key
    }

    ///Sets the key of the current element
    fn assign_key(&mut self, key: ElementKey) {
        self.key = Some(key);
    }

}

impl ElementData {
    ///Returns the ID of the element (if any)
    pub fn id(&self) -> Option<&str> {
        for attrib in self.attribs.iter() {
            if let Attribute::Id(id) = attrib {
                return Some(id.as_str());
            }
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }


    pub fn attribs(&self) -> &Vec<Attribute> {
        &self.attribs
    }

    ///Get the FoLiA class if the element is not encoded yet, returns an error otherwise
    pub fn class(&self) -> Result<Option<&str>,FoliaError> {
        for attrib in self.attribs().iter() {
            if let Attribute::Class(s) = attrib {
                return Ok(Some(s));
            } else if attrib.decodable() {
                return Err(FoliaError::EncodeError("Querying for a decoded attribute on attributes that are not decoded yet".to_string()));
            }
        }
        Ok(None)
    }

    ///Get the FoLiA set if the element is not encoded yet, returns an error otherwise
    pub fn set(&self) -> Result<Option<&str>,FoliaError> {
        for attrib in self.attribs().iter() {
            if let Attribute::Set(s) = attrib {
                return Ok(Some(s));
            } else if attrib.decodable() {
                return Err(FoliaError::EncodeError("Querying for a decoded attribute on attributes that are not decoded yet".to_string()));
            }
        }
        Ok(None)
    }

    ///Get the FoLiA subset if the element is not encoded yet, returns an error otherwise (only
    ///applies to elements with elementtype ``ElementType::Feature``
    pub fn subset(&self) -> Result<Option<&str>,FoliaError> {
        for attrib in self.attribs().iter() {
            if let Attribute::Subset(s) = attrib {
                return Ok(Some(s));
            } else if attrib.decodable() {
                return Err(FoliaError::EncodeError("Querying for a decoded attribute on attributes that are not decoded yet".to_string()));
            }
        }
        Ok(None)
    }

    ///Get the Processor ID if the element is not encoded yet, returns an error otherwise
    pub fn processor(&self) -> Result<Option<&str>,FoliaError> {
        for attrib in self.attribs().iter() {
            if let Attribute::Processor(s) = attrib {
                return Ok(Some(s));
            } else if attrib.decodable() {
                return Err(FoliaError::EncodeError("Querying for a decoded attribute on attributes that are not decoded yet".to_string()));
            }
        }
        Ok(None)
    }

    pub fn confidence(&self) -> Option<f64> {
        for attrib in self.attribs().iter() {
            if let Attribute::Confidence(f) = attrib {
                return Some(*f);
            }
        }
        None
    }


    ///Get the FoLiA class key if the element is encoded, returns an error otherwise
    pub fn class_key(&self) -> Result<Option<ClassKey>,FoliaError> {
        for attrib in self.attribs().iter() {
            if let Attribute::ClassRef(k) = attrib {
                return Ok(Some(*k));
            } else if attrib.encodable() {
                return Err(FoliaError::EncodeError("Querying for an encoded attribute on attributes that are not encoded yet".to_string()));
            }
        }
        Ok(None)
    }

    ///Get the FoLiA set/declaration key if the element is encoded, returns an error otherwise
    pub fn declaration_key(&self) -> Result<Option<DecKey>,FoliaError> {
        for attrib in self.attribs().iter() {
            if let Attribute::DeclarationRef(k) = attrib {
                return Ok(Some(*k));
            } else if attrib.encodable() {
                return Err(FoliaError::EncodeError("Querying for an encoded attribute on attributes that are not encoded yet".to_string()));
            }
        }
        Ok(None)
    }

    ///Get the FoLiA processor key if the element is encoded, returns an error otherwise
    pub fn processor_key(&self) -> Result<Option<ProcKey>,FoliaError> {
        for attrib in self.attribs().iter() {
            if let Attribute::ProcessorRef(k) = attrib {
                return Ok(Some(*k));
            } else if attrib.encodable() {
                return Err(FoliaError::EncodeError("Querying for an encoded attribute on attributes that are not encoded yet".to_string()));
            }
        }
        Ok(None)
    }

    ///Get the FoLiA subset key if the element is encoded, returns an error otherwise
    pub fn subset_key(&self) -> Result<Option<SubsetKey>,FoliaError> {
        for attrib in self.attribs().iter() {
            if let Attribute::SubsetRef(k) = attrib {
                return Ok(Some(*k));
            } else if attrib.encodable() {
                return Err(FoliaError::EncodeError("Querying for an encoded attribute on attributes that are not encoded yet".to_string()));
            }
        }
        Ok(None)
    }

    ///Get the ID reference if this element is a wref
    pub fn idref(&self) -> Option<&str> {
        for attrib in self.attribs().iter() {
            if let Attribute::Idref(id) = attrib {
                return Some(id);
            }
        }
        None
    }
}

pub trait ReadElement {
    //Main accessors that provide access to the underlying datastructure
    //so we can rely on a generic implementation here
    fn elementdata(&self) -> &ElementData;
    fn document(&self) -> Option<&Document>;


    ///Return the key of this element
    fn key(&self) -> Option<ElementKey> {
        self.elementdata().key()
    }

    ///Gives access to all attributes
    fn attribs(&self) -> &Vec<Attribute> {
        &self.elementdata().attribs()
    }

    ///Get the element type of this element
    fn elementtype(&self) -> ElementType {
        self.elementdata().elementtype
    }

    ///Get a specific attribute by type
    fn attrib(&self, atype: AttribType) -> Option<&Attribute> {
        self.elementdata().attrib(atype)
    }

    ///Check if a specific attribute exists (by type)
    fn has_attrib(&self, atype: AttribType) -> bool {
        self.elementdata().has_attrib(atype)
    }

    ///Get the FoliA set
    fn set(&self) -> Option<&str> {
        if let Some(declaration) = self.get_declaration() {
            if let Some(set) = &declaration.set {
                Some(set.as_str())
            } else {
                None
            }
        } else {
            None
        }
    }

    ///Get the FoliA subset (only apply to elements of type ``ElementType::Feature`` aka
    ///``<feat>``)
    fn subset(&self) -> Option<&str> {
        if let Some(declaration) = self.get_declaration() {
            if let Some(subset_key) = self.subset_key() {
                if let Some(subset) = &declaration.get_subset(subset_key) {
                    return Some(subset);
                }
            }
        }
        None
    }

    ///Get the FoLiA class
    fn class(&self) -> Option<&str> {
        if let Some(class_key) =  self.class_key() {
            if let Some(declaration) = self.get_declaration() {
                if self.subset_key().is_some() { //we have a subset (this happens on ElementType::Feature only)
                    //class is a subclass
                    return declaration.get_subclass(class_key);
                } else {
                    //class is a normal class
                    return declaration.get_class(class_key);
                }
            }
        }
        None
    }

    ///Get the processor ID
    fn processor(&self) -> Option<&str> {
        if let Some(processor) = self.get_processor() {
            Some(processor.id.as_str())
        } else {
            None
        }
    }

    ///Get the annotator (i.e. the processor name) associated with this element
    fn annotator(&self) -> Option<&str> {
        if let Some(processor) = self.get_processor() {
            Some(processor.name.as_str())
        } else {
            //fall back to old annotator
            for attrib in self.attribs() {
                if let Attribute::Annotator(annotator) = attrib {
                    return Some(annotator);
                }
            }
            None
        }
    }

    ///Get the annotator (i.e. the processor name) associated with this element
    fn annotatortype(&self) -> Option<ProcessorType> {
        if let Some(processor) = self.get_processor() {
            Some(processor.processortype)
        } else {
            //fall back to old annotator
            for attrib in self.attribs() {
                if let Attribute::AnnotatorType(annotatortype) = attrib {
                    return Some(*annotatortype);
                }
            }
            None
        }
    }

    ///Get the ID of the element
    fn id(&self) -> Option<&str> {
        self.elementdata().id()
    }

    ///Get the class key, i.e. the encoded (numeric) form of the class
    fn class_key(&self) -> Option<ClassKey> {
        self.elementdata().class_key().expect("Unwrapping class key result")
    }
    ///Get the subset key, i.e. the encoded (numeric) form of the subset
    fn subset_key(&self) -> Option<SubsetKey> {
        self.elementdata().subset_key().expect("Unwrapping subset key result")
    }
    ///Get the declaration key, i.e. the encoded (numeric) form of the set (and annotationtype)
    fn declaration_key(&self) -> Option<DecKey> {
        self.elementdata().declaration_key().expect("Unwrapping declaration key result")
    }
    ///Get the processor key, i.e. the encoded (numeric) form of the processor
    fn processor_key(&self) -> Option<ProcKey> {
        self.elementdata().processor_key().expect("Unwrapping Processor key result")
    }
    ///Get the parent's element key, i.e. the encoded (numeric) form of the parent element
    fn parent_key(&self) -> Option<ElementKey> {
        self.elementdata().parent_key()
    }

    ///Get the declaration instance
    fn get_declaration(&self) -> Option<&Declaration> {
        if self.document().is_none() {
            None
        } else {
            if let Some(declaration_key) = self.declaration_key() {
               self.document().unwrap().get_declaration(declaration_key)
            } else {
               None
            }
        }
    }

    ///Get the processor instance
    fn get_processor(&self) -> Option<&Processor> {
        if self.document().is_none() {
            None
        } else {
            if let Some(processor_key) = self.processor_key() {
                self.document().unwrap().get_processor(processor_key)
            } else {
                None
            }
        }
    }

    ///Get the parent element
    fn get_parent(&self) -> Option<Element> {
        if self.document().is_none() {
            None
        } else {
            if let Some(parent_key) =  self.parent_key() {
                self.document().unwrap().get_element(parent_key)
            } else {
                None
            }
        }
    }

    ///Get the index of this element relative to the parent
    fn get_index(&self) -> Option<usize> {
        if let Some(parent) = self.get_parent() {
            for (i, data) in parent.elementdata().data.iter().enumerate() {
                if let DataType::Element(refkey) = data {
                    if *refkey == self.key().expect("unwrapping key") {
                        return Some(i);
                    }
                }
            }
        }
        None
    }

    ///Serialise this element (and everything under it) to XML
    fn xml(&self, indent: usize) -> Result<String, FoliaError> {
        if let Some(doc) = self.document() {
            let mut writer = match indent {
                0 => Writer::new(Cursor::new(Vec::new())),
                indent =>  Writer::new_with_indent(Cursor::new(Vec::new()), b' ', indent)
            };
            doc.xml_elements(&mut writer, self.key().unwrap())?;
            let result = writer.into_inner().into_inner();
            let result = from_utf8(&result).expect("encoding utf-8");
            Ok(result.to_string())
        } else {
            Err(FoliaError::SerialisationError("Unable to serialise orphaned elements".to_string()))
        }
    }

    ///Resolve the reference (assuming this element provides a reference)
    fn resolve(&self) -> Option<Element> {
        if let Some(doc) = self.document() {
            for attrib in self.attribs().iter() {
                if let Attribute::Idref(id) = attrib {
                    return doc.get_element_by_id(id);
                }
            }
        }
        None
    }

}


impl<'a> PartialEq for Element<'a> {
    ///Equality test for elements, two elements are considered equal if they have the same key
    fn eq(&self, other: &Self) -> bool {
        self.data.key().is_some() && self.data.key() == other.data.key()
    }
}
impl<'a> Eq for Element<'a> { }

impl<'a> fmt::Display for Element<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(text) = self.text(&TextParameters::default()) {
            write!(f, "{}", text)
        } else {
            Err(fmt::Error)
        }
    }
}


/*
impl<'a> Into<ElementData> for Element<'a> {
    ///Decodes an element and returns a **copy** ElementData, therefore it should be used sparingly.
    ///The copy is detached from any parents and does not hold child elements.
    fn into(self) -> ElementData {

    ///Decodes an element and returns a **copy**, therefore it should be used sparingly.
    ///It does not decode relations between elements (data/children and parent), only set, class
    ///and processor.
    pub fn decode(&self, document: &Document) -> Self {
        let mut decoded_attribs: Vec<Attribute> = self.attribs.iter().map(|a| a.clone()).collect();
        if let Some(set) = self.set_decode(document) {
            decoded_attribs.push(Attribute::Set(set.to_string()));
        }
        if let Some(class) = self.class_decode(document) {
            decoded_attribs.push(Attribute::Class(class.to_string()));
        }
        if let Some(processor) = self.processor_decode(document) {
            decoded_attribs.push(Attribute::Processor(processor.to_string()));
        }
        Self::new(self.elementtype).with_attribs(decoded_attribs)
    }

    }
}
*/

impl<'a> ReadElement for Element<'a> {
    fn elementdata(&self) -> &ElementData {
        self.data
    }
    fn document(&self) -> Option<&Document> {
        self.document
    }
}

impl<'a> Element<'a> {
    ///High-level function to get a particular annotation by annotation type and set. This function
    ///returns only one annotation (the first one if there are multiple) and returns None if it does not exists.
    pub fn get_annotation(&self, annotationtype: AnnotationType, set: Cmp<String>, recursion: Recursion) -> Option<Element> {
        self.get_element(annotationtype.elementtype(), set, recursion)
    }

    ///High-level function to get a particular annotation by annotation type and set, returns an
    ///iterator.
    pub fn get_annotations(&self, annotationtype: AnnotationType, set: Cmp<String>, recursion: Recursion) -> SelectElementsIterator {
        self.get_elements(annotationtype.elementtype(), set, recursion)
    }

    ///High-level function to get a particular annotation by annotation type and set, returns an
    ///iterator.
    pub fn get_elements(&self, elementtype: ElementType, set: Cmp<String>, recursion: Recursion) -> SelectElementsIterator {
        self.select(Selector::from_query(self.document().expect("Unwrapping document on element for get_annotations()"), &Query::select().element(Cmp::Is(elementtype)).set(set)).expect("Compiling query for get_annotations()"), recursion)
    }

    ///High-level function to get a particular annotation by element type and set. This function
    ///returns only one annotation (the first one if there are multiple) and returns None if it does not exists.
    pub fn get_element(&self, elementtype: ElementType, set: Cmp<String>, recursion: Recursion) -> Option<Element> {
        if self.document.is_none() { //saves us from a panic in the deeper callV
            None
        } else {
            self.get_elements(elementtype,set, recursion).next().map(|e| e.element)
        }
    }

    ///High-level function to get a particular feature by annotation type and set, returns an
    ///iterator.
    pub fn get_features(&self, subset: Cmp<String>) -> SelectElementsIterator {
        self.select(
                Selector::from_query(self.document().expect("Unwrapping document on element for get_features()"),
                    &Query::select()
                           .element(Cmp::Is(ElementType::Feature))
                           .contexttype(Cmp::Is(self.elementtype()))
                           .set(match self.set() {
                               Some(set) => Cmp::Is(set.to_string()),
                               None => Cmp::None,
                            })
                           .subset(subset)
                ).expect("Compiling query for get_features()")
        , Recursion::No)
    }


    ///High-level function to get a particular feature by annotation type and set, returns an
    ///single element (the first feature).
    pub fn get_feature(&self, subset: Cmp<String>) -> Option<Element> {
        if self.document.is_none() { //saves us from a panic in the deeper call
            None
        } else {
            self.get_features(subset).next().map(|e| e.element)
        }
    }

    ///High level-function to get a particular ancestor by annoation type and set. This function
    ///returns only one element
    pub fn get_ancestor(&self, elementtype: ElementType, set: Cmp<String>) -> Option<Element> {
        if self.document.is_none() { //saves us from a panic in the deeper call
            None
        } else {
            self.get_ancestors(elementtype,set).next().map(|e| e.element)
        }
    }

    ///High level-function to get a particular ancestor by annoation type and set. This function
    ///returns only one element
    pub fn get_ancestor_by_group(&self, elementgroup: ElementGroup, set: Cmp<String>) -> Option<Element> {
        if self.document.is_none() { //saves us from a panic in the deeper call
            None
        } else {
            self.get_ancestors_by_group(elementgroup,set).next().map(|e| e.element)
        }
    }

    ///Returns an iterator over the ancestors of this element, starting with the parent and moving
    ///up the tree.
    pub fn get_ancestors(&self, elementtype: ElementType, set: Cmp<String>) -> AncestorIterator {
        self.ancestors(Selector::from_query(self.document().expect("Unwrapping document on element for get_ancestors()"), &Query::select().element(Cmp::Is(elementtype)).set(set)).expect("Compiling query for get_ancestors()"))
    }

    ///Returns an iterator over the ancestors of this element, starting with the parent and moving
    ///up the tree.
    pub fn get_ancestors_by_group(&self, elementgroup: ElementGroup, set: Cmp<String>) -> AncestorIterator {
        self.ancestors(Selector::from_query(self.document().expect("Unwrapping document on element for get_ancestors_by_group()"), &Query::select().elementgroup(Cmp::Is(elementgroup)).set(set)).expect("Compiling query for get_ancestors_by_group()"))
    }

}


impl ElementData {

    ///Get Attribute
    pub fn attrib(&self, atype: AttribType) -> Option<&Attribute> {
        for attribute in self.attribs.iter() {
            if attribute.attribtype() == atype {
                return Some(attribute);
            }
        }
        None
    }


    ///Check if the attribute exists
    pub fn has_attrib(&self, atype: AttribType) -> bool {
        self.attribs.iter().find(|&a| a.attribtype() == atype).is_some()
    }


    ///Deletes (and returns) the specified attribute
    pub fn del_attrib(&mut self, atype: AttribType) -> Option<Attribute> {
        let position = self.attribs.iter().position(|a| a.attribtype() == atype);
        if let Some(position) = position {
            Some(self.attribs.remove(position))
        } else {
            None
        }
    }

    ///Sets/adds and attribute
    pub fn set_attrib(&mut self, attrib: Attribute) {
        //ensure we don't insert duplicates
        self.del_attrib(attrib.attribtype());
        //add the attribute
        self.attribs.push(attrib);
    }

    ///Sets/adds and attribute (builder pattern)
    pub fn with_attrib(mut self, attrib: Attribute) -> Self {
        self.set_attrib(attrib);
        self
    }

    ///Sets all attributes at once (takes ownership)
    pub fn set_attribs(&mut self, attribs: Vec<Attribute>) {
        self.attribs = attribs;
    }

    ///Sets all attributes at once (takes ownership)
    pub fn with_attribs(mut self, attribs: Vec<Attribute>) -> Self {
        self.set_attribs(attribs);
        self
    }



    ///High-level builder method to provide the span
    pub fn with_span(mut self, span_ids: &[&str]) -> Self {
        for id in span_ids.iter() {
            self = self.add_element(ElementData::new(ElementType::WordReference).with_attrib(Attribute::Idref(id.to_string())));
        }
        self
    }

    ///High-level builder method to provide textcontent
    pub fn with_text(self, text: String) -> Self {
        self.add_element(ElementData::new(ElementType::TextContent).with(DataType::Text(text)))
    }

    ///High-level builder method: add child to be constructed. The actual encoding happens later
    ///when ``Document.add_element()`` is called.
    pub fn add_element(mut self, data: ElementData) -> Self {
        self.data.push(DataType::AddElement(data));
        self
    }


    ///Low-level add function
    pub fn push(&mut self, datatype: DataType) {
        self.data.push(datatype);
    }

    ///Builder variant for push that adds data
    pub fn with(mut self, data: DataType) -> Self {
        self.data.push(data);
        self
    }

    ///Low-level add function, element children are expressed as keys (``DataType::Element(key)``)
    pub fn with_children(mut self, data: Vec<DataType>) -> Self {
        for dt in data.into_iter() {
            self.data.push(dt);
        }
        self
    }

    ///Returns the key of the parent element of this element
    pub fn parent_key(&self) -> Option<ElementKey> {
        self.parent
    }


    ///Sets the key of the parent element of this element
    pub fn set_parent_key(&mut self, parent: Option<ElementKey>) {
        self.parent = parent;
    }

    ///Builder method (can be chained) that sets the key of the parent element of this element
    pub fn with_parent_key(mut self, parent: Option<ElementKey>) -> Self {
        self.set_parent_key(parent);
        self
    }

    ///Low-level get function
    pub fn get_data_at(&self, index: usize) -> Option<&DataType> {
        self.data.get(index)
    }

    ///Returns the number of data items contained in this element
    pub fn len(&self) -> usize {
        self.data.len()
    }

    ///Returns the index of the specified data item (``refchild``)
    pub fn index(&self, refchild: &DataType) -> Option<usize> {
        self.data.iter().position(|child| *child == *refchild)
    }

    ///Remove (and return) the child at the specified index, does not delete it from the underlying store
    pub fn remove(&mut self, index: usize) -> Option<DataType> {
        if index >= self.data.len() {
            None
        } else {
            Some(self.data.remove(index))
        }
    }


    ///Simple constructor for an empty element (optionally with attributes)
    pub fn new(elementtype: ElementType) -> ElementData {
        Self { elementtype: elementtype, attribs: Vec::new(), data: Vec::new(), key: None, parent: None }
    }


}




