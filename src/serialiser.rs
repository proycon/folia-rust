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
use crate::document::Document;

impl Document {
    ///Serialises a document to XML (vector of bytes, utf-8)
    pub fn xml(&self, root_key: ElementKey) -> Result<Vec<u8>, FoliaError> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        //Start the root tag (and obtain data for its end)
        let root_end = if let Some(element) = self.elementstore.get(root_key) {
            let tagstring = element.elementtype.to_string();
            let tag = tagstring.as_bytes();
            let start = BytesStart::owned(tag.to_vec(), tag.len());
            writer.write_event(Event::Start(start)).map_err(|err| FoliaError::SerialisationError(format!("{}",err)))?;
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
                    writer.write_event(Event::End(end)).map_err(|err| FoliaError::SerialisationError(format!("{}",err)))?;
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
                        writer.write_event(Event::Start(start)).map_err(|err| FoliaError::SerialisationError(format!("{}",err)))?;
                        let end = BytesEnd::owned(tag.to_vec());
                        stack.push(end);
                    }
                },
                DataType::Text(text) => {
                    let text = BytesText::from_plain_str(text.as_str());
                    writer.write_event(Event::Text(text)).map_err(|err| FoliaError::SerialisationError(format!("{}",err)))?;
                },
                DataType::Comment(comment) => {
                }
            }
            previous_depth = item.depth;
        }

        //don't forget the final closing elements
        while let Some(end) = stack.pop() {
            writer.write_event(Event::End(end)).map_err(|err| FoliaError::SerialisationError(format!("{}",err)))?;
        }

        //Write root end tag
        writer.write_event(Event::End(root_end)).map_err(|err| FoliaError::SerialisationError(format!("{}",err)))?;
        let result = writer.into_inner().into_inner();
        Ok(result)
    }
}
