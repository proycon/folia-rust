use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::borrow::Cow;
use std::str::FromStr;

use quick_xml::Reader;
use quick_xml::events::Event;

use crate::error::*;
use crate::attrib::*;

//foliaspec:elementtype
#[derive(Debug,Copy,Clone,PartialEq)]
pub enum ElementType { ActorFeature, Alternative, AlternativeLayers, BegindatetimeFeature, Caption, Cell, Chunk, ChunkingLayer, Comment, Content, CoreferenceChain, CoreferenceLayer, CoreferenceLink, Correction, Current, Definition, DependenciesLayer, Dependency, DependencyDependent, Description, Division, DomainAnnotation, EnddatetimeFeature, EntitiesLayer, Entity, Entry, ErrorDetection, Event, Example, External, Feature, Figure, ForeignData, FunctionFeature, Gap, Head, HeadFeature, Headspan, Hiddenword, Hyphbreak, Label, LangAnnotation, LemmaAnnotation, LevelFeature, Linebreak, LinkReference, List, ListItem, Metric, ModalityFeature, Morpheme, MorphologyLayer, New, Note, Observation, ObservationLayer, Original, Paragraph, Part, PhonContent, Phoneme, PhonologyLayer, PolarityFeature, PosAnnotation, Predicate, Quote, Reference, Relation, Row, SemanticRole, SemanticRolesLayer, SenseAnnotation, Sentence, Sentiment, SentimentLayer, Source, SpanRelation, SpanRelationLayer, Speech, Statement, StatementLayer, StatementRelation, StrengthFeature, String, StyleFeature, SubjectivityAnnotation, Suggestion, SynsetFeature, SyntacticUnit, SyntaxLayer, Table, TableHead, Target, Term, Text, TextContent, TextMarkupCorrection, TextMarkupError, TextMarkupGap, TextMarkupReference, TextMarkupString, TextMarkupStyle, TimeFeature, TimeSegment, TimingLayer, Utterance, ValueFeature, Whitespace, Word, WordReference }

//foliaspec:annotationtype
//Defines all annotation types (as part of the AnnotationType enumeration)
#[derive(Debug,Copy,Clone)]
pub enum AnnotationType { TEXT, TOKEN, DIVISION, PARAGRAPH, HEAD, LIST, FIGURE, WHITESPACE, LINEBREAK, SENTENCE, POS, LEMMA, DOMAIN, SENSE, SYNTAX, CHUNKING, ENTITY, CORRECTION, ERRORDETECTION, PHON, SUBJECTIVITY, MORPHOLOGICAL, EVENT, DEPENDENCY, TIMESEGMENT, GAP, QUOTE, NOTE, REFERENCE, RELATION, SPANRELATION, COREFERENCE, SEMROLE, METRIC, LANG, STRING, TABLE, STYLE, PART, UTTERANCE, ENTRY, TERM, DEFINITION, EXAMPLE, PHONOLOGICAL, PREDICATE, OBSERVATION, SENTIMENT, STATEMENT, ALTERNATIVE, RAWCONTENT, COMMENT, DESCRIPTION, HYPHENATION, HIDDENTOKEN }

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

#[derive(Debug,Clone)]
pub struct Selector<'a> {
    pub elementtype: TypeSelector,
    pub set: SetSelector<'a>,
    pub recursive: bool,
}

#[derive(Debug,Copy,Clone)]
pub enum SetSelector<'a> {
    SomeSet(&'a str),
    AnySet,
    NoSet
}

#[derive(Debug,Clone)]
pub enum TypeSelector {
    SomeType(ElementType),
    MultiType(Vec<ElementType>),
    AnyType,
}

pub struct SelectIterator<'a> {
    selector: Selector<'a>,
    element: &'a FoliaElement,
}

impl<'a> Iterator for SelectIterator<'a> {
    type Item = DataType;

    fn next(&mut self) -> Option<Self::Item> {
        //TODO
    }
}

pub trait Select {
    fn select(&self, selector: Selector) -> SelectIterator;
}

impl FoliaElement {

    ///Get Attribute
    pub fn attrib(&self, atype: AttribType) -> Option<&Attribute> {
        for attribute in self.attribs.iter() {
            if attribute.attribtype() == atype {
                return Some(attribute);
            }
        }
        None
    }

