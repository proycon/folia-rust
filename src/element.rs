use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::borrow::Cow;
use std::str::{FromStr,from_utf8};
use std::string::ToString;
use std::fmt;
use std::iter::ExactSizeIterator;
use std::convert::Into;

use quick_xml::Reader;

use crate::common::*;
use crate::types::*;
use crate::error::*;
use crate::attrib::*;
use crate::elementstore::*;
use crate::store::*;
use crate::metadata::*;
use crate::document::{Document};




pub enum ValidationStrategy {
    NoValidation,
    ShallowValidation,
    DeepValidation
}

pub struct Properties {
    xmltag: String,
    annotationtype: AnnotationType,
    accepted_data: Vec<ElementType>,
    required_attribs: Vec<AttribType>,
    optional_attribs: Vec<AttribType>,
    occurrences: u32, //How often can this element occur under the parent? (0 = unlimited)
    occurrences_per_set: u32, //How often can a particular element+set combination occur under the parent (0 = unlimited)
    textdelimiter: Option<String>, //Delimiter to use when dynamically gathering text
    printable: bool, //Is this element printable? (i.e. can the text() method be called?)
    speakable: bool, //Is this element phonetically representablly? (i.e. can the phon() method be called?)
    hidden: bool, //Is this element hidden? (only applies to Hiddenword for now)
    xlink: bool, //Can the element carry xlink references?
    textcontainer: bool, //Does the element directly take textual content (e.g. TextContent (t) is a textcontainer)
    phoncontainer: bool, //Does the element directly take phonetic content (e.g. PhonContent (ph) is a phoncontainer)
    subset: Option<String>, //used for Feature subclasses
    auth: bool, //The default authoritative state for this element
    primaryelement: bool, //Is this the primary element for the advertised annotation type?
    auto_generate_id: bool, //Automatically generate an ID if none was provided?
    setonly: bool, //States that the element may take a set property only, and not a class property
    wrefable: bool //Indicates whether this element is referable as a token/word (applies only to a very select few elements, such as w, morpheme, and phoneme)
}

pub struct FoliaElement {
    pub elementtype: ElementType,
    pub attribs: Vec<Attribute>,

    //encoded (inter-element)
    data: Vec<DataType>,
    parent: Option<IntId>,

    //encoded (relation to other stores)
    processor: Option<ProcIntId>,
    declaration: Option<DecIntId>,
    class: Option<ClassIntId>
}

impl MaybeIdentifiable for FoliaElement {
    fn id(&self) -> Option<String> {
        self.attrib_string(AttribType::ID)
    }
}

impl FoliaElement {
    ///Encode for storage, this only encodes attributes (set,class,processor) maintained
    ///in other stores, which are inherent to the element.
    ///It does not handle relations between elements (data/children and parent)
    ///nor does it add the element itself to the store (but this is instead invoked as part of adding an element
    ///to the store). This function takes and returns ownership.
    ///
    pub fn encode(mut self, declarationstore: &mut DeclarationStore, provenancestore: &mut ProvenanceStore) -> Result<Self, FoliaError> {
        //encode the element for storage
        let set: Option<&Attribute> = self.attribs.iter().find(|&a| {
            match a {
                Attribute::Set(_) => true,
                _ => false,
            }
        });
        let set: Option<Cow<str>> = set.map(|a| a.value() );

        if let Some(annotationtype) = self.elementtype.annotationtype() {
            let decintid = declarationstore.add(Declaration::new(annotationtype, set.map(|x| x.into_owned() )))?;
            self.declaration = Some(decintid);
        }
        //TODO: handle processor and class

        //remove encoded attributes
        self.attribs.retain(|a| match a {
            Attribute::Set(_) | Attribute::Class(_) | Attribute::Processor(_) => false,
            _ => true
        });

        Ok(self)
    }
}


impl FoliaElement {

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

    ///Sets all attributes at once (takes ownership)
    pub fn with_attribs(mut self, attribs: Vec<Attribute>) -> Self {
        self.set_attribs(attribs);
        self
    }

    //attribute getters (shortcuts)
    pub fn class(&self) -> Option<String> { self.attrib_string(AttribType::CLASS)  }
    pub fn set(&self) -> Option<String> { self.attrib_string(AttribType::SET)  }
    pub fn processor(&self) -> Option<String> { self.attrib_string(AttribType::PROCESSOR)  }

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

    pub fn get_parent(&self) -> Option<IntId> {
        self.parent
    }

    pub fn get_processor(&self) -> Option<ProcIntId> {
        self.processor
    }

    pub fn get_declaration(&self) -> Option<DecIntId> {
        self.declaration
    }

    pub fn set_parent(&mut self, parent: Option<IntId>) {
        self.parent = parent;
    }

    pub fn with_parent(mut self, parent: Option<IntId>) -> Self {
        self.set_parent(parent);
        self
    }

    ///Low-level get function
    pub fn get(&self, index: usize) -> Option<&DataType> {
        self.data.get(index)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn index(&self, refchild: &DataType) -> Option<usize> {
        self.data.iter().position(|child| *child == *refchild)
    }

    ///Remove (and return) the child at the specified index
    pub fn remove(&mut self, index: usize) -> Option<DataType> {
        if index >= self.data.len() {
            None
        } else {
            Some(self.data.remove(index))
        }
    }

    /*
    pub fn get_mut(&mut self, index: usize) -> Option<&mut DataType> {
        self.data.get_mut(index)
    }

    pub fn get_mut_last(&mut self) -> Option<&mut DataType> {
        let index = self.data.len() - 1;
        self.data.get_mut(index)
    }

    pub fn get_last(&self) -> Option<&DataType> {
        let index = self.data.len() - 1;
        self.data.get(index)
    }
    */

    ///Simple constructor for an empty element (optionally with attributes)
    pub fn new(elementtype: ElementType) -> FoliaElement {
        Self { elementtype: elementtype, attribs: Vec::new(), data: Vec::new(), parent: None, declaration: None, processor: None, class: None }
    }

    pub fn parse_attributes<R: BufRead>(reader: &Reader<R>, attribiter: quick_xml::events::attributes::Attributes) -> Result<Vec<Attribute>, FoliaError> {
        let mut attributes: Vec<Attribute> = Vec::new();
        for attrib in attribiter {
            match Attribute::parse(&reader, &attrib.unwrap()) {
                Ok(attrib) => { attributes.push(attrib); },
                Err(e) => { return Err(e); }
            }
        }
        Ok(attributes)
    }

    ///Parse this element from XML, note that this does not handle the child elements, those are
    ///appended by the main parser in Document::parse_body()
    pub fn parse<R: BufRead>(reader: &Reader<R>, event: &quick_xml::events::BytesStart) -> Result<FoliaElement, FoliaError> {
        let attributes: Vec<Attribute> = FoliaElement::parse_attributes(reader, event.attributes())?;
        let elementtype = ElementType::from_str(from_utf8(event.local_name()).unwrap())?;
        Ok(FoliaElement::new(elementtype).with_attribs(attributes))
    }
}

/*
impl Select for FoliaElement {
    fn select(&self, selector: Selector) -> SelectIterator {
    }
}
*/

