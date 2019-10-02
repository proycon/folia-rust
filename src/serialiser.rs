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

        let mut doc_start = BytesStart::borrowed_name(b"FoLiA");
        doc_start.push_attribute(("xmlns", str::from_utf8(NSFOLIA).unwrap() ));
        doc_start.push_attribute(("xmlns:xlink", str::from_utf8(NSXLINK).unwrap() ));
        doc_start.push_attribute(("xml:id",self.id.as_str()));
        doc_start.push_attribute(("version",FOLIAVERSION ));
        doc_start.push_attribute(("generator", GENERATOR ));
        writer.write_event(Event::Start(doc_start)).map_err(to_serialisation_error)?;
        writer.write_event(Event::Text(BytesText::from_plain(NL))).map_err(to_serialisation_error)?;

        self.xml_metadata(&mut writer)?;

        self.xml_elements(&mut writer, root_key)?;

        writer.write_event(Event::Text(BytesText::from_plain(NL))).map_err(to_serialisation_error)?;
        writer.write_event(Event::End(BytesEnd::borrowed(b"FoLiA"))).map_err(to_serialisation_error)?;
        let result = writer.into_inner().into_inner();
        Ok(result)
    }

    fn xml_metadata(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> Result<(), FoliaError> {
        let mut metadata_start = BytesStart::borrowed_name(b"metadata");
        if let Some(metadatatype) = &self.metadata.metadatatype {
            metadata_start.push_attribute(("type", metadatatype.as_str() ));
        }
        if let Some(src) = &self.metadata.src {
            metadata_start.push_attribute(("src", src.as_str() ));
        }
        writer.write_event(Event::Start(metadata_start)).map_err(to_serialisation_error)?;
        writer.write_event(Event::Text(BytesText::from_plain(NL))).map_err(to_serialisation_error)?;
        self.xml_declarations(writer)?;
        self.xml_provenance(writer)?;
        for (meta_id, value) in self.metadata.data.iter() {
            let mut meta_start = BytesStart::borrowed_name(b"meta");
            meta_start.push_attribute(("id", meta_id.as_str() ));
            writer.write_event(Event::Start(meta_start)).map_err(to_serialisation_error)?;
            writer.write_event(Event::Text(BytesText::from_plain_str(value))).map_err(to_serialisation_error)?;
            writer.write_event(Event::End(BytesEnd::borrowed(b"meta"))).map_err(to_serialisation_error)?;
            writer.write_event(Event::Text(BytesText::from_plain(NL))).map_err(to_serialisation_error)?;
        }
        //there is a bit too much duplication going on here, to be solved later
        for (submetadata_id, submetadata) in self.submetadata.iter() {
            let mut submetadata_start = BytesStart::borrowed_name(b"submetadata");
            submetadata_start.push_attribute(("xml:id", submetadata_id.as_str() ));
            if let Some(metadatatype) = &submetadata.metadatatype {
                submetadata_start.push_attribute(("type", metadatatype.as_str() ));
            }
            if let Some(src) = &self.metadata.src {
                submetadata_start.push_attribute(("src", src.as_str() ));
            }
            for (meta_id, value) in submetadata.data.iter() {
                let mut meta_start = BytesStart::borrowed_name(b"meta");
                meta_start.push_attribute(("id", meta_id.as_str() ));
                writer.write_event(Event::Start(meta_start)).map_err(to_serialisation_error)?;
                writer.write_event(Event::Text(BytesText::from_plain_str(value))).map_err(to_serialisation_error)?;
                writer.write_event(Event::End(BytesEnd::borrowed(b"meta"))).map_err(to_serialisation_error)?;
                writer.write_event(Event::Text(BytesText::from_plain(NL))).map_err(to_serialisation_error)?;
            }
        }
        writer.write_event(Event::End(BytesEnd::borrowed(b"metadata"))).map_err(to_serialisation_error)?;
        writer.write_event(Event::Text(BytesText::from_plain(NL))).map_err(to_serialisation_error)?;
        Ok(())
    }

    fn xml_declarations(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> Result<(), FoliaError> {
        writer.write_event(Event::Start( BytesStart::borrowed_name(b"annotations"))).map_err(to_serialisation_error)?;
        writer.write_event(Event::Text(BytesText::from_plain(NL))).map_err(to_serialisation_error)?;
        for declaration in self.declarations() {
            if let Some(declaration) = declaration {
                let tagname = format!("{}-annotation", declaration.annotationtype.as_str());
                let mut dec_start = BytesStart::owned_name(tagname.as_bytes());
                if let Some(set) = &declaration.set {
                    dec_start.push_attribute(("set", set.as_str() ));
                }
                if let Some(alias) = &declaration.alias {
                    dec_start.push_attribute(("alias", alias.as_str() ));
                }
                if let Some(format) = &declaration.format {
                    dec_start.push_attribute(("format", format.as_str() ));
                }
                let dec_end = BytesEnd::owned(tagname.as_bytes().to_vec());
                if declaration.processors.is_empty() {
                    writer.write_event(Event::Empty(dec_start)).map_err(to_serialisation_error)?;
                } else {
                    writer.write_event(Event::Start(dec_start)).map_err(to_serialisation_error)?;
                    for proc_key in declaration.processors.iter() {
                        if let Some(processor) = self.get_processor(*proc_key) {
                            let mut ann_start = BytesStart::borrowed_name(b"annotator");
                            ann_start.push_attribute(("processor", processor.id.as_str() ));
                            writer.write_event(Event::Empty(ann_start)).map_err(to_serialisation_error)?;
                        } else {
                            return Err(FoliaError::InternalError(format!("Unable to resolve referenced processor during serialisation")));
                        }
                    }
                    writer.write_event(Event::End(dec_end)).map_err(to_serialisation_error)?;
                }
                writer.write_event(Event::Text(BytesText::from_plain(NL))).map_err(to_serialisation_error)?;
            }
        }
        writer.write_event(Event::End(BytesEnd::borrowed(b"annotations"))).map_err(to_serialisation_error)?;
        writer.write_event(Event::Text(BytesText::from_plain(NL))).map_err(to_serialisation_error)?;
        Ok(())
    }

    fn xml_provenance(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> Result<(), FoliaError> {
        writer.write_event(Event::Start( BytesStart::borrowed_name(b"provenance"))).map_err(to_serialisation_error)?;
        writer.write_event(Event::Text(BytesText::from_plain(NL))).map_err(to_serialisation_error)?;
        for processor_key in self.provenancestore.chain.iter() {
            self.xml_processor(writer, *processor_key)?;
        }
        writer.write_event(Event::End(BytesEnd::borrowed(b"provenance"))).map_err(to_serialisation_error)?;
        writer.write_event(Event::Text(BytesText::from_plain(NL))).map_err(to_serialisation_error)?;
        Ok(())
    }

    fn xml_processor(&self, writer: &mut Writer<Cursor<Vec<u8>>>, processor_key: ProcKey) -> Result<(),FoliaError> {
        if let Some(processor) = self.get_processor(processor_key) {
            let mut processor_start = BytesStart::borrowed_name(b"processor");
            processor_start.push_attribute(("xml:id", processor.id.as_str() ));
            processor_start.push_attribute(("name", processor.name.as_str() ));
            processor_start.push_attribute(("type", processor.processortype.as_str() ));
            if !processor.version.is_empty() {
                processor_start.push_attribute(("version", processor.version.as_str() ));
            }
            if !processor.folia_version.is_empty() {
                processor_start.push_attribute(("folia_version", processor.folia_version.as_str() ));
            }
            if !processor.document_version.is_empty() {
                processor_start.push_attribute(("document_version", processor.document_version.as_str() ));
            }
            if !processor.command.is_empty() {
                processor_start.push_attribute(("command", processor.command.as_str() ));
            }
            if !processor.host.is_empty() {
                processor_start.push_attribute(("host", processor.host.as_str() ));
            }
            if !processor.user.is_empty() {
                processor_start.push_attribute(("user", processor.user.as_str() ));
            }
            if !processor.begindatetime.is_empty() {
                processor_start.push_attribute(("begindatetime", processor.begindatetime.as_str() ));
            }
            if !processor.enddatetime.is_empty() {
                processor_start.push_attribute(("enddatetime", processor.enddatetime.as_str() ));
            }
            if !processor.src.is_empty() {
                processor_start.push_attribute(("src", processor.src.as_str() ));
            }
            if !processor.format.is_empty() {
                processor_start.push_attribute(("format", processor.format.as_str() ));
            }
            if !processor.resourcelink.is_empty() {
                processor_start.push_attribute(("resourcelink", processor.resourcelink.as_str() ));
            }
            if processor.processors.is_empty() {
                writer.write_event(Event::Empty(processor_start)).map_err(to_serialisation_error)?;
            } else {
                writer.write_event(Event::Start(processor_start)).map_err(to_serialisation_error)?;
                for subprocessor_key in processor.processors.iter() {
                    self.xml_processor(writer, *subprocessor_key)?;
                }
                writer.write_event(Event::End(BytesEnd::borrowed(b"processor"))).map_err(to_serialisation_error)?;
            }
            writer.write_event(Event::Text(BytesText::from_plain(NL))).map_err(to_serialisation_error)?;
        }
        Ok(())
    }

    pub(crate) fn xml_elements(&self, writer: &mut Writer<Cursor<Vec<u8>>>, root_key: ElementKey) -> Result<(), FoliaError> {
        //caches declarations that are defaults
        let dec_is_default: Vec<bool> = self.declarationstore.default_mask();

        //Select children
        let mut stack: Vec<(BytesEnd,ElementType,String)> = vec![];
        let mut previous_depth = 0;
        let mut last_start: String = "<ROOT>".to_string();
        for item in self.select_data_by_key(root_key,Selector::all_data(),Recursion::Always, true, false) {
            while item.depth < previous_depth {
                if let Some((end,elementtype,tagstring)) = stack.pop() {
                    writer.write_event(Event::End(end)).map_err(to_serialisation_error)?;
                    if !ElementGroup::TextMarkup.contains(elementtype) {
                        writer.write_event(Event::Text(BytesText::from_plain(NL))).map_err(to_serialisation_error)?;
                    }
                    //eprintln!("[DEBUG] <-- Popped end tag {} ({})", tagstring.as_str(), item.depth);
                } else {
                    return Err(FoliaError::SerialisationError(format!("Unable to pop the end tag stack depth {} < previous depth {}, last start element={}", item.depth, previous_depth, last_start)));
                }
                previous_depth -= 1;
            }
            match item.data {
                DataType::Element(key) => {
                    if let Some(element) = self.get_element(*key) {
                        let tagstring = element.elementtype().to_string();
                        //eprintln!("[DEBUG] (processing tag {} ({}))", tagstring.as_str(), item.depth);
                        let tag = tagstring.as_bytes();
                        let mut start = BytesStart::owned(tag.to_vec(), tag.len());
                        for attrib in element.attribs().iter() {
                            if *attrib != Attribute::Ignore && !attrib.decodable()  {
                                start.push_attribute((attrib.attribtype().into(), format!("{}",attrib).as_str() ));
                            }
                        }
                        if let Some(declaration_key) = element.declaration_key() {
                            //check if the declaration is the default, no need to serialise set then
                            if !dec_is_default.get(declaration_key as usize).expect("checking default")  && element.elementtype() != ElementType::Feature {
                                //decode encoded attributes
                                if let Some(set) = element.set() {
                                    start.push_attribute(("set", set) );
                                }
                            }
                            if let Some(subset) = element.subset() {
                                start.push_attribute(("subset", subset) );
                            }
                            if let Some(class) = element.class() {
                                start.push_attribute(("class", class) );
                            }
                            if let Some(processor) = element.processor() {
                                //check if this processor is the default one, if so we don't need
                                //to serialise it
                                let is_default: bool = if let Some(declaration) = self.get_declaration(declaration_key) {
                                    if declaration.processors.len() == 1 {
                                        declaration.processors.get(0) == element.processor_key().as_ref()
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                };

                                if !is_default {
                                    start.push_attribute(("processor", processor) );
                                }
                            }
                        }
                        if element.data.is_empty() {
                            writer.write_event(Event::Empty(start)).map_err(to_serialisation_error)?;
                        } else {
                            writer.write_event(Event::Start(start)).map_err(to_serialisation_error)?;
                            let end = BytesEnd::owned(tag.to_vec());
                            //eprintln!("[DEBUG] --> Pushed start tag {} ({})", tagstring.as_str(), item.depth);
                            last_start = tagstring.clone();
                            stack.push((end,element.elementtype(),tagstring));
                        }
                        if !ElementGroup::TextMarkup.contains(element.elementtype()) && element.elementtype() != ElementType::TextContent && element.elementtype() != ElementType::PhonContent {
                            writer.write_event(Event::Text(BytesText::from_plain(NL))).map_err(to_serialisation_error)?;
                        }
                    }
                },
                DataType::Text(text) => {
                    //eprintln!("[DEBUG] (processing text)");
                    let text = BytesText::from_plain_str(text.as_str());
                    writer.write_event(Event::Text(text)).map_err(to_serialisation_error)?;
                },
                DataType::Comment(comment) => {
                    //eprintln!("[DEBUG] (processing comment)");
                    let text = BytesText::from_plain_str(comment.as_str());
                    writer.write_event(Event::Comment(text)).map_err(to_serialisation_error)?;
                }
                DataType::SpanReference(_) => {
                    //does not serialise
                },
            }
            previous_depth = item.depth;
        }

        //don't forget the final closing elements
        while let Some((end, _elementtype, tagstring)) = stack.pop() {
            writer.write_event(Event::End(end)).map_err(to_serialisation_error)?;
            //eprintln!("[DEBUG] <-- Popped final end tag {}", tagstring.as_str());
        }

        Ok(())
    }
}
