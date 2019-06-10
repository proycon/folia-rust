pub mod folia {

extern crate quick_xml;

use quick_xml::Reader;
use quick_xml::events::Event;
use std::error::Error;
use std::path::{Path};
use std::fmt;
use std::io;
use std::str;
use std::io::BufReader;
use std::fs::File;


const NSFOLIA: &[u8] = b"http://ilk.uvt.nl/folia";

// ------------------------------ ERROR DEFINITIONS & IMPLEMENTATIONS -------------------------------------------------------------
//
#[derive(Debug)]
pub enum FoliaError {
    IoError(io::Error),
    XmlError(quick_xml::Error),
    ParseError(String),
    IndexError,
}

impl From<io::Error> for FoliaError {
    fn from(err: io::Error) -> FoliaError {
        FoliaError::IoError(err)
    }
}

impl From<quick_xml::Error> for FoliaError {
    fn from(err: quick_xml::Error) -> FoliaError {
        FoliaError::XmlError(err)
    }
}

impl Error for FoliaError {
    fn description(&self) -> &str {
        match *self {
            FoliaError::IoError(ref err) => err.description(),
            FoliaError::XmlError(ref err) => "XML Error",
            FoliaError::ParseError(ref err) => err,
            FoliaError::IndexError => "invalid index",
        }
    }

    fn cause(&self)  -> Option<&Error> {
        match *self {
            FoliaError::IoError(ref err) => Some(err as &Error),
            FoliaError::XmlError(ref err) => None,
            FoliaError::ParseError(ref err) => None, //TODO
            FoliaError::IndexError => None,
        }
    }
}

impl fmt::Display for FoliaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FoliaError::IoError(ref err) => fmt::Display::fmt(err, f),
            FoliaError::XmlError(ref err) => fmt::Display::fmt(err, f),
            FoliaError::ParseError(ref err) => fmt::Display::fmt(err, f),
            FoliaError::IndexError => fmt::Display::fmt("invalid index", f),
        }
    }
}

// -------------------------------------------------------------------------------------------

pub enum ElementType {
    ActorFeature,
    Alternative,
    AlternativeLayers,
    BegindatetimeFeature,
    Caption,
    Cell,
    Chunk,
    ChunkingLayer,
    Comment,
    Content,
    CoreferenceChain,
    CoreferenceLayer,
    CoreferenceLink,
    Correction,
    Current,
    Definition,
    DependenciesLayer,
    Dependency,
    DependencyDependent,
    Description,
    Division,
    DomainAnnotation,
    EnddatetimeFeature,
    EntitiesLayer,
    Entity,
    Entry,
    ErrorDetection,
    Event,
    Example,
    External,
    Feature,
    Figure,
    ForeignData,
    FunctionFeature,
    Gap,
    Head,
    HeadFeature,
    Headspan,
    Hiddenword,
    Hyphbreak,
    Label,
    LangAnnotation,
    LemmaAnnotation,
    LevelFeature,
    Linebreak,
    LinkReference,
    List,
    ListItem,
    Metric,
    ModalityFeature,
    Morpheme,
    MorphologyLayer,
    New,
    Note,
    Observation,
    ObservationLayer,
    Original,
    Paragraph,
    Part,
    PhonContent,
    Phoneme,
    PhonologyLayer,
    PolarityFeature,
    PosAnnotation,
    Predicate,
    Quote,
    Reference,
    Relation,
    Row,
    SemanticRole,
    SemanticRolesLayer,
    SenseAnnotation,
    Sentence,
    Sentiment,
    SentimentLayer,
    Source,
    SpanRelation,
    SpanRelationLayer,
    Speech,
    Statement,
    StatementLayer,
    StatementRelation,
    StrengthFeature,
    String,
    StyleFeature,
    SubjectivityAnnotation,
    Suggestion,
    SynsetFeature,
    SyntacticUnit,
    SyntaxLayer,
    Table,
    TableHead,
    Target,
    Term,
    Text,
    TextContent,
    TextMarkupCorrection,
    TextMarkupError,
    TextMarkupGap,
    TextMarkupString,
    TextMarkupStyle,
    TimeFeature,
    TimeSegment,
    TimingLayer,
    Utterance,
    ValueFeature,
    Whitespace,
    Word,
    WordReference,
}