    ///Get attribute value as a string reference, only works for attributes that are strings
    pub fn attrib_str(&self, atype: AttribType) -> Option<&str> {
        if let Some(attrib) = self.attrib(atype) {
            if let Cow::Borrowed(s) = attrib.unwrap() {
                Some(s)
            }  else {
                None
            }
        } else {
            None
        }
    }

    ///Check if the attribute exists
    pub fn has_attrib(&self, atype: AttribType) -> bool {
        self.attribs.iter().find(|&a| a.attribtype() == atype).is_some()
    }

    ///Deletes (and returns) the specified attribute
    pub fn del_attrib(&mut self, atype: AttribType) -> Option<Attribute> {
        let position = self.attribs.iter().position(|a| a.attribtype() == atype);
        if let Some(position) = position {
            Some(self.attribs.remove(position))
        } else {
            None
        }
    }

    ///Sets/adds and attribute
    pub fn set_attrib(&mut self, attrib: Attribute) {
        //ensure we don't insert duplicates
        self.del_attrib(attrib.attribtype());
        //add the attribute
        self.attribs.push(attrib);
    }

    //attribute getters (shortcuts)
    pub fn id(&self) -> Option<&str> { self.attrib_str(AttribType::ID)  }
    pub fn class(&self) -> Option<&str> { self.attrib_str(AttribType::CLASS)  }
    pub fn set(&self) -> Option<&str> { self.attrib_str(AttribType::SET)  }
    pub fn processor(&self) -> Option<&str> { self.attrib_str(AttribType::PROCESSOR)  }

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

    fn parse_attributes<R: BufRead>(reader: &Reader<R>, attribiter: quick_xml::events::attributes::Attributes) -> Result<Vec<Attribute>, FoliaError> {
        let mut attributes: Vec<Attribute> = Vec::new();
        for attrib in attribiter {
            match Attribute::parse(&reader, &attrib.unwrap()) {
                Ok(attrib) => { attributes.push(attrib); },
                Err(e) => { return Err(e); }
            }
        }
        Ok(attributes)
    }

    fn parse<R: BufRead>(reader: &Reader<R>, event: &quick_xml::events::BytesStart) -> Result<FoliaElement, FoliaError> {
        let attributes: Vec<Attribute> = FoliaElement::parse_attributes(reader, event.attributes())?;
        let elementtype = getelementtype(str::from_utf8(event.local_name()).unwrap())?;
        Ok(FoliaElement { elementtype: elementtype, attribs: attributes, data: Vec::new() })
    }
}

impl Select for FoliaElement {
    fn select(&self, selector: Selector) -> SelectIterator {
        //TODO: implement
    }
}

