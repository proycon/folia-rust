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

    fn is_encoded(&self) -> bool {
        for attrib in self.attribs.iter() {
            if let Attribute::Class(_) | Attribute::Set(_) | Attribute::Processor(_) = attrib {
                    return false
            }
        }
        true
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
}

//public API

impl<'a> Element<'a> {
    fn attribs(&self) -> &Vec<Attribute> {
        &self.data.attribs()
    }

    pub fn elementtype(&self) -> ElementType {
        self.data.elementtype
    }

    ///Get the FoliA set
    pub fn set(&self) -> Option<&str> {
        if let Some(declaration) = self.get_declaration() {
            declaration.set.map(|s| s.as_str())
        } else {
            None
        }
    }

    ///Get the FoLiA class
    pub fn class(&self) -> Option<&str> {
        if let Some(class_key) =  self.class_key() {
            if let Some(declaration) = self.get_declaration() {
                return declaration.get_class(class_key);
            }
        }
        None
    }

    ///Get the processor ID
    pub fn processor(&self) -> Option<&str> {
        if let Some(processor) = self.get_processor() {
            Some(processor.id.as_str())
        } else {
            None
        }
    }

    pub fn class_key(&self) -> Option<ClassKey> {
        self.data.class_key().expect("Unwrapping class key result")
    }
    pub fn declaration_key(&self) -> Option<DecKey> {
        self.data.declaration_key().expect("Unwrapping declaration key result")
    }
    pub fn processor_key(&self) -> Option<ProcKey> {
        self.data.processor_key().expect("Unwrapping Processor key result")
    }
    pub fn parent_key(&self) -> Option<ElementKey> {
        self.data.parent_key()
    }

    ///Get the declaration instance
    pub fn get_declaration(&self) -> Option<&'a Declaration> {
        if self.document.is_none() {
            None
        } else {
            if let Some(declaration_key) = self.declaration_key() {
               self.document.unwrap().get_declaration(declaration_key)
            } else {
               None
            }
        }
    }

    ///Get the processor instance
    pub fn get_processor(&self) -> Option<&'a Processor> {
        if self.document.is_none() {
            None
        } else {
            if let Some(processor_key) = self.processor_key() {
                self.document.unwrap().get_processor(processor_key)
            } else {
                None
            }
        }
    }

    ///Get the parent element
    pub fn get_parent(&self) -> Option<Element> {
        if self.document.is_none() {
            None
        } else {
            if let Some(parent_key) =  self.parent_key() {
                self.document.unwrap().get_element(parent_key)
            } else {
                None
            }
        }
    }
}

impl<'a> PartialEq for Element<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.data.key().is_some() && self.data.key() == other.data.key()
    }
}
impl<'a> Eq for Element<'a> { }


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




