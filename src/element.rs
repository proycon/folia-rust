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
pub struct FoliaElement {
    pub elementtype: ElementType,
    pub attribs: Vec<Attribute>,

    //encoded (inter-element)
    pub(crate) data: Vec<DataType>,
    pub(crate) key: Option<ElementKey>,
    pub(crate) parent: Option<ElementKey>,

    //encoded attributes
    pub(crate) enc_attribs: Option<EncodedAttributes>,
}

impl Storable<ElementKey> for FoliaElement {
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

impl FoliaElement {

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
        if let Some(declaration_key) = self.declaration_key() {
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
        Self { elementtype: elementtype, attribs: Vec::new(), data: Vec::new(), key: None, parent: None, enc_attribs: None }
    }

    ///Create a new element and assumes it is already encoded (though empty), so the user shouldn't pass any unencoded attributes (OBSOLETE?)
    pub fn new_as_encoded(elementtype: ElementType) -> FoliaElement {
        Self { elementtype: elementtype, attribs: Vec::new(), data: Vec::new(), parent: None, key: None, enc_attribs: Some(EncodedAttributes::default()) }
    }

    ///Returns the text content of a given element
    pub fn text(&self, doc: &Document, set: DecKey, textclass: ClassKey, strict: bool, retaintokenisation: bool, previousdelimiter: Option<String>) -> Result<String,FoliaError> {
        let key = self.key().ok_or(FoliaError::KeyError("Element has no key".to_string()))?;

        let properties = doc.props(self.elementtype);

        if properties.textcontainer {
            //we are a text container (<t> or markup or something)
            let mut text: String = String::new();
            for item in self.data.iter()  {
                match item {
                    DataType::Text(item_text) => {
                        text += &item_text;
                    },
                    DataType::Element(element_key) => {
                        if let Some(element) = doc.elementstore.get(*element_key) {
                            let properties = doc.props(element.elementtype);
                            if properties.printable {
                                if !text.is_empty() {
                                    if let Some(textdelimiter) = properties.textdelimiter {
                                        text += textdelimiter;
                                    }
                                }
                                let textpart = element.text(doc,set,textclass,strict, retaintokenisation,None)?;
                                text += &textpart;
                            }
                        }
                    },
                    _ => {},
                }
            }
            Ok(text)
        } else if !properties.printable || properties.hidden {
            Err(FoliaError::NoTextError("No such text".to_string()))
        } else {
            //Get text from children first
            let mut delimiter: String = String::new();
            let mut text: String = String::new();
            let mut textcontent_element: Option<&FoliaElement> = None;
            for element in self.data.iter() {
                if let DataType::Element(element_key) = element {
                    if let Some(element) = doc.elementstore.get(*element_key) {
                        if ElementGroup::Structure.contains(element.elementtype) ||
                           element.elementtype == ElementType::Correction ||
                           ElementGroup::Span.contains(element.elementtype) {

                           if let Ok(textpart) = element.text(doc,set,textclass,false, retaintokenisation, Some(delimiter.clone())) {
                               //delimiter will be buffered and only printed upon next iteration
                               text += &textpart;
                               delimiter = element.get_textdelimiter(doc, retaintokenisation).to_string();
                           }
                        } else if element.elementtype == ElementType::TextContent {
                            textcontent_element = Some(element);
                        }
                    }
                }
            }
            if text.is_empty() && textcontent_element.is_some() {
                if let Ok(parttext) = textcontent_element.unwrap().text(doc,set,textclass,false,retaintokenisation, None) {
                    text = parttext
                }
            }

            if !text.is_empty() && previousdelimiter.is_some() {
                text = previousdelimiter.unwrap() + text.as_str();
            }

            if !text.is_empty() {
                Ok(text)
            } else {
                Err(FoliaError::NoTextError("No such text".to_string()))
            }
        }
    }

    ///Returns the text delimiter for this element
    pub fn get_textdelimiter(&self, doc: &Document, retaintokenisation: bool) -> &str {
        let properties =  doc.props(self.elementtype);
        if properties.textdelimiter.is_none() {
            //no text delimiter of itself, recurse into children to inherit delimiter
            for item in self.data.iter().rev() {
                if let DataType::Element(element_key) = item {
                }
            }
            ""
        } else if properties.optional_attribs.contains(&AttribType::SPACE) {
            let space: bool = retaintokenisation || match self.attrib(AttribType::SPACE) {
                Some(Attribute::Space(space)) => {
                    *space
                },
                _ => {
                    true
                }
            };
            if space {
                properties.textdelimiter.unwrap()
            } else {
                ""
            }
        } else {
            properties.textdelimiter.unwrap()
        }
    }

    ///Returns the text content of a given element
    ///This method takes string parameters for set and textclass, which can be set to None to
    ///fallback to the default text set and "current class".
    pub fn text_encode(&self, doc: &Document, set: Option<&str>, textclass: Option<&str>, strict: bool, retaintokenisation: bool) -> Result<String,FoliaError> {
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
        if let Some(dec_key) = doc.declarationstore.id_to_key(DeclarationStore::index_id(AnnotationType::TOKEN, &Some(set)).as_str()) {
            let class_key = doc.declarationstore.encode_class(dec_key, textclass)?;
            self.text(doc, dec_key, class_key,strict,retaintokenisation, None)
        } else {
            Err(FoliaError::EncodeError("No declaration for the specified text set/class".to_string()))
        }
    }

    /*
    pub fn textelement_encode(&self, element_key: ElementKey, set: Option<&str>, textclass: Option<&str>) -> Option<&FoliaElement> {
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
    */

}

/*
impl Select for FoliaElement {
    fn select(&self, selector: Selector) -> SelectIterator {
    }
}
*/

impl Encoder<FoliaElement> for Document {
    ///Actively encode for storage, this encodes attributes that need to be encoded (such as set,class,processor), and adds them to their respective stores.
    ///It does not handle relations between elements (data/children and parent)
    ///nor does it add the element itself to the store
    ///to the store).
    fn encode(&mut self, element: &mut FoliaElement) -> Result<(), FoliaError> {
        if element.is_encoded() {
            //already encoded, nothing to do
            return Ok(());
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
                    if let Ok(class_key) = self.declarationstore.add_class(deckey, class) {
                        enc_attribs.class = Some(class_key);
                    }
                }
            }

            if let Some(declaration) = self.declarationstore.get(deckey) {
                enc_attribs.processor = declaration.default_processor() //returns an Option, may be overriden later if a specific processor is et
            }
        }

        if let Some(processor) = element.attrib(AttribType::PROCESSOR) {
            let processor_id: &str  = &processor.value();

            if let Some(processor_key) = self.provenancestore.id_to_key(processor_id) {
                enc_attribs.processor = Some(processor_key); //overrides the earlier-set default (if any)
            }
        }

        //remove encoded attributes
        element.attribs.retain(|a| match a {
            Attribute::Set(_) | Attribute::Class(_) | Attribute::Processor(_) => false,
            _ => true
        });

        element.set_enc_attribs(Some(enc_attribs));

        Ok(())
    }
}

