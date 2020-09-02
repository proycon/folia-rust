use crate::common::*;
use crate::types::*;
use crate::element::*;
use crate::attrib::*;

use std::collections::HashMap;

use strum::IntoEnumIterator;

#[derive(Debug,Clone,PartialEq)]
pub enum AcceptedData {
    AcceptElementType(ElementType),
    AcceptElementGroup(ElementGroup),
}


pub struct Properties {
    pub xmltag: &'static str,
    pub annotationtype: Option<AnnotationType>,
    pub accepted_data: &'static [AcceptedData],
    pub required_data: &'static [AcceptedData],
    pub required_attribs: &'static [AttribType],
    pub optional_attribs: &'static [AttribType],
    pub occurrences: u32, //How often can this element occur under the parent? (0 = unlimited)
    pub occurrences_per_set: u32, //How often can a particular element+set combination occur under the parent (0 = unlimited)
    pub textdelimiter: Option<&'static str>, //Delimiter to use when dynamically gathering text
    pub printable: bool, //Is this element printable? (i.e. can the text() method be called?)
    pub speakable: bool, //Is this element phonetically representablly? (i.e. can the phon() method be called?)
    pub hidden: bool, //Is this element hidden? (only applies to Hiddenword for now)
    pub xlink: bool, //Can the element carry xlink references?
    pub textcontainer: bool, //Does the element directly take textual content (e.g. TextContent (t) is a textcontainer)
    pub phoncontainer: bool, //Does the element directly take phonetic content (e.g. PhonContent (ph) is a phoncontainer)
    pub subset: Option<&'static str>, //used for Feature subclasses
    pub auth: bool, //The default authoritative state for this element
    pub primaryelement: bool, //Is this the primary element for the advertised annotation type?
    pub auto_generate_id: bool, //Automatically generate an ID if none was provided?
    pub setonly: bool, //States that the element may take a set property only, and not a class property
    pub wrefable: bool, //Indicates whether this element is referable as a token/word (applies only to a very select few elements, such as w, morpheme, and phoneme)
    pub label: &'static str
}

impl Default for Properties {
    fn default()  -> Self {
        Self {
            xmltag: "",
            annotationtype: None,
            accepted_data: &[],
            required_data: &[],
            required_attribs: &[],
            optional_attribs: &[],
            occurrences: 0,
            occurrences_per_set: 0,
            textdelimiter: None,
            printable: false,
            speakable: false,
            hidden: false,
            xlink: false,
            textcontainer: false,
            phoncontainer: false,
            subset: None,
            auth: true,
            primaryelement: false,
            auto_generate_id: false,
            setonly: false,
            wrefable: false,
            label: "",
        }
    }
}

pub struct Specification {
    pub properties: HashMap<ElementType,Properties>
}

impl Default for Specification {
    fn default() -> Self {
        let mut spec = Self {
            properties: HashMap::new()
        };
        for elementtype in ElementType::iter() {
            spec.properties.insert( elementtype, Properties::new(elementtype) );
        }
        spec
    }
}

impl Specification {
    pub fn get(&self, elementtype: ElementType) -> &Properties {
        self.properties.get(&elementtype).expect("Unwrapping properties for element type")
    }
}

