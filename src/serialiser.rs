use std::io::Cursor;
use std::fs::File;
use std::str;
use std::str::FromStr;
use std::borrow::ToOwned;
use std::string::ToString;

use std::io::Write;
use std::io::BufWriter;
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
use crate::document::Document;

const NL: &[u8] = b"\n";

fn to_serialisation_error(err: quick_xml::Error) -> FoliaError {
    FoliaError::SerialisationError(format!("{}",err))
}

impl Document {
    ///Serialises a document to XML (vector of bytes, utf-8)
    pub fn xml(&self, root_key: ElementKey) -> Result<Vec<u8>, FoliaError> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut doc_start = BytesStart::borrowed(b"FoLiA", 5);
        doc_start.push_attribute(("xmlns", str::from_utf8(NSFOLIA).unwrap() ));
        doc_start.push_attribute(("xmlns:xlink", str::from_utf8(NSXLINK).unwrap() ));
        doc_start.push_attribute(("version",FOLIAVERSION ));
        doc_start.push_attribute(("generator", GENERATOR ));
        writer.write_event(Event::Start(doc_start)).map_err(to_serialisation_error)?;

        self.xml_metadata(&mut writer)?;

        self.xml_elements(&mut writer, root_key)?;

        writer.write_event(Event::End(BytesEnd::borrowed(b"FoLiA"))).map_err(to_serialisation_error)?;
        let result = writer.into_inner().into_inner();
        Ok(result)
    }

    fn xml_metadata(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> Result<(), FoliaError> {
        let mut metadata_start = BytesStart::borrowed(b"metadata", 8);
        if let Some(metadatatype) = &self.metadata.metadatatype {
            metadata_start.push_attribute(("type", metadatatype.as_str() ));
        }
        if let Some(src) = &self.metadata.src {
            metadata_start.push_attribute(("src", src.as_str() ));
        }
        writer.write_event(Event::Start(metadata_start)).map_err(to_serialisation_error)?;
        self.xml_declarations(writer)?;
        self.xml_provenance(writer)?;
        writer.write_event(Event::End(BytesEnd::borrowed(b"metadata"))).map_err(to_serialisation_error)?;
        writer.write_event(Event::Text(BytesText::from_plain(NL))).map_err(to_serialisation_error)?;
        Ok(())
    }

    fn xml_declarations(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> Result<(), FoliaError> {
        writer.write_event(Event::Start( BytesStart::borrowed(b"annotations", 11))).map_err(to_serialisation_error)?;
        for declaration in self.declarationstore.iter() {
            if let Some(declaration) = declaration {
                let tagname = format!("{}-annotation", declaration.annotationtype.as_str());
                let mut dec_start = BytesStart::owned_name(tagname.as_bytes());
                if let Some(set) = &declaration.set {
                    dec_start.push_attribute(("set", set.as_str() ));
                }
                if let Some(alias) = &declaration.alias {
                    dec_start.push_attribute(("alias", alias.as_str() ));
                }
                let dec_end = BytesEnd::owned(tagname.as_bytes().to_vec());
                writer.write_event(Event::Start(dec_start)).map_err(to_serialisation_error)?;
                writer.write_event(Event::End(dec_end)).map_err(to_serialisation_error)?;
            }
        }
        writer.write_event(Event::End(BytesEnd::borrowed(b"annotations"))).map_err(to_serialisation_error)?;
        Ok(())
    }

    fn xml_provenance(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> Result<(), FoliaError> {
        writer.write_event(Event::Start( BytesStart::borrowed(b"provenance", 11))).map_err(to_serialisation_error)?;
        writer.write_event(Event::End(BytesEnd::borrowed(b"provenance"))).map_err(to_serialisation_error)?;
        Ok(())
    }

    fn xml_elements(&self, writer: &mut Writer<Cursor<Vec<u8>>>, root_key: ElementKey) -> Result<(), FoliaError> {
        //Start the root tag (and obtain data for its end)
        let root_end = if let Some(element) = self.elementstore.get(root_key) {
            let tagstring = element.elementtype.to_string();
            let tag = tagstring.as_bytes();
            let start = BytesStart::owned(tag.to_vec(), tag.len());
            writer.write_event(Event::Start(start)).map_err(to_serialisation_error)?;
            BytesEnd::owned(tag.to_vec())
        } else {
            return Err(FoliaError::SerialisationError(format!("Specified root element not found: {}", root_key)));
        };

        //Select children
        let mut stack = vec![];
        let mut previous_depth = 0;
        for item in self.elementstore.select(root_key,Selector::new(TypeSelector::AnyType, SetSelector::AnySet),true) {
            while item.depth < previous_depth {
                if let Some(end) = stack.pop() {
                    writer.write_event(Event::End(end)).map_err(to_serialisation_error)?;
                } else {
                    return Err(FoliaError::SerialisationError("Unable to pop the end tag stack".to_string()));
                }
                previous_depth -= 1;
            }
            match item.data {
                DataType::Element(key) => {
                    if let Some(element) = self.elementstore.get(*key) {
                        let tagstring = element.elementtype.to_string();
                        let tag = tagstring.as_bytes();
                        let mut start = BytesStart::owned(tag.to_vec(), tag.len());
                        for attrib in element.attribs.iter() {
                            let value: &str = &attrib.value();
                            start.push_attribute((attrib.attribtype().into(), value ));
                        }
                        //decode encoded attributes
                        if let Some(set) = element.decoded_set(&self.declarationstore) {
                            start.push_attribute(("set", set) );
                        }
                        if let Some(class) = element.decoded_class(&self.declarationstore) {
                            start.push_attribute(("class", class) );
                        }
                        if let Some(processor) = element.decoded_processor(&self.provenancestore) {
                            start.push_attribute(("processor", processor) );
                        }
                        writer.write_event(Event::Start(start)).map_err(to_serialisation_error)?;
                        let end = BytesEnd::owned(tag.to_vec());
                        stack.push(end);
                    }
                },
                DataType::Text(text) => {
                    let text = BytesText::from_plain_str(text.as_str());
                    writer.write_event(Event::Text(text)).map_err(to_serialisation_error)?;
                },
                DataType::Comment(comment) => {
                }
            }
            previous_depth = item.depth;
        }

        //don't forget the final closing elements
        while let Some(end) = stack.pop() {
            writer.write_event(Event::End(end)).map_err(to_serialisation_error)?;
        }

        //Write root end tag
        writer.write_event(Event::End(root_end)).map_err(to_serialisation_error)?;
        Ok(())
    }
}
