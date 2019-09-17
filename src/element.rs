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
use crate::parser::*;
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

#[derive(Default,Clone)]
pub struct EncodedAttributes {
    //encoded (relation to other stores)
    processor: Option<ProcKey>,
    declaration: Option<DecKey>,
    class: Option<ClassKey>
}

#[derive(Clone)]
pub struct FoliaElement {
    pub elementtype: ElementType,
    pub attribs: Vec<Attribute>,

    //encoded (inter-element)
    data: Vec<DataType>,
    parent: Option<ElementKey>,

    //encoded attributes
    enc_attribs: Option<EncodedAttributes>,
}


impl MaybeIdentifiable for FoliaElement {
    fn maybe_id(&self) -> Option<Cow<str>> {
        if let Some(attrib) = self.attrib(AttribType::ID) {
            Some(attrib.value())
        } else {
            None
        }
    }
}

impl CheckEncoded for FoliaElement {
    fn encoded(&self) -> bool {
        self.enc_attribs.is_some()
    }
}

impl FoliaElement {
    ///Encode for storage, this only encodes attributes (set,class,processor) maintained
    ///in other stores, which are inherent to the element.
    ///It does not handle relations between elements (data/children and parent)
    ///nor does it add the element itself to the store (but this is instead invoked as part of adding an element
    ///to the store). This function takes and returns ownership.
    pub fn encode(mut self, declarationstore: &mut DeclarationStore, provenancestore: &mut ProvenanceStore) -> Result<Self, FoliaError> {
        let mut enc_attribs: EncodedAttributes = EncodedAttributes::default();

        //encode the element for storage
        let set = self.attrib(AttribType::SET);

        if let Some(annotationtype) = self.elementtype.annotationtype() {
            //Declare the element (either declares anew or just resolves the to the right
            //declaration.
            let deckey = declarationstore.declare(annotationtype, &set.map(|x| x.value().into_owned() ), &None)?;
            enc_attribs.declaration = Some(deckey);

            if let Some(class) = self.attrib(AttribType::CLASS) {
                if let Attribute::Class(class) = class {
                    if let Ok(class_key) = declarationstore.add_class(deckey, class) {
                        enc_attribs.class = Some(class_key);
                    }
                }
            }

            if let Some(declaration) = declarationstore.get(deckey) {
                enc_attribs.processor = declaration.default_processor() //returns an Option, may be overriden later if a specific processor is et
            }
        }

        if let Some(processor) = self.attrib(AttribType::PROCESSOR) {
            let processor_id: &str  = &processor.value();

            if let Some(processor_key) = provenancestore.id_to_key(processor_id) {
                enc_attribs.processor = Some(processor_key); //overrides the earlier-set default (if any)
            }
        }

        //remove encoded attributes
        self.attribs.retain(|a| match a {
            Attribute::Set(_) | Attribute::Class(_) | Attribute::Processor(_) => false,
            _ => true
        });

        self.set_enc_attribs(Some(enc_attribs));

        Ok(self)
    }

    ///Decodes an element and returns a **copy**, therefore it should be used sparingly.
    ///It does not decode relations between elements (data/children and parent), only set, class
    ///and processor.
    pub fn decode(&self, declarationstore: &DeclarationStore, provenancestore: &ProvenanceStore) -> Self {
        let mut decoded_attribs: Vec<Attribute> = self.attribs.iter().map(|a| a.clone()).collect();
        if let Some(set) = self.set_decode(declarationstore) {
            decoded_attribs.push(Attribute::Set(set.to_string()));
        }
        if let Some(class) = self.class_decode(declarationstore) {
            decoded_attribs.push(Attribute::Class(class.to_string()));
        }
        if let Some(processor) = self.processor_decode(provenancestore) {
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
    pub fn declaration<'a>(&self, declarationstore: &'a DeclarationStore) -> (Option<&'a Declaration>) {
        if let Some(declaration_key) = self.set_key() {
           declarationstore.get(declaration_key).map(|b| &**b)
        } else {
            None
        }
    }

    ///Get the processor from the provenance store, given an encoded element
    pub fn processor<'a>(&self, provenancestore: &'a ProvenanceStore) -> (Option<&'a Processor>) {
        if let Some(processor_key) = self.processor_key() {
            provenancestore.get(processor_key).map(|b| &**b)
        } else {
            None
        }
    }

    ///Get set as a str from an encoded element.
    pub fn set_decode<'a>(&self, declarationstore: &'a DeclarationStore) -> (Option<&'a str>) {
        if let Some(declaration) = self.declaration(declarationstore) {
                return declaration.set.as_ref().map(|s| &**s);
        }
        None
    }

    ///Get a class as a str from an encoded element
    pub fn class_decode<'a>(&self, declarationstore: &'a DeclarationStore) -> (Option<&'a str>) {
        if let Some(class_key) = self.class_key() {
            if let Some(declaration) = self.declaration(declarationstore) {
                if let Some(classes) = &declaration.classes {
                    if let Some(class) = classes.get(class_key) {
                        return Some(class.as_str());
                    }
                }
            }
        }
        None
    }

    pub fn processor_decode<'a>(&self, provenancestore: &'a ProvenanceStore) -> (Option<&'a str>) {
        if let Some(processor) = self.processor(provenancestore) {
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

    ///Alias for declaration_key
    pub fn set_key(&self) -> Option<DecKey> {
        self.declaration_key()
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

    pub fn get_parent(&self) -> Option<ElementKey> {
        self.parent
    }

    pub fn get_processor(&self) -> Option<ProcKey> {
        self.enc_attribs.as_ref().map(|enc_attribs| enc_attribs.processor).and_then(std::convert::identity) //and then flattens option (no flatten() in stable rust yet)
    }

    pub fn get_declaration(&self) -> Option<DecKey> {
        self.enc_attribs.as_ref().map(|enc_attribs| enc_attribs.declaration).and_then(std::convert::identity) //and then flattens option (no flatten() in stable rust yet)
    }

    pub fn set_parent(&mut self, parent: Option<ElementKey>) {
        self.parent = parent;
    }

    pub fn with_parent(mut self, parent: Option<ElementKey>) -> Self {
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

    ///Remove (and return) the child at the specified index, does not delete it from the underlying store
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
        Self { elementtype: elementtype, attribs: Vec::new(), data: Vec::new(), parent: None, enc_attribs: None }
    }

    ///Create a new element and assumes it is already encoded (though empty), so the user shouldn't pass any unencoded attributes
    pub fn new_as_encoded(elementtype: ElementType) -> FoliaElement {
        Self { elementtype: elementtype, attribs: Vec::new(), data: Vec::new(), parent: None, enc_attribs: Some(EncodedAttributes::default()) }
    }

    ///Returns the text content of a given element, only makes sense if the element is a text
    pub fn text(&self, elementstore: &ElementStore, set: Option<DecKey>, textclass: Option<ClassKey>) -> Result<Cow<str>,FoliaError> {
        unimplemented!()
    }

}

/*
impl Select for FoliaElement {
    fn select(&self, selector: Selector) -> SelectIterator {
    }
}
*/