#[derive(Debug,Copy,Clone)]
pub enum AttribType {
    ID, SET, CLASS, ANNOTATOR, ANNOTATORTYPE, CONFIDENCE, N, DATETIME, BEGINTIME, ENDTIME, SRC, SPEAKER, TEXTCLASS, METADATA, IDREF, SPACE, PROCESSOR
}

pub fn attribtypeclass(atype: AttribType) -> AttribType {
    match atype {
        AttribType::SET => AttribType::CLASS,
        AttribType::PROCESSOR => AttribType::ANNOTATOR,
        AttribType::ANNOTATORTYPE => AttribType::ANNOTATOR,
        _  => atype,
    }
}

pub enum Attribute {
    Id(String),
    Set(String),
    Class(String),
    Annotator(String),
    Confidence(f64),
    N(String),
    DateTime(String),
    BeginTime(String),
    EndTime(String),
    Src(String),
    Speaker(String),
    Textclass(String),
    Metadata(String),
    Idref(String),
    Space(String),

    Processor(String),
    Href(String),
}

pub enum AnnotationType {
    TEXT, TOKEN, DIVISION, PARAGRAPH, HEAD, LIST, FIGURE, WHITESPACE, LINEBREAK, SENTENCE, POS, LEMMA, DOMAIN, SENSE, SYNTAX, CHUNKING, ENTITY, CORRECTION, ERRORDETECTION, PHON, SUBJECTIVITY, MORPHOLOGICAL, EVENT, DEPENDENCY, TIMESEGMENT, GAP, QUOTE, NOTE, REFERENCE, RELATION, SPANRELATION, COREFERENCE, SEMROLE, METRIC, LANG, STRING, TABLE, STYLE, PART, UTTERANCE, ENTRY, TERM, DEFINITION, EXAMPLE, PHONOLOGICAL, PREDICATE, OBSERVATION, SENTIMENT, STATEMENT, ALTERNATIVE, RAWCONTENT, COMMENT, DESCRIPTION, HYPHENATION, HIDDENTOKEN
}

pub enum DataType {
    Text(String),
    Element(FoliaElement),
    Comment(String),
}

pub enum BodyType {
    Text,
    Speech
}

pub struct Properties {
    xmltag: String,
    annotationtype: AnnotationType,
    accepted_data: Vec<ElementType>,
    required_attribs: Vec<AttribType>,
    optional_attribs: Vec<AttribType>,
    occurrences: u32, //How often can this element occur under the parent? (0 = unlimited)
    occurrences_per_set: u32, //How often can a particular element+set combination occur under the parent (0 = unlimited)
    textdelimiter: Option<String>, //Delimiter to use when dynamically gathering text
    printable: bool, //Is this element printable? (i.e. can the text() method be called?)
    speakable: bool, //Is this element phonetically representablly? (i.e. can the phon() method be called?)
    hidden: bool, //Is this element hidden? (only applies to Hiddenword for now)
    xlink: bool, //Can the element carry xlink references?
    textcontainer: bool, //Does the element directly take textual content (e.g. TextContent (t) is a textcontainer)
    phoncontainer: bool, //Does the element directly take phonetic content (e.g. PhonContent (ph) is a phoncontainer)
    subset: Option<String>, //used for Feature subclasses
    auth: bool, //The default authoritative state for this element
    primaryelement: bool, //Is this the primary element for the advertised annotation type?
    auto_generate_id: bool, //Automatically generate an ID if none was provided?
    setonly: bool, //States that the element may take a set property only, and not a class property
    wrefable: bool //Indicates whether this element is referable as a token/word (applies only to a very select few elements, such as w, morpheme, and phoneme)
}

pub struct FoliaElement {
    elementtype: ElementType,
    data: Vec<DataType>,
    attribs: Vec<Attribute>,
}

impl FoliaElement {
    pub fn select(&self, elementtype: ElementType, set: Option<String>, recursive: bool, ignore: bool) {
    }