fn getelementtype(tag: &str) -> Result<ElementType, FoliaError> {
    //foliaspec:string_elementtype_map
    match tag {
      "actor" =>  Ok(ElementType::ActorFeature),
      "alt" =>  Ok(ElementType::Alternative),
      "altlayers" =>  Ok(ElementType::AlternativeLayers),
      "begindatetime" =>  Ok(ElementType::BegindatetimeFeature),
      "caption" =>  Ok(ElementType::Caption),
      "cell" =>  Ok(ElementType::Cell),
      "chunk" =>  Ok(ElementType::Chunk),
      "chunking" =>  Ok(ElementType::ChunkingLayer),
      "comment" =>  Ok(ElementType::Comment),
      "content" =>  Ok(ElementType::Content),
      "coreferencechain" =>  Ok(ElementType::CoreferenceChain),
      "coreferences" =>  Ok(ElementType::CoreferenceLayer),
      "coreferencelink" =>  Ok(ElementType::CoreferenceLink),
      "correction" =>  Ok(ElementType::Correction),
      "current" =>  Ok(ElementType::Current),
      "def" =>  Ok(ElementType::Definition),
      "dependencies" =>  Ok(ElementType::DependenciesLayer),
      "dependency" =>  Ok(ElementType::Dependency),
      "dep" =>  Ok(ElementType::DependencyDependent),
      "desc" =>  Ok(ElementType::Description),
      "div" =>  Ok(ElementType::Division),
      "domain" =>  Ok(ElementType::DomainAnnotation),
      "enddatetime" =>  Ok(ElementType::EnddatetimeFeature),
      "entities" =>  Ok(ElementType::EntitiesLayer),
      "entity" =>  Ok(ElementType::Entity),
      "entry" =>  Ok(ElementType::Entry),
      "errordetection" =>  Ok(ElementType::ErrorDetection),
      "event" =>  Ok(ElementType::Event),
      "ex" =>  Ok(ElementType::Example),
      "external" =>  Ok(ElementType::External),
      "feat" =>  Ok(ElementType::Feature),
      "figure" =>  Ok(ElementType::Figure),
      "foreign-data" =>  Ok(ElementType::ForeignData),
      "function" =>  Ok(ElementType::FunctionFeature),
      "gap" =>  Ok(ElementType::Gap),
      "head" =>  Ok(ElementType::Head),
      "headfeature" =>  Ok(ElementType::HeadFeature),
      "hd" =>  Ok(ElementType::Headspan),
      "hiddenw" =>  Ok(ElementType::Hiddenword),
      "t-hbr" =>  Ok(ElementType::Hyphbreak),
      "label" =>  Ok(ElementType::Label),
      "lang" =>  Ok(ElementType::LangAnnotation),
      "lemma" =>  Ok(ElementType::LemmaAnnotation),
      "level" =>  Ok(ElementType::LevelFeature),
      "br" =>  Ok(ElementType::Linebreak),
      "xref" =>  Ok(ElementType::LinkReference),
      "list" =>  Ok(ElementType::List),
      "item" =>  Ok(ElementType::ListItem),
      "metric" =>  Ok(ElementType::Metric),
      "modality" =>  Ok(ElementType::ModalityFeature),
      "morpheme" =>  Ok(ElementType::Morpheme),
      "morphology" =>  Ok(ElementType::MorphologyLayer),
      "new" =>  Ok(ElementType::New),
      "note" =>  Ok(ElementType::Note),
      "observation" =>  Ok(ElementType::Observation),
      "observations" =>  Ok(ElementType::ObservationLayer),
      "original" =>  Ok(ElementType::Original),
      "p" =>  Ok(ElementType::Paragraph),
      "part" =>  Ok(ElementType::Part),
      "ph" =>  Ok(ElementType::PhonContent),
      "phoneme" =>  Ok(ElementType::Phoneme),
      "phonology" =>  Ok(ElementType::PhonologyLayer),
      "polarity" =>  Ok(ElementType::PolarityFeature),
      "pos" =>  Ok(ElementType::PosAnnotation),
      "predicate" =>  Ok(ElementType::Predicate),
      "quote" =>  Ok(ElementType::Quote),
      "ref" =>  Ok(ElementType::Reference),
      "relation" =>  Ok(ElementType::Relation),
      "row" =>  Ok(ElementType::Row),
      "semrole" =>  Ok(ElementType::SemanticRole),
      "semroles" =>  Ok(ElementType::SemanticRolesLayer),
      "sense" =>  Ok(ElementType::SenseAnnotation),
      "s" =>  Ok(ElementType::Sentence),
      "sentiment" =>  Ok(ElementType::Sentiment),
      "sentiments" =>  Ok(ElementType::SentimentLayer),
      "source" =>  Ok(ElementType::Source),
      "spanrelation" =>  Ok(ElementType::SpanRelation),
      "spanrelations" =>  Ok(ElementType::SpanRelationLayer),
      "speech" =>  Ok(ElementType::Speech),
      "statement" =>  Ok(ElementType::Statement),
      "statements" =>  Ok(ElementType::StatementLayer),
      "rel" =>  Ok(ElementType::StatementRelation),
      "strength" =>  Ok(ElementType::StrengthFeature),
      "str" =>  Ok(ElementType::String),
      "style" =>  Ok(ElementType::StyleFeature),
      "subjectivity" =>  Ok(ElementType::SubjectivityAnnotation),
      "suggestion" =>  Ok(ElementType::Suggestion),
      "synset" =>  Ok(ElementType::SynsetFeature),
      "su" =>  Ok(ElementType::SyntacticUnit),
      "syntax" =>  Ok(ElementType::SyntaxLayer),
      "table" =>  Ok(ElementType::Table),
      "tablehead" =>  Ok(ElementType::TableHead),
      "target" =>  Ok(ElementType::Target),
      "term" =>  Ok(ElementType::Term),
      "text" =>  Ok(ElementType::Text),
      "t" =>  Ok(ElementType::TextContent),
      "t-correction" =>  Ok(ElementType::TextMarkupCorrection),
      "t-error" =>  Ok(ElementType::TextMarkupError),
      "t-gap" =>  Ok(ElementType::TextMarkupGap),
      "t-ref" =>  Ok(ElementType::TextMarkupReference),
      "t-str" =>  Ok(ElementType::TextMarkupString),
      "t-style" =>  Ok(ElementType::TextMarkupStyle),
      "time" =>  Ok(ElementType::TimeFeature),
      "timesegment" =>  Ok(ElementType::TimeSegment),
      "timing" =>  Ok(ElementType::TimingLayer),
      "utt" =>  Ok(ElementType::Utterance),
      "value" =>  Ok(ElementType::ValueFeature),
      "whitespace" =>  Ok(ElementType::Whitespace),
      "w" =>  Ok(ElementType::Word),
      "wref" =>  Ok(ElementType::WordReference),
        _ => Err(FoliaError::ParseError(format!("Unknown tag has no associated element type: {}",tag).to_string()))
    }

}

