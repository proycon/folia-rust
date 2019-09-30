use std::path::{Path};
use std::io::BufRead;
use std::io::BufReader;
use std::str;
use std::str::FromStr;
use std::borrow::ToOwned;
use std::string::ToString;
use std::fmt::Display;
use std::collections::HashMap;

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
use crate::document::*;
use crate::specification::*;

impl Document {
    ///Parses a FoLiA document given a reader
    pub(crate) fn parse<R: BufRead>(reader: &mut Reader<R>, properties: DocumentProperties) -> Result<Self, FoliaError> {

        let mut body: Option<ElementData> = None;
        let mut buf = Vec::new();
        let mut nsbuf = Vec::new();

        let mut doc = Self {
                            id: "untitled".to_string(),
                            filename: None,
                            version: FOLIAVERSION.to_string(),
                            elementstore: ElementStore::default(),
                            provenancestore: ProvenanceStore::default(),
                            declarationstore: DeclarationStore::default(),
                            metadata: Metadata::default(),
                            submetadata: HashMap::default(),
                            autodeclare: properties.autodeclare,
        };

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
                                        doc.id = attrib.unescape_and_decode_value(&reader).expect("Parsing ID")
                                    }
                                    b"version" => {
                                        doc.version = attrib.unescape_and_decode_value(&reader).expect("Parsing version")
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
        let mut submetadata: Option<String> = None;
        let mut text: Option<String> = None;
        let mut meta_id: Option<String> = None;
        let mut declaration_key: Option<DecKey> = None;
        let mut annotators: Vec<(DecKey,String)> = Vec::new(); //mapping of declaration keys to processor ids; temporary structure
        let mut processor_stack: Vec<ProcKey> = vec![];
        loop {
            let e = reader.read_namespaced_event(&mut buf, &mut nsbuf)?;
            match e {
                (ref ns, Event::Empty(ref e)) => {
                    match (*ns, e.local_name())  {
                        (Some(ns), b"processor") if ns == NSFOLIA && parseprovenance => {
                            let processor = Processor::parse(&reader, &e)?;
                            if processor_stack.is_empty() {
                                doc.add_processor(processor)?;
                            } else {
                                let parent_key = processor_stack.last().expect("Polling processor stack");
                                doc.add_subprocessor(*parent_key, processor)?;
                            }
                        },
                        (Some(ns), b"annotator") if ns == NSFOLIA && parsedeclarations => {
                            let mut processor_id: Option<String> = None;
                            for attrib in e.attributes() {
                                let attrib = attrib.expect("unwrapping annotator attribute");
                                if let Ok(value) = attrib.unescape_and_decode_value(&reader) {
                                    match attrib.key {
                                        b"processor" => {
                                            processor_id = Some(value.clone());
                                        },
                                        otherwise => {
                                            eprintln!("WARNING: Unhandled attribute annotator/@{:?}",str::from_utf8(otherwise).unwrap());
                                        }
                                    }
                                }
                            }
                            if let Some(processor_id) = processor_id {
                                if let Some(declaration_key) = declaration_key {
                                    //add to a temporary structure because we don't know about the
                                    //processors yet at this stage, provenance still has to be
                                    //parsed, after that we resolve everything from this structure.
                                    annotators.push((declaration_key, processor_id));
                                }
                            }
                        },
                        _ => {
                        }
                    }
                },
                (ref ns, Event::Start(ref e)) => {
                    match (*ns, e.local_name())  {
                        (Some(ns), b"metadata") if ns == NSFOLIA => {
                            for attrib in e.attributes() {
                                let attrib = attrib.expect("unwrapping metadata attribute");
                                if let Ok(value) = attrib.unescape_and_decode_value(&reader) {
                                    match attrib.key {
                                        b"src" => {
                                            doc.metadata.src = Some(value);
                                        },
                                        b"type" => {
                                            doc.metadata.metadatatype = Some(value);
                                        },
                                        otherwise => {
                                            eprintln!("WARNING: Unhandled attribute metadata/@{:?}",str::from_utf8(otherwise).unwrap());
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
                        (Some(ns), b"submetadata") if ns == NSFOLIA => {
                            let mut processor_id: Option<String> = None;
                            let mut submetadata_type: Option<String> = None;
                            for attrib in e.attributes() {
                                let attrib = attrib.expect("unwrapping annotator attribute");
                                if let Ok(value) = attrib.unescape_and_decode_value(&reader) {
                                    match attrib.key {
                                        b"xml:id" => {
                                            submetadata = Some(value.clone());
                                        },
                                        b"type" => {
                                            submetadata_type = Some(value.clone());
                                        },
                                        otherwise => {
                                            eprintln!("WARNING: Unhandled attribute submetadata/@{:?}",str::from_utf8(otherwise).unwrap());
                                        }
                                    }
                                }
                            }
                            if submetadata.is_none() {
                                return Err(FoliaError::ParseError("Submetadata has no ID".to_string()));
                            }
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
                                            eprintln!("WARNING: Unhandled attribute meta/@{:?}",str::from_utf8(otherwise).unwrap());
                                        }
                                    }
                                }
                            }
                            text = None;
                        },
                        (Some(ns), b"processor") if ns == NSFOLIA && parseprovenance => {
                            let processor = Processor::parse(&reader, &e)?;
                            if processor_stack.is_empty() {
                                let processor_key = doc.add_processor(processor)?;
                                processor_stack.push(processor_key);
                            } else {
                                let parent_key = processor_stack.last().expect("Polling processor stack");
                                let processor_key = doc.add_subprocessor(*parent_key, processor)?;
                                processor_stack.push(processor_key);
                            }
                        },
                        (Some(ns), tag) if ns == NSFOLIA && parsedeclarations => {
                            let declaration = Declaration::parse(&reader, e, tag)?;
                            let result = doc.add_declaration(declaration)?;
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
                        (Some(ns), b"submetadata") if ns == NSFOLIA => {
                            submetadata = None;
                        },
                        (Some(ns), b"metadata") if ns == NSFOLIA => {
                            break;
                        },
                        (Some(ns), b"annotations") if ns == NSFOLIA => {
                            parsedeclarations = false;
                        },
                        (Some(ns), b"provenance") if ns == NSFOLIA => {
                            parseprovenance = false;

                            //we are done with provenance, we can now assign processors to
                            //declarations using our temporary structure
                            for (dec_key, processor_id) in annotators.iter() {
                                if let Some(processor_key) = doc.get_processor_key_by_id(processor_id) {
                                    if let Some(declaration) = doc.get_mut_declaration(*dec_key) {
                                        declaration.processors.push(processor_key);
                                    }
                                }
                            }
                        },
                        (Some(ns), b"processor") if ns == NSFOLIA && parseprovenance => {
                            processor_stack.pop();
                        },
                        (Some(ns), b"meta") if ns == NSFOLIA => {
                            if let (Some(text), Some(meta_id)) = (&text, &meta_id) {
                                if let Some(submetadata_id) = &submetadata {
                                    let submetadata_opt: Option<&mut Metadata> = doc.submetadata.get_mut(submetadata_id);
                                    if let Some(submetadata) = submetadata_opt {
                                        submetadata.data.insert(meta_id.clone(), text.clone());
                                    }
                                } else {
                                    doc.metadata.data.insert(meta_id.clone(), text.clone());
                                }
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
                            if let Ok((attribs,_))  =  ElementData::parse_attributes(&reader, e.attributes(), ElementType::Text) {
                                body = Some(ElementData::new(ElementType::Text).with_attribs(attribs));
                            }
                            break;
                        },
                        (Some(ns), b"speech") if ns == NSFOLIA => {
                            if let Ok((attribs,_))  =  ElementData::parse_attributes(&reader, e.attributes(), ElementType::Speech) {
                                body = Some(ElementData::new(ElementType::Speech).with_attribs(attribs));
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


        if let Some(body) = body {
            doc.add(body,None)?;
            doc.apply_properties(properties)?;
            doc.parse_elements(reader, &mut buf, &mut nsbuf)?;
            Ok(doc)
        } else {
            Err(FoliaError::ParseError("No body found".to_string()))
        }
    }

    ///Parses all elementsm from XML, this in turn invokes all parsers for the subelements
    pub(crate) fn parse_elements<R: BufRead>(&mut self, reader: &mut Reader<R>, mut buf: &mut Vec<u8>, mut nsbuf: &mut Vec<u8>) -> Result<(), FoliaError> {
        if !self.elementstore.items.is_empty() {
            let mut stack: Vec<ElementKey> = vec![0]; //0 is the root/body element, we always start with it
            loop {
                let e = reader.read_namespaced_event(&mut buf, &mut nsbuf)?;
                match e {
                    (Some(ns), Event::Empty(ref e)) if ns == NSFOLIA => {
                        //EMPTY TAG FOUND (<tag/>)
                        //eprintln!("EMPTY TAG: {}", str::from_utf8(e.local_name()).expect("Tag is not valid utf-8"));
                        let (elem, children) = ElementData::parse(reader, e)?;
                        let key = self.add(elem,stack.last().map(|key| *key))?;

                        // Since there is no Event::End after, directly append it to the current node
                        if let Some(parent_key) = stack.last() {
                            self.attach_element(*parent_key, key)?;
                        }
                        //add immediate children (limited to those derived from XML attributes)
                        for child in children {
                            let child_key = self.add(child, Some(key))?;
                            self.attach_element(key, child_key)?;
                        }
                        self.post_add(key, Some(&stack))?;
                    },
                    (Some(ns), Event::Start(ref e)) if ns == NSFOLIA => {
                        //START TAG FOUND (<tag>)
                        //eprintln!("START TAG: {}", str::from_utf8(e.local_name()).expect("Tag is not valid utf-8"));
                        let (elem, children) = ElementData::parse(reader, e)?;
                        let key = self.add(elem,stack.last().map(|key| *key))?;
                        stack.push(key);
                        //add immediate children (limited to those derived from XML attributes)
                        for child in children {
                            let child_key = self.add(child, Some(key))?;
                            self.attach_element(key, child_key)?;
                        }
                    },
                    (Some(ns), Event::End(ref e)) if ns == NSFOLIA => {
                        //END TAG FOUND (</tag>)
                        //eprintln!("END TAG: {}", str::from_utf8(e.local_name()).expect("Tag is not valid utf-8"));
                        if stack.len() <= 1 {
                            break;
                        }
                        let key = stack.pop().unwrap();
                        self.post_add(key, Some(&stack))?;
                        if let Some(elem) = self.get_elementdata(key) {

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
                            self.attach_element(*parent_key, key)?;
                        }
                    },
                    (None, Event::Text(s)) => {
                        let text = s.unescape_and_decode(reader)?;
                        if text.trim() != "" {
                            //eprintln!("TEXT: {}", text);
                            if let Some(parent_key) = stack.last() {
                                self.get_mut_elementdata(*parent_key).map( |parent| {
                                    parent.push(DataType::Text(text));
                                });
                            }
                        }
                    },
                    (None, Event::CData(s)) => {
                        let text = reader.decode(&s)?;
                        if text.trim() != "" {
                            if let Some(parent_key) = stack.last() {
                                self.get_mut_elementdata(*parent_key).map( |parent| {
                                    parent.push(DataType::Text(text.to_string()));
                                });
                            }
                        }
                    },
                    (None, Event::Comment(s)) => {
                        let comment = reader.decode(&s)?;
                        if comment.trim() != "" {
                            if let Some(parent_key) = stack.last() {
                                self.get_mut_elementdata(*parent_key).map( |parent| {
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
        if suffix != "-annotation" {
            Err(FoliaError::ParseError(format!("Expected declaration element, got: {}", tag)))
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

impl Processor {

    ///Parse this element from XML, note that this does not handle the child elements, those are
    ///appended by the main parser in Document::parse_body()
    pub(crate) fn parse<R: BufRead>(reader: &Reader<R>, event: &quick_xml::events::BytesStart) -> Result<Processor, FoliaError> {
        let mut processor = Processor::default();
        for attrib in event.attributes()  {
            if let Ok(attrib) = attrib {
                let value = attrib.unescape_and_decode_value(&reader).expect("Parsing attribute value for processor");
                match  attrib.key {
                    b"xml:id" => { processor.id = value; },
                    b"name" => { processor.name = value; },
                    b"version" => { processor.version = value; },
                    b"folia_version" => { processor.folia_version = value; },
                    b"document_version" => { processor.document_version = value; },
                    b"command" => { processor.command = value; },
                    b"host" => { processor.host = value; },
                    b"user" => { processor.user = value; },
                    b"begindatetime" => { processor.begindatetime = value; },
                    b"enddatetime" => { processor.enddatetime = value; },
                    b"src" => { processor.src = value; },
                    b"format" => { processor.format = value; },
                    b"resourcelink" => { processor.resourcelink = value; },
                    b"type" => { match value.as_str() {
                        "manual" => processor.processortype = ProcessorType::Manual,
                        "auto" => processor.processortype = ProcessorType::Auto,
                        "generator" => processor.processortype = ProcessorType::Generator,
                        "datasource" => processor.processortype = ProcessorType::DataSource,
                        _ => {
                            return Err(FoliaError::ParseError(format!("Invalid processor type: {:?}", value )));
                        }
                    }},
                    tag => {
                        return Err(FoliaError::ParseError(format!("Unknown attribute on processor, got: {:?}", str::from_utf8(tag).unwrap() )));
                    }
                }
            }
        }
        Ok(processor)
    }
}

impl ElementData {
    fn parse_attributes<R: BufRead>(reader: &Reader<R>, attribiter: quick_xml::events::attributes::Attributes, elementtype: ElementType) -> Result<(Vec<Attribute>,Vec<ElementData>), FoliaError> {
        let mut attributes: Vec<Attribute> = Vec::new();
        let mut children: Vec<ElementData> = Vec::new();
        'outerloop: for attrib in attribiter {
            let attrib = &attrib.expect("Parsing XML attribute");
            match Attribute::parse(&reader, attrib) {
                //normal behaviour
                Ok(attrib) => { attributes.push(attrib); },
                Err(e) => {
                    //an attribute can be a shortcut for a subset and translate to a
                    //ElementType::Feature
                    if let Ok(value) = attrib.unescape_and_decode_value(&reader) {
                        let properties = Properties::new(elementtype);
                        let featuregroup = ElementGroup::Feature;
                        for accepteddata in properties.accepted_data.iter() {
                            if let AcceptedData::AcceptElementType(acceptedtype) = accepteddata {
                                if featuregroup.contains(*acceptedtype) {
                                    let properties = Properties::new(*acceptedtype);
                                    if properties.subset.is_some() && str::from_utf8(attrib.key).unwrap() == properties.subset.unwrap() {
                                        let child = ElementData::new(ElementType::Feature).
                                                                with_attribs(vec![
                                                                    Attribute::Subset(properties.subset.unwrap().to_string()),
                                                                    Attribute::Class(value.clone())
                                                                ]).
                                                                with_children(vec![DataType::Text(value.clone())]);
                                        children.push(child);
                                        continue 'outerloop;
                                    }
                                }
                            }
                        }
                    }
                    return Err(e);
                }
            }
        }
        Ok((attributes, children))
    }

    ///Parse this element from XML, note that this does not handle the child elements, those are
    ///appended by the main parser in Document::parse_body()
    pub(crate) fn parse<R: BufRead>(reader: &Reader<R>, event: &quick_xml::events::BytesStart) -> Result<(ElementData,Vec<ElementData>), FoliaError> {
        let elementtype = ElementType::from_str(str::from_utf8(event.local_name()).expect("utf-8 decoding"))?;
        let (attributes, children) = ElementData::parse_attributes(reader, event.attributes(), elementtype)?;
        Ok((ElementData::new(elementtype).with_attribs(attributes), children))
    }
}

impl Declaration {
    pub(crate) fn parse<R: BufRead>(reader: &Reader<R>, event: &quick_xml::events::BytesStart, tag: &[u8]) -> Result<Declaration, FoliaError> {
        let declaration_type = get_declaration_type(str::from_utf8(tag).expect("utf-8 decoding"))?;
        let mut set: Option<String> = None;
        let mut alias: Option<String> = None;
        let mut format: Option<String> = None;
        for attrib in event.attributes() {
            let attrib = attrib.expect("unwrapping declaration attribute");
            if let Ok(value) = attrib.unescape_and_decode_value(&reader) {
                match attrib.key {
                    b"set" => {
                        set = Some(value.clone());
                    },
                    b"alias" => {
                        alias = Some(value.clone());
                    },
                    b"format" => {
                        format = Some(value.clone());
                    },
                    b"annotator" => {
                        //TODO: handle old-style default
                    },
                    b"annotatortype" => {
                        //TODO: handle old-style default
                    },
                    otherwise => {
                        eprintln!("WARNING: Unhandled attribute on declaration: @{:?}",str::from_utf8(otherwise).unwrap());
                    }
                }
            }
        }
        Ok(Declaration::new(declaration_type, set, alias, format))
    }
}
