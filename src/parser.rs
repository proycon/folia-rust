use std::path::{Path};
use std::io::BufRead;
use std::io::BufReader;
use std::str;
use std::str::FromStr;
use std::borrow::ToOwned;
use std::string::ToString;
use std::fmt::Display;

use quick_xml::Reader;
use quick_xml::events::Event;

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
    ///Parse a FoLiA document
    pub fn parse<R: BufRead>(reader: &mut Reader<R>) -> Result<Self, FoliaError> {

        let mut body: Option<FoliaElement> = None;
        let mut buf = Vec::new();
        let mut nsbuf = Vec::new();
        let mut id: String = String::new();
        let mut metadata = Metadata::default();
        let mut declarationstore = DeclarationStore::default();
        let mut provenancestore = ProvenanceStore::default();

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
                                        id = attrib.unescape_and_decode_value(&reader).expect("Parsing ID")
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
        loop {
            let e = reader.read_namespaced_event(&mut buf, &mut nsbuf)?;
            match e {
                (ref ns, Event::Start(ref e)) => {
                    match (*ns, e.local_name())  {
                        (Some(ns), b"metadata") if ns == NSFOLIA => {
                            for attrib in e.attributes() {
                                let attrib = attrib.expect("unwrapping metadata attribute");
                                if let Ok(value) = attrib.unescape_and_decode_value(&reader) {
                                    match attrib.key {
                                        b"src" => {
                                            metadata.src = Some(value);
                                        },
                                        b"type" => {
                                            metadata.metadatatype = Some(value);
                                        },
                                        otherwise => {
                                            eprintln!("WARNING: Unhandled attribute metadata/@{:?}",otherwise);
                                        }
                                    }
                                }
                            }
                        },
                        (Some(ns), b"annotations") if ns == NSFOLIA => {
                            declarationstore.parse(reader)?;
                        },
                        (Some(ns), b"provenance") if ns == NSFOLIA => {
                            provenancestore.parse(reader)?;
                        },
                        (Some(ns), b"meta") if ns == NSFOLIA => {
                            let result = parse_meta(reader,e)?;
                            if let (key,value) = result {
                                metadata.data.insert(key, value);
                            }
                        },
                        (Some(ns), tag) if ns == NSFOLIA => {
                            return Err(FoliaError::ParseError(format!("Unexpected FoLiA element: {}",  str::from_utf8(tag).expect("decoding tag from utf-8")).to_string()));
                        },
                        (Some(ns),tag) if ns != NSFOLIA => {
                            return Err(FoliaError::ParseError(format!("Expected FoLiA namespace, got namespace {} with tag {}", str::from_utf8(ns).expect("decoding namespace from utf-8"), str::from_utf8(tag).expect("decoding XML tag from utf-8")).to_string()));
                        }
                        (None,tag) => {
                            return Err(FoliaError::ParseError(format!("Expected FoLiA namespace, got no namespace with tag {}",  str::from_utf8(tag).expect("decoding tag from utf-8")).to_string()));
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                },
                (ref ns, Event::End(ref e)) => {
                    match (*ns, e.local_name())  {
                        (Some(ns), b"metadata") if ns == NSFOLIA => {
                            break;
                        },
                        _ => { }
                    }
                },
                (_, Event::Eof) => {
                    return Err(FoliaError::ParseError("Premature end of document".to_string()));
                }
                (_,_) => {}
            }
        };

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
                        /*(Some(ns), _) if ns == NSFOLIA => {
                            //just ignore everything else for now
                        },*/
                        (Some(ns),tag) => {
                            return Err(FoliaError::ParseError(format!("Expected FoLiA namespace, got namespace {} with tag {}", str::from_utf8(ns).expect("decoding namespace from utf-8"), str::from_utf8(tag).expect("decoding XML tag from utf-8")).to_string()));
                        }
                        (None,tag) => {
                            return Err(FoliaError::ParseError(format!("Expected FoLiA namespace, got no namespace with tag {}",  str::from_utf8(tag).expect("decoding tag from utf-8")).to_string()));
                        }
                    }
                },
                (_, Event::Eof) => {
                    return Err(FoliaError::ParseError("Premature end of document".to_string()));
                }
                (_,_) => {}
            }
        };


        let mut doc = Self { id: id, filename: None, elementstore: ElementStore::default(), provenancestore: provenancestore, declarationstore: declarationstore, metadata: metadata };
        if let Some(body) = body {
            let key = doc.add(body);
            doc.parse_elements(reader, &mut buf, &mut nsbuf)?;
            Ok(doc)
        } else {
            Err(FoliaError::ParseError("No body found".to_string()))
        }
    }

    ///Parses all elementsm from XML, this in turn invokes all parsers for the subelements
    pub fn parse_elements<R: BufRead>(&mut self, reader: &mut Reader<R>, mut buf: &mut Vec<u8>, mut nsbuf: &mut Vec<u8>) -> Result<(), FoliaError> {
        if !self.elementstore.is_empty() {
            let mut stack: Vec<ElementKey> = vec![0]; //0 is the root/body element, we always start with it
            loop {
                let e = reader.read_namespaced_event(&mut buf, &mut nsbuf)?;
                match e {
                    (Some(ns), Event::Empty(ref e)) if ns == NSFOLIA => {
                        //EMPTY TAG FOUND (<tag/>)
                        //eprintln!("EMPTY TAG: {}", str::from_utf8(e.local_name()).expect("Tag is not valid utf-8"));
                        let elem = FoliaElement::parse(reader, e)?;
                        let key = self.add(elem)?;
                        stack.push(key);

                        // Since there is no Event::End after, directly append it to the current node
                        if let Some(parent_key) = stack.last() {
                            self.elementstore.attach(*parent_key, key)?;
                        }
                    },
                    (Some(ns), Event::Start(ref e)) if ns == NSFOLIA => {
                        //START TAG FOUND (<tag>)
                        //eprintln!("START TAG: {}", str::from_utf8(e.local_name()).expect("Tag is not valid utf-8"));
                        let elem = FoliaElement::parse(reader, e)?;
                        let key = self.add(elem)?;
                        stack.push(key);
                    },
                    (Some(ns), Event::End(ref e)) if ns == NSFOLIA => {
                        //END TAG FOUND (</tag>)
                        //eprintln!("END TAG: {}", str::from_utf8(e.local_name()).expect("Tag is not valid utf-8"));
                        if stack.len() <= 1 {
                            break;
                        }
                        let key = stack.pop().unwrap();
                        if let Some(elem) = self.elementstore.get(key) {

                            //verify we actually close the right thing (otherwise we have malformed XML)
                            let elementname = str::from_utf8(e.local_name()).expect("Decoding XML tag from utf-8");
                            let elementtype = ElementType::from_str(elementname)?;
                            if elem.elementtype != elementtype {
                                return Err(FoliaError::ParseError(format!("Malformed XML? Invalid element closed: {}, expected: {}", elementname, elem.elementtype.to_string() )));
                            }
                        } else {
                            return Err(FoliaError::InternalError(format!("ID from stack does not exist! {}", key )));
                        }

                        //add element to parent (the previous one in the stack)
                        if let Some(parent_key) = stack.last() {
                            self.elementstore.attach(*parent_key, key)?;
                        }
                    },
                    (None, Event::Text(s)) => {
                        let text = s.unescape_and_decode(reader)?;
                        if text.trim() != "" {
                            //eprintln!("TEXT: {}", text);
                            if let Some(parent_key) = stack.last() {
                                self.elementstore.get_mut(*parent_key).map( |mut parent| {
                                    parent.push(DataType::Text(text));
                                });
                            }
                        }
                    },
                    (None, Event::CData(s)) => {
                        let text = reader.decode(&s)?;
                        if text.trim() != "" {
                            if let Some(parent_key) = stack.last() {
                                self.elementstore.get_mut(*parent_key).map( |mut parent| {
                                    parent.push(DataType::Text(text.to_string()));
                                });
                            }
                        }
                    },
                    (None, Event::Comment(s)) => {
                        let comment = reader.decode(&s)?;
                        if comment.trim() != "" {
                            if let Some(parent_key) = stack.last() {
                                self.elementstore.get_mut(*parent_key).map( |mut parent| {
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
            Err(FoliaError::InternalError("No root element".to_string()))
        }
    }
}

///Parses a meta element
pub fn parse_meta<R: BufRead>(reader: &mut Reader<R>, event: &quick_xml::events::BytesStart) -> Result<(String,String), FoliaError> {
    let mut meta_id: String = String::new();

    for attrib in event.attributes() {
        let attrib = attrib.expect("unwrapping metadata attribute");
        if let Ok(value) = attrib.unescape_and_decode_value(&reader) {
            match attrib.key {
                b"id" | b"xml:id" => {
                    meta_id = value;
                },
                otherwise => {
                    eprintln!("WARNING: Unhandled attribute meta/@{:?}",otherwise);
                }
            }
        }
    }

    if meta_id.is_empty() {
        Err(FoliaError::ParseError("No ID found for <meta>".to_string()))
    } else {
        let text = parse_until_end(reader, b"meta")?;
        Ok((meta_id, text.unwrap_or("".to_string())))
    }
}


///Parses until the end of an element, returning any text if present
fn parse_until_end<R: BufRead>(reader: &mut Reader<R>, tag: &[u8]) -> Result<Option<String>,FoliaError> {
    let mut buf = Vec::new(); //not sure we can do this,
    let mut nsbuf = Vec::new();
    let mut text_option: Option<String> = None;

    loop {
        let e = reader.read_namespaced_event(&mut buf, &mut nsbuf)?;
        match e {
            (ref ns, Event::End(ref e)) => {
                match (*ns, e.local_name())  {
                    (Some(ns), foundtag) if ns == NSFOLIA && tag == foundtag  => {
                        break;
                    },
                    _ => { }
                }
            },
            (None, Event::Text(s)) => {
                let text = s.unescape_and_decode(reader)?;
                text_option = Some(text)
            },
            _ => {},
        }
    }
    Ok(text_option)
}

impl DeclarationStore {
    ///Parses the <annotations> block
    pub fn parse<R: BufRead>(&mut self, reader: &mut Reader<R>) -> Result<(), FoliaError> {
        Ok(())
    }

}

impl ProvenanceStore {
    ///Parses the <provenance> block
    pub fn parse<R: BufRead>(&mut self, reader: &mut Reader<R>) -> Result<(), FoliaError> {
        Ok(())
    }

}
