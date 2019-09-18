use std::path::{Path};
use std::io::BufRead;
use std::io::BufReader;
use std::io::Cursor;
use std::fs::File;
use std::str;
use std::str::FromStr;
use std::borrow::Cow;
use std::string::ToString;

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
        let mut elementstore = ElementStore::default();
        elementstore.add(match bodytype {
            BodyType::Text => FoliaElement::new_as_encoded(ElementType::Text),
            BodyType::Speech => FoliaElement::new_as_encoded(ElementType::Speech),
        })?;
        Ok(Self {
            id: id.to_string(),
            filename: None,
            version: FOLIAVERSION.to_string(),
            elementstore: elementstore,
            provenancestore:  ProvenanceStore::default(),
            declarationstore: DeclarationStore::default(),
            metadata: Metadata::default(),
        })
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


    ///Add an element to the document, this will result in an orphaned element, use ``add_to()`` instead
    pub fn add(&mut self, element: FoliaElement) -> Result<ElementKey, FoliaError> {
        let element = element.encode(&mut self.declarationstore, &mut self.provenancestore)?;
        self.elementstore.add(element)
    }


    ///Remove an element from the document
    pub fn remove(&mut self, key: ElementKey) {
        //self.elementstore.remove(key)
        unimplemented!()
    }

    ///Adds a new element (``element``) as a child of an existing one (``parent_key``). Takes
    ///ownership of the element. Returns the key.
    pub fn add_to(&mut self, parent_key: ElementKey, element: FoliaElement) -> Result<ElementKey, FoliaError> {
        let element = element.encode(&mut self.declarationstore, &mut self.provenancestore)?;
        self.elementstore.add_to(parent_key, element)
    }

    ///Add a processor to the the provenance chain
    ///Returns the key
    pub fn add_processor(&mut self, processor: Processor) -> Result<ProcKey, FoliaError> {
        self.provenancestore.add_to_chain(processor)
    }

    ///Add a declaration. It is strongly recommended to use ``declare()`` instead.
    ///Returns the key.
    pub fn add_declaration(&mut self, declaration: Declaration) -> Result<DecKey, FoliaError> {
        self.declarationstore.add(declaration)
    }

    ///Add a declaration. Returns the key. If the declaration already exists it simply returns the
    ///key of the existing one.
    pub fn declare(&mut self, annotationtype: AnnotationType, set: &Option<String>, alias: &Option<String>) -> Result<DecKey,FoliaError> {
        self.declarationstore.declare(annotationtype, set, alias)
    }

    ///Returns the ID of the document
    pub fn id(&self) -> &str { &self.id }

    ///Returns the filename associated with this document (i.e. the file from which it was loaded)
    pub fn filename(&self) -> Option<&str> { self.filename.as_ref().map(String::as_str) } //String::as_str equals  |x| &**x


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


    ///Returns the text of the given element
    pub fn text(&self, element_key: ElementKey, set: DecKey, textclass: ClassKey) -> Result<Cow<str>,FoliaError> {
        if let Some(element) = self.elementstore.get(element_key) {
            element.text(self, set, textclass)
        } else {
            Err(FoliaError::KeyError(format!("No such element key: {}", element_key)))
        }
    }

    ///Returns the text of the given element
    pub fn text_encode(&self, element_key: ElementKey, set: Option<&str>, textclass: Option<&str>) -> Result<Cow<str>,FoliaError> {
        if let Some(element) = self.elementstore.get(element_key) {
            element.text_encode(self, set, textclass)
        } else {
            Err(FoliaError::KeyError(format!("No such element key: {}", element_key)))
        }
    }




}