fn getelementname(elementtype: ElementType) -> &'static str {
    //foliaspec:elementtype_string_map
    match elementtype {
      ElementType::ActorFeature => "actor",
      ElementType::Alternative => "alt",
      ElementType::AlternativeLayers => "altlayers",
      ElementType::BegindatetimeFeature => "begindatetime",
      ElementType::Caption => "caption",
      ElementType::Cell => "cell",
      ElementType::Chunk => "chunk",
      ElementType::ChunkingLayer => "chunking",
      ElementType::Comment => "comment",
      ElementType::Content => "content",
      ElementType::CoreferenceChain => "coreferencechain",
      ElementType::CoreferenceLayer => "coreferences",
      ElementType::CoreferenceLink => "coreferencelink",
      ElementType::Correction => "correction",
      ElementType::Current => "current",
      ElementType::Definition => "def",
      ElementType::DependenciesLayer => "dependencies",
      ElementType::Dependency => "dependency",
      ElementType::DependencyDependent => "dep",
      ElementType::Description => "desc",
      ElementType::Division => "div",
      ElementType::DomainAnnotation => "domain",
      ElementType::EnddatetimeFeature => "enddatetime",
      ElementType::EntitiesLayer => "entities",
      ElementType::Entity => "entity",
      ElementType::Entry => "entry",
      ElementType::ErrorDetection => "errordetection",
      ElementType::Event => "event",
      ElementType::Example => "ex",
      ElementType::External => "external",
      ElementType::Feature => "feat",
      ElementType::Figure => "figure",
      ElementType::ForeignData => "foreign-data",
      ElementType::FunctionFeature => "function",
      ElementType::Gap => "gap",
      ElementType::Head => "head",
      ElementType::HeadFeature => "headfeature",
      ElementType::Headspan => "hd",
      ElementType::Hiddenword => "hiddenw",
      ElementType::Hyphbreak => "t-hbr",
      ElementType::Label => "label",
      ElementType::LangAnnotation => "lang",
      ElementType::LemmaAnnotation => "lemma",
      ElementType::LevelFeature => "level",
      ElementType::Linebreak => "br",
      ElementType::LinkReference => "xref",
      ElementType::List => "list",
      ElementType::ListItem => "item",
      ElementType::Metric => "metric",
      ElementType::ModalityFeature => "modality",
      ElementType::Morpheme => "morpheme",
      ElementType::MorphologyLayer => "morphology",
      ElementType::New => "new",
      ElementType::Note => "note",
      ElementType::Observation => "observation",
      ElementType::ObservationLayer => "observations",
      ElementType::Original => "original",
      ElementType::Paragraph => "p",
      ElementType::Part => "part",
      ElementType::PhonContent => "ph",
      ElementType::Phoneme => "phoneme",
      ElementType::PhonologyLayer => "phonology",
      ElementType::PolarityFeature => "polarity",
      ElementType::PosAnnotation => "pos",
      ElementType::Predicate => "predicate",
      ElementType::Quote => "quote",
      ElementType::Reference => "ref",
      ElementType::Relation => "relation",
      ElementType::Row => "row",
      ElementType::SemanticRole => "semrole",
      ElementType::SemanticRolesLayer => "semroles",
      ElementType::SenseAnnotation => "sense",
      ElementType::Sentence => "s",
      ElementType::Sentiment => "sentiment",
      ElementType::SentimentLayer => "sentiments",
      ElementType::Source => "source",
      ElementType::SpanRelation => "spanrelation",
      ElementType::SpanRelationLayer => "spanrelations",
      ElementType::Speech => "speech",
      ElementType::Statement => "statement",
      ElementType::StatementLayer => "statements",
      ElementType::StatementRelation => "rel",
      ElementType::StrengthFeature => "strength",
      ElementType::String => "str",
      ElementType::StyleFeature => "style",
      ElementType::SubjectivityAnnotation => "subjectivity",
      ElementType::Suggestion => "suggestion",
      ElementType::SynsetFeature => "synset",
      ElementType::SyntacticUnit => "su",
      ElementType::SyntaxLayer => "syntax",
      ElementType::Table => "table",
      ElementType::TableHead => "tablehead",
      ElementType::Target => "target",
      ElementType::Term => "term",
      ElementType::Text => "text",
      ElementType::TextContent => "t",
      ElementType::TextMarkupCorrection => "t-correction",
      ElementType::TextMarkupError => "t-error",
      ElementType::TextMarkupGap => "t-gap",
      ElementType::TextMarkupReference => "t-ref",
      ElementType::TextMarkupString => "t-str",
      ElementType::TextMarkupStyle => "t-style",
      ElementType::TimeFeature => "time",
      ElementType::TimeSegment => "timesegment",
      ElementType::TimingLayer => "timing",
      ElementType::Utterance => "utt",
      ElementType::ValueFeature => "value",
      ElementType::Whitespace => "whitespace",
      ElementType::Word => "w",
      ElementType::WordReference => "wref",
    }

}

