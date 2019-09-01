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
            BodyType::Text => FoliaElement::new(ElementType::Text),
            BodyType::Speech => FoliaElement::new(ElementType::Speech),
        });
        Ok(Self {
            id: id.to_string(),
            filename: None,
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
        let mut result = Self::parse(&mut reader);
        if let Ok(ref mut doc) = result {
            //associate the filename with the document
            doc.filename = Some(filename.to_string());
        }
        return result;
    }

    ///Load a FoliA document from XML string representation
    pub fn from_str(data: &str) -> Result<Self, FoliaError> {
        let mut reader = Reader::from_str(data);
        reader.trim_text(true);
        Self::parse(&mut reader)
    }


    ///Add an element to the document, this will result in an orphaned element, use add_to() instead
    pub fn add(&mut self, element: FoliaElement) -> Result<IntId, FoliaError> {
        let element = element.encode(&mut self.declarationstore, &mut self.provenancestore)?;
        self.elementstore.add(element)
    }


    ///Get an element from the document
    pub fn remove(&mut self, intid: IntId) {
        //self.elementstore.remove(intid)
        unimplemented!()
    }

    pub fn add_to(&mut self, parent_intid: IntId, element: FoliaElement) -> Result<IntId, FoliaError> {
        let element = element.encode(&mut self.declarationstore, &mut self.provenancestore)?;
        self.elementstore.add_to(parent_intid, element)
    }

    pub fn add_processor(&mut self, processor: Processor) -> Result<ProcIntId, FoliaError> {
        unimplemented!();
    }

    pub fn declare(&mut self, declaration: Declaration) -> Result<DecIntId, FoliaError> {
        self.declarationstore.add(declaration)
    }

    pub fn id(&self) -> &str { &self.id }
    pub fn filename(&self) -> Option<&str> { self.filename.as_ref().map(String::as_str) } //String::as_str equals  |x| &**x




}
