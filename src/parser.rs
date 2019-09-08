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
        let mut parsedeclarations = false;
        let mut parseprovenance = false;
        let mut text: Option<String> = None;
        let mut meta_id: Option<String> = None;
        let mut declaration_key: Option<DecKey> = None;
        let mut processor_stack: Vec<ProcKey> = vec![];
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
                            parsedeclarations = true;
                        },
                        (Some(ns), b"provenance") if ns == NSFOLIA => {
                            parseprovenance = true;
                        },
                        (Some(ns), b"meta") if ns == NSFOLIA => {
                            for attrib in e.attributes() {
                                let attrib = attrib.expect("unwrapping meta attribute");
                                if let Ok(value) = attrib.unescape_and_decode_value(&reader) {
                                    match attrib.key {
                                        b"id" | b"xml:id" => {
                                            meta_id = Some(value.clone());
                                        },
                                        otherwise => {
                                            eprintln!("WARNING: Unhandled attribute meta/@{:?}",otherwise);
                                        }
                                    }
                                }
                            }
                            text = None;
                        },
                        (Some(ns), b"processor") if ns == NSFOLIA && parseprovenance => {
                            //TODO: parse processor (use processor_stack)
                        },
                        (Some(ns), b"annotator") if ns == NSFOLIA && parsedeclarations => {
                            //TODO: parse annotator (use declaration_key)
                        },
                        (Some(ns), tag) if ns == NSFOLIA && parsedeclarations => {
                            let declaration_type = get_declaration_type(str::from_utf8(tag).unwrap())?;
                            let mut set: Option<String> = None;
                            for attrib in e.attributes() {
                                let attrib = attrib.expect("unwrapping declaration attribute");
                                if let Ok(value) = attrib.unescape_and_decode_value(&reader) {
                                    match attrib.key {
                                        b"set" => {
                                            set = Some(value.clone());
                                        },
                                        b"alias" => {
                                            //TODO: parse alias
                                        },
                                        b"annotator" => {
                                            //TODO: handle old-style default
                                        },
                                        b"annotatortype" => {
                                            //TODO: handle old-style default
                                        },
                                        otherwise => {
                                            eprintln!("WARNING: Unhandled attribute on declaration: @{:?}",otherwise);
                                        }
                                    }
                                }
                            }
                            let declaration = Declaration::new(declaration_type, set);
                            let result = declarationstore.add(declaration)?;
                            declaration_key = Some(result);
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
                (None, Event::Text(s)) => {
                    let text_s = s.unescape_and_decode(reader)?;
                    text = Some(text_s);
                },
                (ref ns, Event::End(ref e)) => {
                    match (*ns, e.local_name())  {
                        (Some(ns), b"metadata") if ns == NSFOLIA => {
                            break;
                        },
                        (Some(ns), b"annotations") if ns == NSFOLIA => {
                            parsedeclarations = false;
                        },
                        (Some(ns), b"provenance") if ns == NSFOLIA => {
                            parseprovenance = false;
                        },
                        (Some(ns), b"processor") if ns == NSFOLIA && parseprovenance => {
                            processor_stack.pop();
                        },
                        (Some(ns), b"meta") if ns == NSFOLIA => {
                            if let (Some(text), Some(meta_id)) = (&text, &meta_id) {
                                metadata.data.insert(meta_id.clone(), text.clone());
                            }
                        },
                        (Some(ns), b"annotator") if ns == NSFOLIA && parsedeclarations => {
                            //nothing to do
                        }
                        (Some(ns), _tag) if ns == NSFOLIA && parsedeclarations => {
                            declaration_key = None;
                        }
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

fn get_declaration_type(tag: &str) -> Result<AnnotationType, FoliaError> {
    if let Some(index) = tag.find("-") {
        let (declaration_type_string, suffix) = tag.split_at(index);
        if suffix != "annotation" {
            Err(FoliaError::ParseError(format!("Expected declaration element, got: {}", tag )))
        } else {
            if let Some(annotationtype) = AnnotationType::from_str(declaration_type_string) {
                Ok(annotationtype)
            } else {
                Err(FoliaError::ParseError(format!("Unknown declaration: {}", tag )))
            }
        }
    } else {
        Err(FoliaError::ParseError(format!("Expected declaration element, got: {}", tag )))
    }
}