fn annotationtype2elementtype(annotationtype: AnnotationType) -> ElementType {
    //foliaspec:annotationtype_elementtype_map
    //A mapping from annotation types to element types, based on the assumption that there is always only one primary element for an annotation type (and possible multiple secondary ones which are not included in this map,w)
    match annotationtype {
        AnnotationType::ALTERNATIVE => ElementType::Alternative,
        AnnotationType::CHUNKING => ElementType::Chunk,
        AnnotationType::COMMENT => ElementType::Comment,
        AnnotationType::RAWCONTENT => ElementType::Content,
        AnnotationType::COREFERENCE => ElementType::CoreferenceChain,
        AnnotationType::CORRECTION => ElementType::Correction,
        AnnotationType::DEFINITION => ElementType::Definition,
        AnnotationType::DEPENDENCY => ElementType::Dependency,
        AnnotationType::DESCRIPTION => ElementType::Description,
        AnnotationType::DIVISION => ElementType::Division,
        AnnotationType::DOMAIN => ElementType::DomainAnnotation,
        AnnotationType::ENTITY => ElementType::Entity,
        AnnotationType::ENTRY => ElementType::Entry,
        AnnotationType::ERRORDETECTION => ElementType::ErrorDetection,
        AnnotationType::EVENT => ElementType::Event,
        AnnotationType::EXAMPLE => ElementType::Example,
        AnnotationType::FIGURE => ElementType::Figure,
        AnnotationType::GAP => ElementType::Gap,
        AnnotationType::HEAD => ElementType::Head,
        AnnotationType::HIDDENTOKEN => ElementType::Hiddenword,
        AnnotationType::HYPHENATION => ElementType::Hyphbreak,
        AnnotationType::LANG => ElementType::LangAnnotation,
        AnnotationType::LEMMA => ElementType::LemmaAnnotation,
        AnnotationType::LINEBREAK => ElementType::Linebreak,
        AnnotationType::LIST => ElementType::List,
        AnnotationType::METRIC => ElementType::Metric,
        AnnotationType::MORPHOLOGICAL => ElementType::Morpheme,
        AnnotationType::NOTE => ElementType::Note,
        AnnotationType::OBSERVATION => ElementType::Observation,
        AnnotationType::PARAGRAPH => ElementType::Paragraph,
        AnnotationType::PART => ElementType::Part,
        AnnotationType::PHON => ElementType::PhonContent,
        AnnotationType::PHONOLOGICAL => ElementType::Phoneme,
        AnnotationType::POS => ElementType::PosAnnotation,
        AnnotationType::PREDICATE => ElementType::Predicate,
        AnnotationType::QUOTE => ElementType::Quote,
        AnnotationType::REFERENCE => ElementType::Reference,
        AnnotationType::RELATION => ElementType::Relation,
        AnnotationType::SEMROLE => ElementType::SemanticRole,
        AnnotationType::SENSE => ElementType::SenseAnnotation,
        AnnotationType::SENTENCE => ElementType::Sentence,
        AnnotationType::SENTIMENT => ElementType::Sentiment,
        AnnotationType::SPANRELATION => ElementType::SpanRelation,
        AnnotationType::STATEMENT => ElementType::Statement,
        AnnotationType::STRING => ElementType::String,
        AnnotationType::SUBJECTIVITY => ElementType::SubjectivityAnnotation,
        AnnotationType::SYNTAX => ElementType::SyntacticUnit,
        AnnotationType::TABLE => ElementType::Table,
        AnnotationType::TERM => ElementType::Term,
        AnnotationType::TEXT => ElementType::TextContent,
        AnnotationType::STYLE => ElementType::TextMarkupStyle,
        AnnotationType::TIMESEGMENT => ElementType::TimeSegment,
        AnnotationType::UTTERANCE => ElementType::Utterance,
        AnnotationType::WHITESPACE => ElementType::Whitespace,
        AnnotationType::TOKEN => ElementType::Word,
    }

}