impl Properties {
    pub fn new(elementtype: ElementType) -> Self {
        //foliaspec:setelementproperties(elementtype)
        //Sets all element properties for all elements
        match elementtype {
            ElementType::ActorFeature => {
                let mut properties = Properties::default();
                properties.xmltag = "";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = Some("actor");
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Feature";
                properties
            },
            ElementType::Alternative => {
                let mut properties = Properties::default();
                properties.xmltag = "alt";
                properties.annotationtype = Some(AnnotationType::ALTERNATIVE);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::MorphologyLayer), AcceptedData::AcceptElementType(ElementType::PhonologyLayer)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = false;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Alternative";
                properties
            },
            ElementType::AlternativeLayers => {
                let mut properties = Properties::default();
                properties.xmltag = "altlayers";
                properties.annotationtype = Some(AnnotationType::ALTERNATIVE);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::ForeignData)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = false;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Alternative Layers";
                properties
            },
            ElementType::BegindatetimeFeature => {
                let mut properties = Properties::default();
                properties.xmltag = "";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = Some("begindatetime");
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Feature";
                properties
            },
            ElementType::Caption => {
                let mut properties = Properties::default();
                properties.xmltag = "caption";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Gap), AcceptedData::AcceptElementType(ElementType::Linebreak), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Paragraph), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Quote), AcceptedData::AcceptElementType(ElementType::Reference), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::Sentence), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent), AcceptedData::AcceptElementType(ElementType::Whitespace)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 1;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("\n\n");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Caption";
                properties
            },
            ElementType::Cell => {
                let mut properties = Properties::default();
                properties.xmltag = "cell";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Entry), AcceptedData::AcceptElementType(ElementType::Event), AcceptedData::AcceptElementType(ElementType::Example), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::Figure), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Gap), AcceptedData::AcceptElementType(ElementType::Head), AcceptedData::AcceptElementType(ElementType::Hiddenword), AcceptedData::AcceptElementType(ElementType::Linebreak), AcceptedData::AcceptElementType(ElementType::List), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Note), AcceptedData::AcceptElementType(ElementType::Paragraph), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::Quote), AcceptedData::AcceptElementType(ElementType::Reference), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::Sentence), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent), AcceptedData::AcceptElementType(ElementType::Whitespace), AcceptedData::AcceptElementType(ElementType::Word)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some(" | ");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Cell";
                properties
            },
            ElementType::Chunk => {
                let mut properties = Properties::default();
                properties.xmltag = "chunk";
                properties.annotationtype = Some(AnnotationType::CHUNKING);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::LinkReference), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::WordReference)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Chunk";
                properties
            },
            ElementType::ChunkingLayer => {
                let mut properties = Properties::default();
                properties.xmltag = "chunking";
                properties.annotationtype = Some(AnnotationType::CHUNKING);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Chunk), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::ForeignData)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ID ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = true;
                properties.wrefable = false;
                properties
            },
            ElementType::Comment => {
                let mut properties = Properties::default();
                properties.xmltag = "comment";
                properties.annotationtype = Some(AnnotationType::COMMENT);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ID,AttribType::METADATA,AttribType::N ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Comment";
                properties
            },
            ElementType::Content => {
                let mut properties = Properties::default();
                properties.xmltag = "content";
                properties.annotationtype = Some(AnnotationType::RAWCONTENT);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::METADATA ];
                properties.occurrences = 1;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Raw Content";
                properties
            },
            ElementType::CoreferenceChain => {
                let mut properties = Properties::default();
                properties.xmltag = "coreferencechain";
                properties.annotationtype = Some(AnnotationType::COREFERENCE);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::CoreferenceLink), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::LinkReference), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Relation)];
                properties.required_data = &[AcceptedData::AcceptElementType(ElementType::CoreferenceLink)];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Coreference Chain";
                properties
            },
            ElementType::CoreferenceLayer => {
                let mut properties = Properties::default();
                properties.xmltag = "coreferences";
                properties.annotationtype = Some(AnnotationType::COREFERENCE);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::CoreferenceChain), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::ForeignData)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ID ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = true;
                properties.wrefable = false;
                properties
            },
            ElementType::CoreferenceLink => {
                let mut properties = Properties::default();
                properties.xmltag = "coreferencelink";
                properties.annotationtype = Some(AnnotationType::COREFERENCE);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Headspan), AcceptedData::AcceptElementType(ElementType::LevelFeature), AcceptedData::AcceptElementType(ElementType::LinkReference), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::ModalityFeature), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::TimeFeature), AcceptedData::AcceptElementType(ElementType::WordReference)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Coreference Link";
                properties
            },
            ElementType::Correction => {
                let mut properties = Properties::default();
                properties.xmltag = "correction";
                properties.annotationtype = Some(AnnotationType::CORRECTION);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Current), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::ErrorDetection), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::New), AcceptedData::AcceptElementType(ElementType::Original), AcceptedData::AcceptElementType(ElementType::Suggestion)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Correction";
                properties
            },
            ElementType::Current => {
                let mut properties = Properties::default();
                properties.xmltag = "current";
                properties.annotationtype = Some(AnnotationType::CORRECTION);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementGroup(ElementGroup::Span), AcceptedData::AcceptElementGroup(ElementGroup::Structure), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[];
                properties.occurrences = 1;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties
            },
            ElementType::Definition => {
                let mut properties = Properties::default();
                properties.xmltag = "def";
                properties.annotationtype = Some(AnnotationType::DEFINITION);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::Figure), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Hiddenword), AcceptedData::AcceptElementType(ElementType::Linebreak), AcceptedData::AcceptElementType(ElementType::List), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Paragraph), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Reference), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::Sentence), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::Table), AcceptedData::AcceptElementType(ElementType::TextContent), AcceptedData::AcceptElementType(ElementType::Utterance), AcceptedData::AcceptElementType(ElementType::Whitespace), AcceptedData::AcceptElementType(ElementType::Word)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("\n\n");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Definition";
                properties
            },
            ElementType::DependenciesLayer => {
                let mut properties = Properties::default();
                properties.xmltag = "dependencies";
                properties.annotationtype = Some(AnnotationType::DEPENDENCY);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Dependency), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::ForeignData)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ID ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = true;
                properties.wrefable = false;
                properties
            },
            ElementType::Dependency => {
                let mut properties = Properties::default();
                properties.xmltag = "dependency";
                properties.annotationtype = Some(AnnotationType::DEPENDENCY);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::DependencyDependent), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Headspan), AcceptedData::AcceptElementType(ElementType::LinkReference), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Relation)];
                properties.required_data = &[AcceptedData::AcceptElementType(ElementType::DependencyDependent), AcceptedData::AcceptElementType(ElementType::Headspan)];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Dependency";
                properties
            },
            ElementType::DependencyDependent => {
                let mut properties = Properties::default();
                properties.xmltag = "dep";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::LinkReference), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::WordReference)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 1;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Dependent";
                properties
            },
            ElementType::Description => {
                let mut properties = Properties::default();
                properties.xmltag = "desc";
                properties.annotationtype = Some(AnnotationType::DESCRIPTION);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ID,AttribType::METADATA,AttribType::N ];
                properties.occurrences = 1;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Description";
                properties
            },
            ElementType::Division => {
                let mut properties = Properties::default();
                properties.xmltag = "div";
                properties.annotationtype = Some(AnnotationType::DIVISION);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Division), AcceptedData::AcceptElementType(ElementType::Entry), AcceptedData::AcceptElementType(ElementType::Event), AcceptedData::AcceptElementType(ElementType::Example), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::Figure), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Gap), AcceptedData::AcceptElementType(ElementType::Head), AcceptedData::AcceptElementType(ElementType::Linebreak), AcceptedData::AcceptElementType(ElementType::List), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Note), AcceptedData::AcceptElementType(ElementType::Paragraph), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Quote), AcceptedData::AcceptElementType(ElementType::Reference), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::Sentence), AcceptedData::AcceptElementType(ElementType::Table), AcceptedData::AcceptElementType(ElementType::TextContent), AcceptedData::AcceptElementType(ElementType::Utterance), AcceptedData::AcceptElementType(ElementType::Whitespace), AcceptedData::AcceptElementType(ElementType::Word)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("\n\n\n");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Division";
                properties
            },
            ElementType::DomainAnnotation => {
                let mut properties = Properties::default();
                properties.xmltag = "domain";
                properties.annotationtype = Some(AnnotationType::DOMAIN);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric)];
                properties.required_data = &[];
                properties.required_attribs = &[ AttribType::CLASS ];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Domain";
                properties
            },
            ElementType::EnddatetimeFeature => {
                let mut properties = Properties::default();
                properties.xmltag = "";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = Some("enddatetime");
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Feature";
                properties
            },
            ElementType::EntitiesLayer => {
                let mut properties = Properties::default();
                properties.xmltag = "entities";
                properties.annotationtype = Some(AnnotationType::ENTITY);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Entity), AcceptedData::AcceptElementType(ElementType::ForeignData)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ID ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = true;
                properties.wrefable = false;
                properties
            },
            ElementType::Entity => {
                let mut properties = Properties::default();
                properties.xmltag = "entity";
                properties.annotationtype = Some(AnnotationType::ENTITY);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::LinkReference), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::WordReference)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Entity";
                properties
            },
            ElementType::Entry => {
                let mut properties = Properties::default();
                properties.xmltag = "entry";
                properties.annotationtype = Some(AnnotationType::ENTRY);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Definition), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Example), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::Term), AcceptedData::AcceptElementType(ElementType::TextContent)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("\n\n");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Entry";
                properties
            },
            ElementType::ErrorDetection => {
                let mut properties = Properties::default();
                properties.xmltag = "errordetection";
                properties.annotationtype = Some(AnnotationType::ERRORDETECTION);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric)];
                properties.required_data = &[];
                properties.required_attribs = &[ AttribType::CLASS ];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Error Detection";
                properties
            },
            ElementType::Event => {
                let mut properties = Properties::default();
                properties.xmltag = "event";
                properties.annotationtype = Some(AnnotationType::EVENT);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::ActorFeature), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::BegindatetimeFeature), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Division), AcceptedData::AcceptElementType(ElementType::EnddatetimeFeature), AcceptedData::AcceptElementType(ElementType::Entry), AcceptedData::AcceptElementType(ElementType::Event), AcceptedData::AcceptElementType(ElementType::Example), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::Figure), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Gap), AcceptedData::AcceptElementType(ElementType::Head), AcceptedData::AcceptElementType(ElementType::Hiddenword), AcceptedData::AcceptElementType(ElementType::Linebreak), AcceptedData::AcceptElementType(ElementType::List), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Note), AcceptedData::AcceptElementType(ElementType::Paragraph), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Quote), AcceptedData::AcceptElementType(ElementType::Reference), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::Sentence), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::Table), AcceptedData::AcceptElementType(ElementType::TextContent), AcceptedData::AcceptElementType(ElementType::Utterance), AcceptedData::AcceptElementType(ElementType::Whitespace), AcceptedData::AcceptElementType(ElementType::Word)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("\n\n");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Event";
                properties
            },
            ElementType::Example => {
                let mut properties = Properties::default();
                properties.xmltag = "ex";
                properties.annotationtype = Some(AnnotationType::EXAMPLE);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::Figure), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Hiddenword), AcceptedData::AcceptElementType(ElementType::Linebreak), AcceptedData::AcceptElementType(ElementType::List), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Paragraph), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Reference), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::Sentence), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::Table), AcceptedData::AcceptElementType(ElementType::TextContent), AcceptedData::AcceptElementType(ElementType::Utterance), AcceptedData::AcceptElementType(ElementType::Whitespace), AcceptedData::AcceptElementType(ElementType::Word)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("\n\n");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Example";
                properties
            },
            ElementType::External => {
                let mut properties = Properties::default();
                properties.xmltag = "external";
                properties.annotationtype = None;
                properties.accepted_data = &[];
                properties.required_data = &[];
                properties.required_attribs = &[ AttribType::SRC ];
                properties.optional_attribs = &[];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "External";
                properties
            },
            ElementType::Feature => {
                let mut properties = Properties::default();
                properties.xmltag = "feat";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Feature";
                properties
            },
            ElementType::Figure => {
                let mut properties = Properties::default();
                properties.xmltag = "figure";
                properties.annotationtype = Some(AnnotationType::FIGURE);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Caption), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Linebreak), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("\n\n");
                properties.printable = true;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Figure";
                properties
            },
            ElementType::ForeignData => {
                let mut properties = Properties::default();
                properties.xmltag = "foreign-data";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties
            },
            ElementType::FunctionFeature => {
                let mut properties = Properties::default();
                properties.xmltag = "";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = Some("function");
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Feature";
                properties
            },
            ElementType::Gap => {
                let mut properties = Properties::default();
                properties.xmltag = "gap";
                properties.annotationtype = Some(AnnotationType::GAP);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Content), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Part)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Gap";
                properties
            },
            ElementType::Head => {
                let mut properties = Properties::default();
                properties.xmltag = "head";
                properties.annotationtype = Some(AnnotationType::HEAD);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Event), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Gap), AcceptedData::AcceptElementType(ElementType::Hiddenword), AcceptedData::AcceptElementType(ElementType::Linebreak), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Paragraph), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Reference), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::Sentence), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent), AcceptedData::AcceptElementType(ElementType::Whitespace), AcceptedData::AcceptElementType(ElementType::Word)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("\n\n");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Head";
                properties
            },
            ElementType::HeadFeature => {
                let mut properties = Properties::default();
                properties.xmltag = "";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = Some("head");
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Feature";
                properties
            },
            ElementType::Headspan => {
                let mut properties = Properties::default();
                properties.xmltag = "hd";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::LinkReference), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::WordReference)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 1;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Head";
                properties
            },
            ElementType::Hiddenword => {
                let mut properties = Properties::default();
                properties.xmltag = "hiddenw";
                properties.annotationtype = Some(AnnotationType::HIDDENTOKEN);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Reference), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some(" ");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = true;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = true;
                properties.label = "Hidden Word/Token";
                properties
            },
            ElementType::Hyphbreak => {
                let mut properties = Properties::default();
                properties.xmltag = "t-hbr";
                properties.annotationtype = Some(AnnotationType::HYPHENATION);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::TextMarkup), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Linebreak)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("");
                properties.printable = true;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = true;
                properties.textcontainer = true;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Hyphbreak";
                properties
            },
            ElementType::Label => {
                let mut properties = Properties::default();
                properties.xmltag = "label";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Hiddenword), AcceptedData::AcceptElementType(ElementType::Linebreak), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Reference), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent), AcceptedData::AcceptElementType(ElementType::Whitespace), AcceptedData::AcceptElementType(ElementType::Word)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("\n\n");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Label";
                properties
            },
            ElementType::LangAnnotation => {
                let mut properties = Properties::default();
                properties.xmltag = "lang";
                properties.annotationtype = Some(AnnotationType::LANG);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric)];
                properties.required_data = &[];
                properties.required_attribs = &[ AttribType::CLASS ];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 1;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Language";
                properties
            },
            ElementType::LemmaAnnotation => {
                let mut properties = Properties::default();
                properties.xmltag = "lemma";
                properties.annotationtype = Some(AnnotationType::LEMMA);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric)];
                properties.required_data = &[];
                properties.required_attribs = &[ AttribType::CLASS ];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 1;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Lemma";
                properties
            },
            ElementType::LevelFeature => {
                let mut properties = Properties::default();
                properties.xmltag = "";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = Some("level");
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Feature";
                properties
            },
            ElementType::Linebreak => {
                let mut properties = Properties::default();
                properties.xmltag = "br";
                properties.annotationtype = Some(AnnotationType::LINEBREAK);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::Relation)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = true;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Linebreak";
                properties
            },
            ElementType::LinkReference => {
                let mut properties = Properties::default();
                properties.xmltag = "xref";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::IDREF ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties
            },
            ElementType::List => {
                let mut properties = Properties::default();
                properties.xmltag = "list";
                properties.annotationtype = Some(AnnotationType::LIST);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Caption), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Event), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Linebreak), AcceptedData::AcceptElementType(ElementType::ListItem), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Note), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Reference), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("\n\n");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "List";
                properties
            },
            ElementType::ListItem => {
                let mut properties = Properties::default();
                properties.xmltag = "item";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Event), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Gap), AcceptedData::AcceptElementType(ElementType::Hiddenword), AcceptedData::AcceptElementType(ElementType::Label), AcceptedData::AcceptElementType(ElementType::Linebreak), AcceptedData::AcceptElementType(ElementType::List), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Note), AcceptedData::AcceptElementType(ElementType::Paragraph), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Quote), AcceptedData::AcceptElementType(ElementType::Reference), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::Sentence), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent), AcceptedData::AcceptElementType(ElementType::Whitespace), AcceptedData::AcceptElementType(ElementType::Word)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("\n");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "List Item";
                properties
            },
            ElementType::Metric => {
                let mut properties = Properties::default();
                properties.xmltag = "metric";
                properties.annotationtype = Some(AnnotationType::METRIC);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::ValueFeature)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Metric";
                properties
            },
            ElementType::ModalityFeature => {
                let mut properties = Properties::default();
                properties.xmltag = "";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = Some("modality");
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Feature";
                properties
            },
            ElementType::Morpheme => {
                let mut properties = Properties::default();
                properties.xmltag = "morpheme";
                properties.annotationtype = Some(AnnotationType::MORPHOLOGICAL);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::FunctionFeature), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Morpheme), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = true;
                properties.label = "Morpheme";
                properties
            },
            ElementType::MorphologyLayer => {
                let mut properties = Properties::default();
                properties.xmltag = "morphology";
                properties.annotationtype = Some(AnnotationType::MORPHOLOGICAL);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Morpheme)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ID ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = true;
                properties.wrefable = false;
                properties
            },
            ElementType::New => {
                let mut properties = Properties::default();
                properties.xmltag = "new";
                properties.annotationtype = Some(AnnotationType::CORRECTION);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementGroup(ElementGroup::Span), AcceptedData::AcceptElementGroup(ElementGroup::Structure), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[];
                properties.occurrences = 1;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties
            },
            ElementType::Note => {
                let mut properties = Properties::default();
                properties.xmltag = "note";
                properties.annotationtype = Some(AnnotationType::NOTE);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Example), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::Figure), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Head), AcceptedData::AcceptElementType(ElementType::Hiddenword), AcceptedData::AcceptElementType(ElementType::Linebreak), AcceptedData::AcceptElementType(ElementType::List), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Paragraph), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Reference), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::Sentence), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::Table), AcceptedData::AcceptElementType(ElementType::TextContent), AcceptedData::AcceptElementType(ElementType::Utterance), AcceptedData::AcceptElementType(ElementType::Whitespace), AcceptedData::AcceptElementType(ElementType::Word)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("\n\n");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Note";
                properties
            },
            ElementType::Observation => {
                let mut properties = Properties::default();
                properties.xmltag = "observation";
                properties.annotationtype = Some(AnnotationType::OBSERVATION);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::LinkReference), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::WordReference)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Observation";
                properties
            },
            ElementType::ObservationLayer => {
                let mut properties = Properties::default();
                properties.xmltag = "observations";
                properties.annotationtype = Some(AnnotationType::OBSERVATION);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Observation)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ID ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = true;
                properties.wrefable = false;
                properties
            },
            ElementType::Original => {
                let mut properties = Properties::default();
                properties.xmltag = "original";
                properties.annotationtype = Some(AnnotationType::CORRECTION);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementGroup(ElementGroup::Span), AcceptedData::AcceptElementGroup(ElementGroup::Structure), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[];
                properties.occurrences = 1;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = false;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties
            },
            ElementType::Paragraph => {
                let mut properties = Properties::default();
                properties.xmltag = "p";
                properties.annotationtype = Some(AnnotationType::PARAGRAPH);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Entry), AcceptedData::AcceptElementType(ElementType::Event), AcceptedData::AcceptElementType(ElementType::Example), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::Figure), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Gap), AcceptedData::AcceptElementType(ElementType::Head), AcceptedData::AcceptElementType(ElementType::Hiddenword), AcceptedData::AcceptElementType(ElementType::Linebreak), AcceptedData::AcceptElementType(ElementType::List), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Note), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Quote), AcceptedData::AcceptElementType(ElementType::Reference), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::Sentence), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent), AcceptedData::AcceptElementType(ElementType::Whitespace), AcceptedData::AcceptElementType(ElementType::Word)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("\n\n");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Paragraph";
                properties
            },
            ElementType::Part => {
                let mut properties = Properties::default();
                properties.xmltag = "part";
                properties.annotationtype = Some(AnnotationType::PART);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementGroup(ElementGroup::Structure), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::TextContent)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some(" ");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Part";
                properties
            },
            ElementType::PhonContent => {
                let mut properties = Properties::default();
                properties.xmltag = "ph";
                properties.annotationtype = Some(AnnotationType::PHON);
                properties.accepted_data = &[];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::METADATA ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = true;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Phonetic Content";
                properties
            },
            ElementType::Phoneme => {
                let mut properties = Properties::default();
                properties.xmltag = "phoneme";
                properties.annotationtype = Some(AnnotationType::PHONOLOGICAL);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::FunctionFeature), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Phoneme), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = true;
                properties.label = "Phoneme";
                properties
            },
            ElementType::PhonologyLayer => {
                let mut properties = Properties::default();
                properties.xmltag = "phonology";
                properties.annotationtype = Some(AnnotationType::PHONOLOGICAL);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Phoneme)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ID ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = true;
                properties.wrefable = false;
                properties
            },
            ElementType::PolarityFeature => {
                let mut properties = Properties::default();
                properties.xmltag = "";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = Some("polarity");
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Feature";
                properties
            },
            ElementType::PosAnnotation => {
                let mut properties = Properties::default();
                properties.xmltag = "pos";
                properties.annotationtype = Some(AnnotationType::POS);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::HeadFeature), AcceptedData::AcceptElementType(ElementType::Metric)];
                properties.required_data = &[];
                properties.required_attribs = &[ AttribType::CLASS ];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 1;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Part-of-Speech";
                properties
            },
            ElementType::Predicate => {
                let mut properties = Properties::default();
                properties.xmltag = "predicate";
                properties.annotationtype = Some(AnnotationType::PREDICATE);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::LinkReference), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::SemanticRole), AcceptedData::AcceptElementType(ElementType::WordReference)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Predicate";
                properties
            },
            ElementType::Quote => {
                let mut properties = Properties::default();
                properties.xmltag = "quote";
                properties.annotationtype = Some(AnnotationType::QUOTE);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Division), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Gap), AcceptedData::AcceptElementType(ElementType::Hiddenword), AcceptedData::AcceptElementType(ElementType::Linebreak), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Paragraph), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::Quote), AcceptedData::AcceptElementType(ElementType::Reference), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::Sentence), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent), AcceptedData::AcceptElementType(ElementType::Utterance), AcceptedData::AcceptElementType(ElementType::Whitespace), AcceptedData::AcceptElementType(ElementType::Word)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("\n\n");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Quote";
                properties
            },
            ElementType::Reference => {
                let mut properties = Properties::default();
                properties.xmltag = "ref";
                properties.annotationtype = Some(AnnotationType::REFERENCE);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Hiddenword), AcceptedData::AcceptElementType(ElementType::Linebreak), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Paragraph), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Quote), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::Sentence), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent), AcceptedData::AcceptElementType(ElementType::Utterance), AcceptedData::AcceptElementType(ElementType::Whitespace), AcceptedData::AcceptElementType(ElementType::Word)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some(" ");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = true;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Reference";
                properties
            },
            ElementType::Relation => {
                let mut properties = Properties::default();
                properties.xmltag = "relation";
                properties.annotationtype = Some(AnnotationType::RELATION);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::LinkReference), AcceptedData::AcceptElementType(ElementType::Metric)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = true;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Relation";
                properties
            },
            ElementType::Row => {
                let mut properties = Properties::default();
                properties.xmltag = "row";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Cell), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::Relation)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("\n");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Table Row";
                properties
            },
            ElementType::SemanticRole => {
                let mut properties = Properties::default();
                properties.xmltag = "semrole";
                properties.annotationtype = Some(AnnotationType::SEMROLE);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Headspan), AcceptedData::AcceptElementType(ElementType::LinkReference), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::WordReference)];
                properties.required_data = &[];
                properties.required_attribs = &[ AttribType::CLASS ];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Semantic Role";
                properties
            },
            ElementType::SemanticRolesLayer => {
                let mut properties = Properties::default();
                properties.xmltag = "semroles";
                properties.annotationtype = Some(AnnotationType::SEMROLE);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Predicate), AcceptedData::AcceptElementType(ElementType::SemanticRole)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ID ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = true;
                properties.wrefable = false;
                properties
            },
            ElementType::SenseAnnotation => {
                let mut properties = Properties::default();
                properties.xmltag = "sense";
                properties.annotationtype = Some(AnnotationType::SENSE);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::SynsetFeature)];
                properties.required_data = &[];
                properties.required_attribs = &[ AttribType::CLASS ];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Semantic Sense";
                properties
            },
            ElementType::Sentence => {
                let mut properties = Properties::default();
                properties.xmltag = "s";
                properties.annotationtype = Some(AnnotationType::SENTENCE);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Entry), AcceptedData::AcceptElementType(ElementType::Event), AcceptedData::AcceptElementType(ElementType::Example), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Gap), AcceptedData::AcceptElementType(ElementType::Hiddenword), AcceptedData::AcceptElementType(ElementType::Linebreak), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Note), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Quote), AcceptedData::AcceptElementType(ElementType::Reference), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent), AcceptedData::AcceptElementType(ElementType::Whitespace), AcceptedData::AcceptElementType(ElementType::Word)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some(" ");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Sentence";
                properties
            },
            ElementType::Sentiment => {
                let mut properties = Properties::default();
                properties.xmltag = "sentiment";
                properties.annotationtype = Some(AnnotationType::SENTIMENT);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Headspan), AcceptedData::AcceptElementType(ElementType::LinkReference), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::PolarityFeature), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::Source), AcceptedData::AcceptElementType(ElementType::StrengthFeature), AcceptedData::AcceptElementType(ElementType::Target), AcceptedData::AcceptElementType(ElementType::WordReference)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Sentiment";
                properties
            },
            ElementType::SentimentLayer => {
                let mut properties = Properties::default();
                properties.xmltag = "sentiments";
                properties.annotationtype = Some(AnnotationType::SENTIMENT);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Sentiment)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ID ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = true;
                properties.wrefable = false;
                properties
            },
            ElementType::Source => {
                let mut properties = Properties::default();
                properties.xmltag = "source";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::LinkReference), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::WordReference)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 1;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Source";
                properties
            },
            ElementType::SpanRelation => {
                let mut properties = Properties::default();
                properties.xmltag = "spanrelation";
                properties.annotationtype = Some(AnnotationType::SPANRELATION);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Relation)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Span Relation";
                properties
            },
            ElementType::SpanRelationLayer => {
                let mut properties = Properties::default();
                properties.xmltag = "spanrelations";
                properties.annotationtype = Some(AnnotationType::SPANRELATION);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::SpanRelation)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ID ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = true;
                properties.wrefable = false;
                properties
            },
            ElementType::Speech => {
                let mut properties = Properties::default();
                properties.xmltag = "speech";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Division), AcceptedData::AcceptElementType(ElementType::Entry), AcceptedData::AcceptElementType(ElementType::Event), AcceptedData::AcceptElementType(ElementType::Example), AcceptedData::AcceptElementType(ElementType::External), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Gap), AcceptedData::AcceptElementType(ElementType::Hiddenword), AcceptedData::AcceptElementType(ElementType::List), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Note), AcceptedData::AcceptElementType(ElementType::Paragraph), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Quote), AcceptedData::AcceptElementType(ElementType::Reference), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::Sentence), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent), AcceptedData::AcceptElementType(ElementType::Utterance), AcceptedData::AcceptElementType(ElementType::Word)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("\n\n\n");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Speech Body";
                properties
            },
            ElementType::Statement => {
                let mut properties = Properties::default();
                properties.xmltag = "statement";
                properties.annotationtype = Some(AnnotationType::STATEMENT);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Headspan), AcceptedData::AcceptElementType(ElementType::LinkReference), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::Source), AcceptedData::AcceptElementType(ElementType::StatementRelation), AcceptedData::AcceptElementType(ElementType::WordReference)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Statement";
                properties
            },
            ElementType::StatementLayer => {
                let mut properties = Properties::default();
                properties.xmltag = "statements";
                properties.annotationtype = Some(AnnotationType::STATEMENT);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Statement)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ID ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = true;
                properties.wrefable = false;
                properties
            },
            ElementType::StatementRelation => {
                let mut properties = Properties::default();
                properties.xmltag = "rel";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::LinkReference), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::WordReference)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 1;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Relation";
                properties
            },
            ElementType::StrengthFeature => {
                let mut properties = Properties::default();
                properties.xmltag = "";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = Some("strength");
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Feature";
                properties
            },
            ElementType::String => {
                let mut properties = Properties::default();
                properties.xmltag = "str";
                properties.annotationtype = Some(AnnotationType::STRING);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::TextContent)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "String";
                properties
            },
            ElementType::StyleFeature => {
                let mut properties = Properties::default();
                properties.xmltag = "";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = Some("style");
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Feature";
                properties
            },
            ElementType::SubjectivityAnnotation => {
                let mut properties = Properties::default();
                properties.xmltag = "subjectivity";
                properties.annotationtype = Some(AnnotationType::SUBJECTIVITY);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric)];
                properties.required_data = &[];
                properties.required_attribs = &[ AttribType::CLASS ];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 1;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Subjectivity/Sentiment";
                properties
            },
            ElementType::Suggestion => {
                let mut properties = Properties::default();
                properties.xmltag = "suggestion";
                properties.annotationtype = Some(AnnotationType::CORRECTION);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementGroup(ElementGroup::Span), AcceptedData::AcceptElementGroup(ElementGroup::Structure), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ID,AttribType::N ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = false;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties
            },
            ElementType::SynsetFeature => {
                let mut properties = Properties::default();
                properties.xmltag = "";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = Some("synset");
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Feature";
                properties
            },
            ElementType::SyntacticUnit => {
                let mut properties = Properties::default();
                properties.xmltag = "su";
                properties.annotationtype = Some(AnnotationType::SYNTAX);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::LinkReference), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::SyntacticUnit), AcceptedData::AcceptElementType(ElementType::WordReference)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Syntactic Unit";
                properties
            },
            ElementType::SyntaxLayer => {
                let mut properties = Properties::default();
                properties.xmltag = "syntax";
                properties.annotationtype = Some(AnnotationType::SYNTAX);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::SyntacticUnit)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ID ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = true;
                properties.wrefable = false;
                properties
            },
            ElementType::Table => {
                let mut properties = Properties::default();
                properties.xmltag = "table";
                properties.annotationtype = Some(AnnotationType::TABLE);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Linebreak), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::Row), AcceptedData::AcceptElementType(ElementType::TableHead)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("\n\n");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Table";
                properties
            },
            ElementType::TableHead => {
                let mut properties = Properties::default();
                properties.xmltag = "tablehead";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::Row)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("\n\n");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Table Header";
                properties
            },
            ElementType::Target => {
                let mut properties = Properties::default();
                properties.xmltag = "target";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::LinkReference), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::WordReference)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 1;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Target";
                properties
            },
            ElementType::Term => {
                let mut properties = Properties::default();
                properties.xmltag = "term";
                properties.annotationtype = Some(AnnotationType::TERM);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Event), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::Figure), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Gap), AcceptedData::AcceptElementType(ElementType::Hiddenword), AcceptedData::AcceptElementType(ElementType::Linebreak), AcceptedData::AcceptElementType(ElementType::List), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Paragraph), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Reference), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::Sentence), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::Table), AcceptedData::AcceptElementType(ElementType::TextContent), AcceptedData::AcceptElementType(ElementType::Utterance), AcceptedData::AcceptElementType(ElementType::Whitespace), AcceptedData::AcceptElementType(ElementType::Word)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("\n\n");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Term";
                properties
            },
            ElementType::Text => {
                let mut properties = Properties::default();
                properties.xmltag = "text";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Division), AcceptedData::AcceptElementType(ElementType::Entry), AcceptedData::AcceptElementType(ElementType::Event), AcceptedData::AcceptElementType(ElementType::Example), AcceptedData::AcceptElementType(ElementType::External), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::Figure), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Gap), AcceptedData::AcceptElementType(ElementType::Hiddenword), AcceptedData::AcceptElementType(ElementType::Linebreak), AcceptedData::AcceptElementType(ElementType::List), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Note), AcceptedData::AcceptElementType(ElementType::Paragraph), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Quote), AcceptedData::AcceptElementType(ElementType::Reference), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::Sentence), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::Table), AcceptedData::AcceptElementType(ElementType::TextContent), AcceptedData::AcceptElementType(ElementType::Whitespace), AcceptedData::AcceptElementType(ElementType::Word)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("\n\n\n");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Text Body";
                properties
            },
            ElementType::TextContent => {
                let mut properties = Properties::default();
                properties.xmltag = "t";
                properties.annotationtype = Some(AnnotationType::TEXT);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::TextMarkup), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Linebreak)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::METADATA ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = true;
                properties.textcontainer = true;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Text";
                properties
            },
            ElementType::TextMarkupCorrection => {
                let mut properties = Properties::default();
                properties.xmltag = "t-correction";
                properties.annotationtype = Some(AnnotationType::CORRECTION);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::TextMarkup), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Linebreak)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("");
                properties.printable = true;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = true;
                properties.textcontainer = true;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties
            },
            ElementType::TextMarkupError => {
                let mut properties = Properties::default();
                properties.xmltag = "t-error";
                properties.annotationtype = Some(AnnotationType::ERRORDETECTION);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::TextMarkup), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Linebreak)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("");
                properties.printable = true;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = true;
                properties.textcontainer = true;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties
            },
            ElementType::TextMarkupGap => {
                let mut properties = Properties::default();
                properties.xmltag = "t-gap";
                properties.annotationtype = Some(AnnotationType::GAP);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::TextMarkup), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Linebreak)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("");
                properties.printable = true;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = true;
                properties.textcontainer = true;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties
            },
            ElementType::TextMarkupReference => {
                let mut properties = Properties::default();
                properties.xmltag = "t-ref";
                properties.annotationtype = Some(AnnotationType::REFERENCE);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::TextMarkup), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Linebreak)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("");
                properties.printable = true;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = true;
                properties.textcontainer = true;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties
            },
            ElementType::TextMarkupString => {
                let mut properties = Properties::default();
                properties.xmltag = "t-str";
                properties.annotationtype = Some(AnnotationType::STRING);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::TextMarkup), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Linebreak)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("");
                properties.printable = true;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = true;
                properties.textcontainer = true;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties
            },
            ElementType::TextMarkupStyle => {
                let mut properties = Properties::default();
                properties.xmltag = "t-style";
                properties.annotationtype = Some(AnnotationType::STYLE);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::TextMarkup), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Linebreak)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("");
                properties.printable = true;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = true;
                properties.textcontainer = true;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties
            },
            ElementType::TimeFeature => {
                let mut properties = Properties::default();
                properties.xmltag = "";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = Some("time");
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Feature";
                properties
            },
            ElementType::TimeSegment => {
                let mut properties = Properties::default();
                properties.xmltag = "timesegment";
                properties.annotationtype = Some(AnnotationType::TIMESEGMENT);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::ActorFeature), AcceptedData::AcceptElementType(ElementType::BegindatetimeFeature), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::EnddatetimeFeature), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::LinkReference), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::WordReference)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Time Segment";
                properties
            },
            ElementType::TimingLayer => {
                let mut properties = Properties::default();
                properties.xmltag = "timing";
                properties.annotationtype = Some(AnnotationType::TIMESEGMENT);
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::TimeSegment)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ID ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = false;
                properties.auto_generate_id = false;
                properties.setonly = true;
                properties.wrefable = false;
                properties
            },
            ElementType::Utterance => {
                let mut properties = Properties::default();
                properties.xmltag = "utt";
                properties.annotationtype = Some(AnnotationType::UTTERANCE);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Gap), AcceptedData::AcceptElementType(ElementType::Hiddenword), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Note), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Quote), AcceptedData::AcceptElementType(ElementType::Reference), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::Sentence), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent), AcceptedData::AcceptElementType(ElementType::Word)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some(" ");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Utterance";
                properties
            },
            ElementType::ValueFeature => {
                let mut properties = Properties::default();
                properties.xmltag = "";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = Some("value");
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Feature";
                properties
            },
            ElementType::Whitespace => {
                let mut properties = Properties::default();
                properties.xmltag = "whitespace";
                properties.annotationtype = Some(AnnotationType::WHITESPACE);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::Relation)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some("");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = false;
                properties.label = "Whitespace";
                properties
            },
            ElementType::Word => {
                let mut properties = Properties::default();
                properties.xmltag = "w";
                properties.annotationtype = Some(AnnotationType::TOKEN);
                properties.accepted_data = &[AcceptedData::AcceptElementGroup(ElementGroup::Layer), AcceptedData::AcceptElementGroup(ElementGroup::Inline), AcceptedData::AcceptElementType(ElementType::Alternative), AcceptedData::AcceptElementType(ElementType::AlternativeLayers), AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Correction), AcceptedData::AcceptElementType(ElementType::Description), AcceptedData::AcceptElementType(ElementType::Feature), AcceptedData::AcceptElementType(ElementType::ForeignData), AcceptedData::AcceptElementType(ElementType::Metric), AcceptedData::AcceptElementType(ElementType::Part), AcceptedData::AcceptElementType(ElementType::PhonContent), AcceptedData::AcceptElementType(ElementType::Reference), AcceptedData::AcceptElementType(ElementType::Relation), AcceptedData::AcceptElementType(ElementType::String), AcceptedData::AcceptElementType(ElementType::TextContent)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::ANNOTATOR,AttribType::BEGINTIME,AttribType::CLASS,AttribType::CONFIDENCE,AttribType::DATETIME,AttribType::ENDTIME,AttribType::ID,AttribType::METADATA,AttribType::N,AttribType::SPACE,AttribType::SPEAKER,AttribType::SRC,AttribType::TEXTCLASS ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = Some(" ");
                properties.printable = true;
                properties.speakable = true;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = true;
                properties.setonly = false;
                properties.wrefable = true;
                properties.label = "Word/Token";
                properties
            },
            ElementType::WordReference => {
                let mut properties = Properties::default();
                properties.xmltag = "wref";
                properties.annotationtype = None;
                properties.accepted_data = &[AcceptedData::AcceptElementType(ElementType::Comment), AcceptedData::AcceptElementType(ElementType::Description)];
                properties.required_data = &[];
                properties.required_attribs = &[];
                properties.optional_attribs = &[ AttribType::IDREF ];
                properties.occurrences = 0;
                properties.occurrences_per_set = 0;
                properties.textdelimiter = None;
                properties.printable = false;
                properties.speakable = false;
                properties.hidden = false;
                properties.xlink = false;
                properties.textcontainer = false;
                properties.phoncontainer = false;
                properties.subset = None;
                properties.auth = true;
                properties.primaryelement = true;
                properties.auto_generate_id = false;
                properties.setonly = false;
                properties.wrefable = false;
                properties
            },
        }

    }
}