    pub fn attrib(&self, atype: AttribType) -> Option<&Attribute> {
        //Get attribute
        for attribute in self.attribs.iter() {
            let result = match (attribute, atype) {
                (Attribute::Id(_), AttribType::ID) => Some(attribute),
                (Attribute::Set(_), AttribType::SET) => Some(attribute),
                (Attribute::Class(_), AttribType::CLASS) => Some(attribute),
                (Attribute::Processor(_), AttribType::PROCESSOR) => Some(attribute),
                (_,_) => None,
            };
            if result.is_some() {
                return result;
            }
        }
        None
    }

    //attribute getters
    pub fn id(&self) -> Option<&String> {
        match self.attrib(AttribType::ID) {
            Some(Attribute::Id(value)) => Some(&value),
            _ => None
        }
    }

    pub fn class(&self) -> Option<&String> {
        match self.attrib(AttribType::CLASS) {
            Some(Attribute::Class(value)) => Some(&value),
            _ => None
        }
    }

    pub fn set(&self) -> Option<&String> {
        match self.attrib(AttribType::SET) {
            Some(Attribute::Set(value)) => Some(&value),
            _ => None
        }
    }

    pub fn processor(&self) -> Option<&String> {
        match self.attrib(AttribType::PROCESSOR) {
            Some(Attribute::Processor(value)) => Some(&value), _ => None
        }
    }

    pub fn append(&mut self, elementtype: ElementType, attribs: Option<Vec<Attribute>>, data: Option<Vec<DataType>>) -> Result<(), FoliaError> {
        let element  = FoliaElement::new(elementtype, attribs, data)?;
        self.data.push(DataType::Element(element));
        Ok(())
    }

    pub fn append_get_mut(&mut self, elementtype: ElementType, attribs: Option<Vec<Attribute>>, data: Option<Vec<DataType>>) -> Result<&mut DataType, FoliaError> {
        self.append(elementtype, attribs, data)?;
        self.get_mut_last().ok_or(FoliaError::IndexError)
    }

    pub fn get(&self, index: usize) -> Option<&DataType> {
        self.data.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut DataType> {
        self.data.get_mut(index)
    }

    pub fn get_mut_last(&mut self) -> Option<&mut DataType> {
        let index = self.data.len() - 1;
        self.data.get_mut(index)
    }

    pub fn get_last(&self) -> Option<&DataType> {
        let index = self.data.len() - 1;
        self.data.get(index)
    }

    pub fn new(elementtype: ElementType, attribs: Option<Vec<Attribute>>, data: Option<Vec<DataType>>) -> Result<FoliaElement, FoliaError> {
        Ok(Self { elementtype: elementtype, attribs: attribs.unwrap_or(Vec::new()), data: data.unwrap_or(Vec::new()) })
    }

    fn parseattributes(reader: &Reader<BufReader<File>>, attribiter: quick_xml::events::attributes::Attributes) -> Result<Vec<Attribute>, FoliaError> {
        let mut attributes: Vec<Attribute> = Vec::new();
        for attrib in attribiter {
            let attrib: quick_xml::events::attributes::Attribute = attrib.unwrap();
            match attrib.key {
                b"id" => {
                    attributes.push(Attribute::Id(attrib.unescape_and_decode_value(&reader).expect("Unable to parse ID")));
                },
                b"set" => {
                    attributes.push(Attribute::Set(attrib.unescape_and_decode_value(&reader).expect("Unable to parse set")));
                },
                b"class" => {
                    attributes.push(Attribute::Class(attrib.unescape_and_decode_value(&reader).expect("Unable to parse class")));
                },
                b"processor" => {
                    attributes.push(Attribute::Processor(attrib.unescape_and_decode_value(&reader).expect("Unable to parse processor")));
                },
                _ => {}
            }
        }
        Ok(attributes)
    }

    /*fn parse(reader: &Reader<BufReader<File>>) -> Result<FoliaElement, FoliaError> {
    }*/

}

pub struct Document {
    id: String,
    filename: Option<String>,
    body: FoliaElement,
}

struct ParseResult {
    id: String,
    body: FoliaElement,
}



impl Document {
    ///Create a new FoLiA document from scratch
    pub fn new(id: &str, bodytype: BodyType) -> Result<Self, FoliaError> {
        let body = match bodytype {
            BodyType::Text => FoliaElement::new(ElementType::Text, None, None).unwrap(),
            BodyType::Speech => FoliaElement::new(ElementType::Speech, None, None).unwrap(),
        };
        Ok(Self { id: id.to_string(), filename: None, body: body })
    }