fn annotationtype2xml(annotationtype: AnnotationType) -> &'static str {
    //foliaspec:annotationtype_xml_map
    //A mapping from annotation types to xml tags (strings)
    match annotationtype {
      AnnotationType::ALTERNATIVE => "alt",
      AnnotationType::CHUNKING => "chunk",
      AnnotationType::COMMENT => "comment",
      AnnotationType::RAWCONTENT => "content",
      AnnotationType::COREFERENCE => "coreferencechain",
      AnnotationType::CORRECTION => "correction",
      AnnotationType::DEFINITION => "def",
      AnnotationType::DEPENDENCY => "dependency",
      AnnotationType::DESCRIPTION => "desc",
      AnnotationType::DIVISION => "div",
      AnnotationType::DOMAIN => "domain",
      AnnotationType::ENTITY => "entity",
      AnnotationType::ENTRY => "entry",
      AnnotationType::ERRORDETECTION => "errordetection",
      AnnotationType::EVENT => "event",
      AnnotationType::EXAMPLE => "ex",
      AnnotationType::FIGURE => "figure",
      AnnotationType::GAP => "gap",
      AnnotationType::HEAD => "head",
      AnnotationType::HIDDENTOKEN => "hiddenw",
      AnnotationType::HYPHENATION => "t-hbr",
      AnnotationType::LANG => "lang",
      AnnotationType::LEMMA => "lemma",
      AnnotationType::LINEBREAK => "br",
      AnnotationType::LIST => "list",
      AnnotationType::METRIC => "metric",
      AnnotationType::MORPHOLOGICAL => "morpheme",
      AnnotationType::NOTE => "note",
      AnnotationType::OBSERVATION => "observation",
      AnnotationType::PARAGRAPH => "p",
      AnnotationType::PART => "part",
      AnnotationType::PHON => "ph",
      AnnotationType::PHONOLOGICAL => "phoneme",
      AnnotationType::POS => "pos",
      AnnotationType::PREDICATE => "predicate",
      AnnotationType::QUOTE => "quote",
      AnnotationType::REFERENCE => "ref",
      AnnotationType::RELATION => "relation",
      AnnotationType::SEMROLE => "semrole",
      AnnotationType::SENSE => "sense",
      AnnotationType::SENTENCE => "s",
      AnnotationType::SENTIMENT => "sentiment",
      AnnotationType::SPANRELATION => "spanrelation",
      AnnotationType::STATEMENT => "statement",
      AnnotationType::STRING => "str",
      AnnotationType::SUBJECTIVITY => "subjectivity",
      AnnotationType::SYNTAX => "su",
      AnnotationType::TABLE => "table",
      AnnotationType::TERM => "term",
      AnnotationType::TEXT => "t",
      AnnotationType::STYLE => "t-style",
      AnnotationType::TIMESEGMENT => "timesegment",
      AnnotationType::UTTERANCE => "utt",
      AnnotationType::WHITESPACE => "whitespace",
      AnnotationType::TOKEN => "w",
    }

}
