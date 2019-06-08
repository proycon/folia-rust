extern crate quick_xml;

use quick_xml::Reader;
use quick_xml::events::Event;
use std::error::Error;
use std::fmt;
use std::io;

// ------------------------------ ERROR DEFINITIONS & IMPLEMENTATIONS -------------------------------------------------------------
//
#[derive(Debug)]
pub enum FoliaError {
    IoError(io::Error),
    IndexError,
}

impl From<io::Error> for FoliaError {
    fn from(err: io::Error) -> FoliaError {
        FoliaError::IoError(err)
    }
}

impl Error for FoliaError {
    fn description(&self) -> &str {
        match *self {
            FoliaError::IoError(ref err) => err.description(),
            FoliaError::IndexError => "invalid index",
        }
    }

    fn cause(&self)  -> Option<&Error> {
        match *self {
            FoliaError::IoError(ref err) => Some(err as &Error),
            FoliaError::IndexError => None,
        }
    }
}

impl fmt::Display for FoliaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FoliaError::IoError(ref err) => fmt::Display::fmt(err, f),
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

}

struct Document {
    id: String,
    filename: Option<String>,
    data: Vec<FoliaElement>
}

impl Document {
    fn new(id: &str, bodytype: BodyType) -> Result<Self, FoliaError> {
        let mut data = Vec::new();
        data.push(match bodytype{
            BodyType::Text => FoliaElement::new(ElementType::Text, None, None).unwrap(),
            BodyType::Speech => FoliaElement::new(ElementType::Speech, None, None).unwrap(),
        });
        Ok(Self { id: id.to_string(), filename: None, data: data })
    }

    //fn load(filename: &String) -> Result<Self, Box<dyn Error>> {
    //}

}

fn main() {
    let mut doc = Document::new("example", BodyType::Text);
}