    ///Load a FoliA document from file
    pub fn fromfile(filename: &str) -> Result<Self, FoliaError> {
        let mut reader = Reader::from_file(Path::new(filename))?;
        reader.trim_text(true);
        let mut result = Self::parse(&mut reader);
        if let Ok(ref mut doc) = result {
            //associate the filename with the document
            doc.filename = Some(filename.to_string());
        }
        return result;
    }

    ///Parse a FoLiA document
    fn parse(reader: &mut Reader<BufReader<File>>) -> Result<Self, FoliaError> {
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
                                    b"id" => {
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
                            if let Ok(attribs)  =  FoliaElement::parseattributes(&reader, e.attributes()) {
                                body = Some(FoliaElement { elementtype: ElementType::Text, data: Vec::new(), attribs: attribs });
                            }
                            break;
                        },
                        (Some(NSFOLIA), b"speech") => {
                            if let Ok(attribs)  =  FoliaElement::parseattributes(&reader, e.attributes()) {
                                body = Some(FoliaElement { elementtype: ElementType::Speech, data: Vec::new(), attribs: attribs });
                            }
                            break;
                        },
                        (Some(NSFOLIA), tag) => {
                            return Err(FoliaError::ParseError(format!("Unknown tag: {}",str::from_utf8(tag).expect("invalid utf-8 in tag")).to_string()));
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


        if let Some(body) = body {
            let mut doc = Self { id: id, body: body, filename: None };
            doc.parsebody(reader, &mut buf)?;
            Ok(doc)
        } else {
            Err(FoliaError::ParseError("No body found".to_string()))
        }
    }

    fn parsebody(&mut self, reader: &mut Reader<BufReader<File>>, buf: &mut Vec<u8>) -> Result<(), FoliaError> {
        let mut stack = vec![&mut self.body];
        let mut nsbuf = Vec::new(); //will creating a new one do or do we need to pass it explicitly?
        loop {
            let e = reader.read_namespaced_event(&mut buf, &mut nsbuf)?;
            match e {
                (ref Some(NSFOLIA), Event::Empty(ref e)) => {
                    let elem = FoliaElement::parse(reader, e)?;
                    // Since there is no Event::End after, directly append it to the current node
                    stack.last_mut().unwrap().data.push(DataType::Element(elem));
                },
                (ref Some(NSFOLIA), Event::Start(ref e)) => {
                    let elem = FoliaElement::parse(reader, e)?;
                    stack.push(elem);
                },
                (ref Some(NSFOLIA), Event::End(ref e)) => {
                    if stack.len() <= 1 {
                        break;
                    }
                    let elem = stack.pop().unwrap();
                    if let Some(to) = stack.last_mut() {
                        //verify we actually close the right thing (otherwise we have malformed XML)
                        if elem.elementtype != e.local_name() { //TODO:  different types!!
                            return Err(FoliaError::ParseError("Malformed XML? Invalid element closed"));
                        }
                        to.data.push(DataType::Element(elem));
                    }
                },
                (None, Event::Text(s)) => {
                    let text = s.unescape_and_decode(reader)?;
                    if text != "" {
                        let mut current_elem = stack.last_mut().unwrap();
                        current_elem.data.push(DataType::Text(text));
                    }
                },
                (None, Event::CData(s)) => {
                    let text = reader.decode(&s).into_owned();
                    if text != "" {
                        let mut current_elem = stack.last_mut().unwrap();
                        current_elem.data.push(DataType::Text(text));
                    }
                },
                (None, Event::Comment(s)) => {
                    let comment = reader.decode(&s).into_owned();
                    if comment != "" {
                        let mut current_elem = stack.last_mut().unwrap();
                        current_elem.data.push(DataType::Comment(comment));
                    }
                },
                (_, Event::Eof) => {
                    break;
                }
                (_,_) => {}
            }
        };
        Ok(())
    }

    pub fn id(&self) -> &str { &self.id }
    pub fn filename(&self) -> Option<&str> { self.filename.as_ref().map(String::as_str) } //String::as_str equals  |x| &**x


}


}//mod
