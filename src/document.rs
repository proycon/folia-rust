use std::path::{Path};
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::str;

use quick_xml::Reader;
use quick_xml::events::Event;

use crate::error::*;
use crate::element::*;
use crate::attrib::*;

pub struct Document {
    pub id: String,
    pub filename: Option<String>,
    pub body: Option<FoliaElement>,
}



impl Document {
    ///Create a new FoLiA document from scratch
    pub fn new(id: &str, bodytype: BodyType) -> Result<Self, FoliaError> {
        let body = match bodytype {
            BodyType::Text => Some(FoliaElement::new(ElementType::Text, None, None).unwrap()),
            BodyType::Speech => Some(FoliaElement::new(ElementType::Speech, None, None).unwrap()),
        };
        Ok(Self { id: id.to_string(), filename: None, body: body })
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
                        (Some(NSFOLIA), b"FoLiA") => {
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
                        (Some(NSFOLIA), _) => {
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
                        (Some(NSFOLIA), b"text") => {
                            if let Ok(attribs)  =  FoliaElement::parse_attributes(&reader, e.attributes()) {
                                body = Some(FoliaElement { elementtype: ElementType::Text, data: Vec::new(), attribs: attribs });
                            }
                            break;
                        },
                        (Some(NSFOLIA), b"speech") => {
                            if let Ok(attribs)  =  FoliaElement::parse_attributes(&reader, e.attributes()) {
                                body = Some(FoliaElement { elementtype: ElementType::Speech, data: Vec::new(), attribs: attribs });
                            }
                            break;
                        },
                        (Some(NSFOLIA), _) => {
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


        let mut doc = Self { id: id, body: body, filename: None };
        if doc.body.is_some() {
            doc.parse_body(reader, &mut buf, &mut nsbuf)?;
            Ok(doc)
        } else {
            Err(FoliaError::ParseError("No body found".to_string()))
        }
    }

    ///Parses the body of the FoLiA document, this in turn invokes all parsers for the subelements
    fn parse_body<R: BufRead>(&mut self, reader: &mut Reader<R>, mut buf: &mut Vec<u8>, mut nsbuf: &mut Vec<u8>) -> Result<(), FoliaError> {
        let mut body: FoliaElement  = self.body.take().unwrap(); //we take ownership, we will put it back in self.body after we're done
        let mut stack = vec![body];
        loop {
            let e = reader.read_namespaced_event(&mut buf, &mut nsbuf)?;
            match e {
                (Some(NSFOLIA), Event::Empty(ref e)) => {
                    let elem = FoliaElement::parse(reader, e)?;
                    // Since there is no Event::End after, directly append it to the current node
                    stack.last_mut().unwrap().data.push(DataType::Element(elem));
                },
                (Some(NSFOLIA), Event::Start(ref e)) => {
                    let elem = FoliaElement::parse(reader, e)?;
                    stack.push(elem);
                },
                (Some(NSFOLIA), Event::End(ref e)) => {
                    if stack.len() <= 1 {
                        break;
                    }
                    let elem = stack.pop().unwrap();
                    if let Some(to) = stack.last_mut() {
                        //verify we actually close the right thing (otherwise we have malformed XML)
                        if elem.elementtype != getelementtype(str::from_utf8(e.local_name()).expect("Tag is not valid utf-8")).expect("Unknown tag") {
                            return Err(FoliaError::ParseError("Malformed XML? Invalid element closed".to_string()));
                        }
                        to.data.push(DataType::Element(elem));
                    }
                },
                (None, Event::Text(s)) => {
                    let text = s.unescape_and_decode(reader)?;
                    if text != "" {
                        let current_elem = stack.last_mut().unwrap();
                        current_elem.data.push(DataType::Text(text));
                    }
                },
                (None, Event::CData(s)) => {
                    let text = reader.decode(&s).into_owned();
                    if text != "" {
                        let current_elem = stack.last_mut().unwrap();
                        current_elem.data.push(DataType::Text(text));
                    }
                },
                (None, Event::Comment(s)) => {
                    let comment = reader.decode(&s).into_owned();
                    if comment != "" {
                        let current_elem = stack.last_mut().unwrap();
                        current_elem.data.push(DataType::Comment(comment));
                    }
                },
                (_, Event::Eof) => {
                    break;
                }
                (_,_) => {}
            }
        };
        self.body = Some(stack.pop().unwrap());
        Ok(())
    }

    pub fn id(&self) -> &str { &self.id }
    pub fn filename(&self) -> Option<&str> { self.filename.as_ref().map(String::as_str) } //String::as_str equals  |x| &**x


}
