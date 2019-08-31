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

    ///Parse a FoLiA document
    fn parse<R: BufRead>(reader: &mut Reader<R>) -> Result<Self, FoliaError> {
        let mut body: Option<FoliaElement> = None;
        let mut buf = Vec::new();
        let mut nsbuf = Vec::new();
        let mut id: String = String::new();

        //parse root
        loop {
            let e = reader.read_namespaced_event(&mut buf, &mut nsbuf)?;
            match e {
                (ref ns, Event::Start(ref e)) => {
                    match (*ns, e.local_name())  {
                        (Some(ns), b"FoLiA") if ns == NSFOLIA => {
                            for attrib in e.attributes() {
                                let attrib: quick_xml::events::attributes::Attribute = attrib.unwrap();
                                match attrib.key {
                                    b"xml:id" => {
                                        id = attrib.unescape_and_decode_value(&reader).expect("Unable to parse ID")
                                    }
                                    _ => {}
                                };
                            }
                            break;
                        },
                        (Some(ns), _) if ns == NSFOLIA => {
                            return Err(FoliaError::ParseError("Encountered unknown root tag".to_string()));
                        },
                        (_ns,_tag) => {
                            return Err(FoliaError::ParseError("Encountered unknown root tag not in FoLiA namespace".to_string()));
                        }
                    }
                },
                (_, Event::Eof) => {
                    return Err(FoliaError::ParseError("Premature end of document".to_string()));
                }
                (_,_) => {}
            }
        };

        //parse metadata
        //TODO

        //find body
        loop {
            let e = reader.read_namespaced_event(&mut buf, &mut nsbuf)?;
            match e {
                (ref ns, Event::Start(ref e)) => {
                    match (*ns, e.local_name())  {
                        (Some(ns), b"text") if ns == NSFOLIA => {
                            if let Ok(attribs)  =  FoliaElement::parse_attributes(&reader, e.attributes()) {
                                body = Some(FoliaElement::new(ElementType::Text).with_attribs(attribs));
                            }
                            break;
                        },
                        (Some(ns), b"speech") if ns == NSFOLIA => {
                            if let Ok(attribs)  =  FoliaElement::parse_attributes(&reader, e.attributes()) {
                                body = Some(FoliaElement::new(ElementType::Speech).with_attribs(attribs));
                            }
                            break;
                        },
                        (Some(ns), _) if ns == NSFOLIA => {
                            //just ignore everything else for now
                        },
                        (Some(ns),tag) => {
                            return Err(FoliaError::ParseError(format!("Expected FoLiA namespace, got namespace {} with tag {}", str::from_utf8(ns).expect("invalid utf-8 in namespace"), str::from_utf8(tag).expect("invalid utf-8 in tag")).to_string()));
                        }
                        (None,tag) => {
                            return Err(FoliaError::ParseError(format!("Expected FoLiA namespace, got no namespace with tag {}",  str::from_utf8(tag).expect("invalid utf-8 in tag")).to_string()));
                        }
                    }
                },
                (_, Event::Eof) => {
                    return Err(FoliaError::ParseError("Premature end of document".to_string()));
                }
                (_,_) => {}
            }
        };


        let mut doc = Self { id: id, filename: None, elementstore: ElementStore::default(), provenancestore: ProvenanceStore::default(), declarationstore: DeclarationStore::default(), metadata: Metadata::default() };
        if let Some(body) = body {
            let intid = doc.elementstore.add(body);
            doc.parse_elements(reader, &mut buf, &mut nsbuf)?;
            Ok(doc)
        } else {
            Err(FoliaError::ParseError("No body found".to_string()))
        }
    }

    ///Parses all elementsm from XML, this in turn invokes all parsers for the subelements
    fn parse_elements<R: BufRead>(&mut self, reader: &mut Reader<R>, mut buf: &mut Vec<u8>, mut nsbuf: &mut Vec<u8>) -> Result<(), FoliaError> {
        if !self.elementstore.is_empty() {
            let mut stack: Vec<IntId> = vec![0]; //0 is the root/body element, we always start with it
            loop {
                let e = reader.read_namespaced_event(&mut buf, &mut nsbuf)?;
                match e {
                    (Some(ns), Event::Empty(ref e)) if ns == NSFOLIA => {
                        //EMPTY TAG FOUND (<tag/>)
                        //eprintln!("EMPTY TAG: {}", str::from_utf8(e.local_name()).expect("Tag is not valid utf-8"));
                        let elem = FoliaElement::parse(reader, e)?;
                        let intid = self.elementstore.add(elem)?;
                        stack.push(intid);

                        // Since there is no Event::End after, directly append it to the current node
                        if let Some(parent_intid) = stack.last() {
                            self.elementstore.attach(*parent_intid, intid);
                        }
                    },
                    (Some(ns), Event::Start(ref e)) if ns == NSFOLIA => {
                        //START TAG FOUND (<tag>)
                        //eprintln!("START TAG: {}", str::from_utf8(e.local_name()).expect("Tag is not valid utf-8"));
                        let elem = FoliaElement::parse(reader, e)?;
                        let intid = self.elementstore.add(elem)?;
                        stack.push(intid);
                    },
                    (Some(ns), Event::End(ref e)) if ns == NSFOLIA => {
                        //END TAG FOUND (</tag>)
                        //eprintln!("END TAG: {}", str::from_utf8(e.local_name()).expect("Tag is not valid utf-8"));
                        if stack.len() <= 1 {
                            break;
                        }
                        let intid = stack.pop().unwrap();
                        if let Some(elem) = self.elementstore.get(intid) {

                            //verify we actually close the right thing (otherwise we have malformed XML)
                            let elementname = str::from_utf8(e.local_name()).expect("Tag is not valid utf-8");
                            let elementtype = ElementType::from_str(elementname)?;
                            if elem.elementtype != elementtype {
                                return Err(FoliaError::ParseError(format!("Malformed XML? Invalid element closed: {}, expected: {}", elementname, elem.elementtype.to_string() )));
                            }
                        } else {
                            eprintln!("ID from stack does not exist! {}", intid ) ;
                        }

                        //add element to parent (the previous one in the stack)
                        if let Some(parent_intid) = stack.last() {
                            self.elementstore.attach(*parent_intid, intid);
                        }
                    },
                    (None, Event::Text(s)) => {
                        let text = s.unescape_and_decode(reader)?;
                        if text.trim() != "" {
                            eprintln!("TEXT: {}", text);
                            if let Some(parent_intid) = stack.last() {
                                self.elementstore.get_mut(*parent_intid).map( |mut parent| {
                                    parent.push(DataType::Text(text));
                                });
                            }
                        }
                    },
                    (None, Event::CData(s)) => {
                        let text = reader.decode(&s)?;
                        if text.trim() != "" {
                            eprintln!("CDATA: {}", text);
                            if let Some(parent_intid) = stack.last() {
                                self.elementstore.get_mut(*parent_intid).map( |mut parent| {
                                    parent.push(DataType::Text(text.to_string()));
                                });
                            }
                        }
                    },
                    (None, Event::Comment(s)) => {
                        let comment = reader.decode(&s)?;
                        if comment.trim() != "" {
                            eprintln!("COMMENT: {}", comment);
                            if let Some(parent_intid) = stack.last() {
                                self.elementstore.get_mut(*parent_intid).map( |mut parent| {
                                    parent.push(DataType::Comment(comment.to_string()));
                                });
                            }
                        }
                    },
                    (_, Event::Eof) => {
                        break;
                    }
                    (_,_) => {}
                }
            };
            Ok(())
        } else {
            Err(FoliaError::ParseError("No root element".to_string()))
        }
    }

    pub fn id(&self) -> &str { &self.id }
    pub fn filename(&self) -> Option<&str> { self.filename.as_ref().map(String::as_str) } //String::as_str equals  |x| &**x

    ///Serialises a document to XML (vector of bytes, utf-8)
    pub fn xml(&self, root_intid: IntId) -> Result<Vec<u8>, FoliaError> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        //Start the root tag (and obtain data for its end)
        let root_end = if let Some(element) = self.elementstore.get(root_intid) {
            let tagstring = element.elementtype.to_string();
            let tag = tagstring.as_bytes();
            let start = BytesStart::owned(tag.to_vec(), tag.len());
            writer.write_event(Event::Start(start)).map_err(|err| FoliaError::SerialisationError(format!("{}",err)))?;
            BytesEnd::owned(tag.to_vec())
        } else {
            return Err(FoliaError::SerialisationError(format!("Specified root element not found: {}", root_intid)));
        };

        //Select children
        let mut stack = vec![];
        let mut previous_depth = 0;
        for item in self.elementstore.select(root_intid,Selector::new(TypeSelector::AnyType, SetSelector::AnySet),true) {
            while item.depth < previous_depth {
                if let Some(end) = stack.pop() {
                    writer.write_event(Event::End(end)).map_err(|err| FoliaError::SerialisationError(format!("{}",err)))?;
                } else {
                    return Err(FoliaError::SerialisationError("Unable to pop the end tag stack".to_string()));
                }
                previous_depth -= 1;
            }
            match item.data {
                DataType::Element(intid) => {
                    if let Some(element) = self.elementstore.get(*intid) {
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
