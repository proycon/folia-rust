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

use quick_xml::Reader;

use crate::common::*;
use crate::types::*;
use crate::error::*;
use crate::attrib::*;
use crate::elementstore::*;
use crate::store::*;
use crate::metadata::*;
use crate::select::*;
use crate::text::*;
use crate::document::{Document};




pub enum ValidationStrategy {
    NoValidation,
    ShallowValidation,
    DeepValidation
}


#[derive(Default,Clone)]
///Encoded attributes represents attributes that are encoded, i.e. attributes that are mapped to a
///numeric key rather than left as decoded strings. Encodes attributes are: Declarations (sets), classes and processors.
pub struct EncodedAttributes {
    //encoded (relation to other stores)
    pub processor: Option<ProcKey>,
    pub declaration: Option<DecKey>,
    pub class: Option<ClassKey>
}

#[derive(Clone)]
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

    //encoded attributes
    pub(crate) enc_attribs: Option<EncodedAttributes>,
}

#[derive(Clone,Copy)]
///Interface to a FoLiA element
pub struct Element<'a> {
    pub(crate) document: Option<&'a Document>,
    pub(crate) data: &'a ElementData
}

impl<'a> Element<'a> {

    pub(crate) fn elementdata(&self) -> &'a ElementData {
        self.data
    }


}


impl Storable<ElementKey> for ElementData {
    fn maybe_id(&self) -> Option<Cow<str>> {
        if let Some(attrib) = self.attrib(AttribType::ID) {
            Some(attrib.value())
        } else {
            None
        }
    }

    fn is_encoded(&self) -> bool {
        self.enc_attribs.is_some()
    }

    ///Returns the key of the current element
    fn key(&self) -> Option<ElementKey> {
        self.key
    }

    ///Sets the key of the current element
    fn set_key(&mut self, key: ElementKey) {
        self.key = Some(key);
    }

}

impl ElementData {

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


    ///Get Attribute
    pub fn attrib(&self, atype: AttribType) -> Option<&Attribute> {
        for attribute in self.attribs.iter() {
            if attribute.attribtype() == atype {
                return Some(attribute);
            }
        }
        None
    }


    ///Get attribute value as a string
    pub fn attrib_string(&self, atype: AttribType) -> Option<String> {
        if let Some(attrib) = self.attrib(atype) {
            if let Cow::Borrowed(s) = attrib.value() {
                Some(s.to_owned())
            }  else {
                None
            }
        } else {
            None
        }
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
    ///Sets all encoded attributes at once (takes ownership)
    pub fn set_enc_attribs(&mut self, enc_attribs: Option<EncodedAttributes>) {
        self.enc_attribs = enc_attribs;
    }

    ///Sets all attributes at once (takes ownership)
    pub fn with_attribs(mut self, attribs: Vec<Attribute>) -> Self {
        self.set_attribs(attribs);
        self
    }

    ///Sets all encoded attributes at once (takes ownership)
    pub fn with_enc_attribs(mut self, enc_attribs: Option<EncodedAttributes>) -> Self {
        self.set_enc_attribs(enc_attribs);
        self
    }

    //attribute getters (shortcuts)

    ///Unencoded class. This only works on decoded elements (returns None otherwise) and does no
    ///decoding itself, use decode_set() if you need to decode
    pub fn class_as_str(&self) -> Option<&str> {
        if let Some(attrib) = self.attrib(AttribType::CLASS) {
            if let Cow::Borrowed(s) = attrib.value() { //assumes value is always a borrowed one
                return Some(s);
            }
        }
        None
    }

    ///Unencoded set. This only works on decoded elements (returns None otherwise) and does no
    ///decoding itself
    pub fn set_as_str(&self) -> Option<&str> {
        if let Some(attrib) = self.attrib(AttribType::SET) {
            if let Cow::Borrowed(s) = attrib.value() { //assumes value is always a borrowed one
                return Some(s);
            }
        }
        None
    }

    ///Unencoded processor. This only works on decoded elements (returns None otherwise) and does
    ///no decoding itself
    pub fn processor_as_str(&self) -> Option<&str> {
        if let Some(attrib) = self.attrib(AttribType::PROCESSOR) {
            if let Cow::Borrowed(s) = attrib.value() { //assumes value is always a borrowed one
                return Some(s);
            }
        }
        None
    }

    ///Get the declaration from the declaration store, given an encoded element
    pub fn declaration<'a>(&self, document: &'a Document) -> (Option<&'a Declaration>) {
        if let Some(declaration_key) = self.declaration_key() {
           document.get_declaration(declaration_key)
        } else {
            None
        }
    }

