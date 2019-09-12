use std::path::{Path};
use std::io::BufRead;
use std::io::BufReader;
use std::io::Cursor;
use std::fs::File;
use std::str;
use std::str::FromStr;
use std::borrow::ToOwned;
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
    pub elementstore: ElementStore,
    pub provenancestore: ProvenanceStore,
    pub declarationstore: DeclarationStore,
    pub metadata: Metadata,
}



impl Document {
    ///Create a new FoLiA document from scratch
    pub fn new(id: &str, bodytype: BodyType) -> Result<Self, FoliaError> {
        let mut elementstore = ElementStore::default();
        elementstore.add(match bodytype {
            BodyType::Text => FoliaElement::new_encoded(ElementType::Text),
            BodyType::Speech => FoliaElement::new_encoded(ElementType::Speech),
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

    ///Load a FoliA document from file
    pub fn from_file(filename: &str) -> Result<Self, FoliaError> {
        let mut reader = Reader::from_file(Path::new(filename))?;
        reader.trim_text(true);
        let mut doc = Self::parse(&mut reader)?;
        //associate the filename with the document
        doc.filename = Some(filename.to_string());
        Ok(doc)
    }

    ///Load a FoliA document from XML string representation
    pub fn from_str(data: &str) -> Result<Self, FoliaError> {
        let mut reader = Reader::from_str(data);
        reader.trim_text(true);
        Self::parse(&mut reader)
    }


    ///Add an element to the document, this will result in an orphaned element, use add_to() instead
    pub fn add(&mut self, element: FoliaElement) -> Result<ElementKey, FoliaError> {
        let element = element.encode(&mut self.declarationstore, &mut self.provenancestore)?;
        self.elementstore.add(element)
    }


    ///REmove an element from the document
    pub fn remove(&mut self, key: ElementKey) {
        //self.elementstore.remove(key)
        unimplemented!()
    }

    pub fn add_to(&mut self, parent_key: ElementKey, element: FoliaElement) -> Result<ElementKey, FoliaError> {
        let element = element.encode(&mut self.declarationstore, &mut self.provenancestore)?;
        self.elementstore.add_to(parent_key, element)
    }

    pub fn add_processor(&mut self, processor: Processor) -> Result<ProcKey, FoliaError> {
        self.provenancestore.add(processor)
    }

    pub fn add_declaration(&mut self, declaration: Declaration) -> Result<DecKey, FoliaError> {
        self.declarationstore.add(declaration)
    }

    pub fn declare(&mut self, annotationtype: AnnotationType, set: &Option<String>, alias: &Option<String>) -> Result<DecKey,FoliaError> {
        self.declarationstore.declare(annotationtype, set, alias)
    }

    pub fn id(&self) -> &str { &self.id }
    pub fn filename(&self) -> Option<&str> { self.filename.as_ref().map(String::as_str) } //String::as_str equals  |x| &**x




}