    ///Get the processor from the provenance store, given an encoded element
    pub fn processor<'a>(&self, document: &'a Document) -> (Option<&'a Processor>) {
        if let Some(processor_key) = self.processor_key() {
            document.get_processor(processor_key)
        } else {
            None
        }
    }

    ///Get set as a str from an encoded element.
    pub fn set_decode<'a>(&self, document: &'a Document) -> (Option<&'a str>) {
        if let Some(declaration) = self.declaration(document) {
                return declaration.set.as_ref().map(|s| &**s);
        }
        None
    }

    ///Get a class as a str from an encoded element
    pub fn class_decode<'a>(&self, document: &'a Document) -> (Option<&'a str>) {
        if let Some(class_key) = self.class_key() {
            if let Some(declaration) = self.declaration(document) {
                if let Some(classes) = &declaration.classes {
                    if let Some(class) = classes.get(class_key) {
                        return Some(class.as_str());
                    }
                }
            }
        }
        None
    }

    pub fn processor_decode<'a>(&self, document: &'a Document) -> (Option<&'a str>) {
        if let Some(processor) = self.processor(document) {
            Some(processor.id.as_str())
        } else {
            None
        }
    }


    ///Get the set (encoded) aka the declaration key
    pub fn declaration_key(&self) -> Option<DecKey> {
        if let Some(enc_attribs) = &self.enc_attribs {
            enc_attribs.declaration
        } else {
            None
        }
    }


    ///Get the class (encoded) aka the class keyy
    pub fn class_key(&self) -> Option<ClassKey> {
        if let Some(enc_attribs) = &self.enc_attribs {
            enc_attribs.class
        } else {
            None
        }
    }

    ///Get the processor (encoded) aka the processor keyy
    pub fn processor_key(&self) -> Option<ProcKey> {
        if let Some(enc_attribs) = &self.enc_attribs {
            enc_attribs.processor
        } else {
            None
        }
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

    ///Low-level add function
    pub fn with_data(mut self, data: Vec<DataType>) -> Self {
        for dt in data.into_iter() {
            self.data.push(dt);
        }
        self
    }


    ///Returns the key of the parent element of this element
    pub fn get_parent(&self) -> Option<ElementKey> {
        self.parent
    }

    ///Returns the key of the processor associated with this element
    pub fn get_processor(&self) -> Option<ProcKey> {
        self.enc_attribs.as_ref().map(|enc_attribs| enc_attribs.processor).and_then(std::convert::identity) //and then flattens option (no flatten() in stable rust yet)
    }

    ///Returns the key of the declaration associated with this element
    pub fn get_declaration(&self) -> Option<DecKey> {
        self.enc_attribs.as_ref().map(|enc_attribs| enc_attribs.declaration).and_then(std::convert::identity) //and then flattens option (no flatten() in stable rust yet)
    }

    ///Sets the key of the parent element of this element
    pub fn set_parent(&mut self, parent: Option<ElementKey>) {
        self.parent = parent;
    }

    ///Builder method (can be chained) that sets the key of the parent element of this element
    pub fn with_parent(mut self, parent: Option<ElementKey>) -> Self {
        self.set_parent(parent);
        self
    }

    ///Low-level get function
    pub fn get(&self, index: usize) -> Option<&DataType> {
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
        Self { elementtype: elementtype, attribs: Vec::new(), data: Vec::new(), key: None, parent: None, enc_attribs: None }
    }

    ///Create a new element and assumes it is already encoded (though empty), so the user shouldn't pass any unencoded attributes (OBSOLETE?)
    pub fn new_as_encoded(elementtype: ElementType) -> ElementData {
        Self { elementtype: elementtype, attribs: Vec::new(), data: Vec::new(), parent: None, key: None, enc_attribs: Some(EncodedAttributes::default()) }
    }


}




