use std::convert::Into;
use std::fmt;

use crate::error::FoliaError;
use crate::element::ElementData;

use strum_macros::EnumIter;

///Internal ID Type, also an Integer ID Type, these are valid only as long as store is in memory
pub type ElementKey = u32;

///Internal Processor ID Type
pub type ProcKey = u16;

///Internal Declaration ID Type (used for FoLiA sets)
pub type DecKey = u16;

pub type SubsetKey = u16;

///Class ID Type (used for FoLiA sets)
pub type ClassKey = u32;

pub type Class = String;
pub type Subset = String;

#[repr(u8)]
#[derive(Copy,Clone,PartialEq,Eq,PartialOrd,Hash,EnumIter,Serialize,Deserialize)]
///This represents the type of a FoLiA element.
//foliaspec:elementtype
pub enum ElementType { ActorFeature, Alternative, AlternativeLayers, BegindatetimeFeature, Caption, Cell, Chunk, ChunkingLayer, Comment, Content, CoreferenceChain, CoreferenceLayer, CoreferenceLink, Correction, Cue, Current, Definition, DependenciesLayer, Dependency, DependencyDependent, Description, Division, DomainAnnotation, EnddatetimeFeature, EntitiesLayer, Entity, Entry, ErrorDetection, Event, Example, External, Feature, Figure, FontFeature, ForeignData, FunctionFeature, Gap, Head, HeadFeature, Headspan, Hiddenword, Hyphbreak, Label, LangAnnotation, LemmaAnnotation, LevelFeature, Linebreak, LinkReference, List, ListItem, Metric, ModalitiesLayer, Modality, ModalityFeature, Morpheme, MorphologyLayer, New, Note, Observation, ObservationLayer, Original, Paragraph, Part, PhonContent, Phoneme, PhonologyLayer, PolarityFeature, PosAnnotation, Predicate, Quote, Reference, Relation, Row, Scope, SemanticRole, SemanticRolesLayer, SenseAnnotation, Sentence, Sentiment, SentimentLayer, SizeFeature, Source, SpanRelation, SpanRelationLayer, Speech, Statement, StatementLayer, StatementRelation, StrengthFeature, String, StyleFeature, SubjectivityAnnotation, Suggestion, SynsetFeature, SyntacticUnit, SyntaxLayer, Table, TableHead, Target, Term, Text, TextContent, TextMarkupCorrection, TextMarkupError, TextMarkupGap, TextMarkupHSpace, TextMarkupLanguage, TextMarkupReference, TextMarkupString, TextMarkupStyle, TextMarkupWhitespace, TimeFeature, TimeSegment, TimingLayer, Utterance, ValueFeature, Whitespace, Word, WordReference }

#[derive(Copy,Clone,PartialEq,Debug,Hash,EnumIter,Serialize,Deserialize)]
///This represents groups of element types that share similar characteristics.
//foliaspec:elementgroup
pub enum ElementGroup { Layer, Content, CorrectionChild, HigherOrder, Inline, Span, SpanRole, Structure, Subtoken, TextMarkup, Feature }

impl ElementGroup {
    ///Returns an array of all element types includes in this group
    pub fn elementtypes(&self) -> &'static [ElementType] {
        //foliaspec:elementgroup_elementtypes_map(self)
        match self {
            ElementGroup::Layer => &[ElementType::ChunkingLayer,ElementType::SpanRelationLayer,ElementType::CoreferenceLayer,ElementType::DependenciesLayer,ElementType::EntitiesLayer,ElementType::MorphologyLayer,ElementType::ObservationLayer,ElementType::PhonologyLayer,ElementType::SemanticRolesLayer,ElementType::SentimentLayer,ElementType::StatementLayer,ElementType::SyntaxLayer,ElementType::TimingLayer,ElementType::ModalitiesLayer],
            ElementGroup::Content => &[ElementType::TextContent,ElementType::PhonContent,ElementType::Content],
            ElementGroup::CorrectionChild => &[ElementType::Current,ElementType::New,ElementType::Original,ElementType::Suggestion],
            ElementGroup::HigherOrder => &[ElementType::Relation,ElementType::Alternative,ElementType::AlternativeLayers,ElementType::SpanRelation,ElementType::Correction,ElementType::Comment,ElementType::Description,ElementType::External,ElementType::Feature,ElementType::Metric,ElementType::String,ElementType::ForeignData,ElementType::Gap],
            ElementGroup::Inline => &[ElementType::DomainAnnotation,ElementType::ErrorDetection,ElementType::LangAnnotation,ElementType::LemmaAnnotation,ElementType::PosAnnotation,ElementType::SenseAnnotation,ElementType::SubjectivityAnnotation],
            ElementGroup::Span => &[ElementType::Chunk,ElementType::CoreferenceChain,ElementType::Modality,ElementType::Dependency,ElementType::Entity,ElementType::Observation,ElementType::Predicate,ElementType::SemanticRole,ElementType::Sentiment,ElementType::Statement,ElementType::SyntacticUnit,ElementType::TimeSegment],
            ElementGroup::SpanRole => &[ElementType::CoreferenceLink,ElementType::DependencyDependent,ElementType::Headspan,ElementType::StatementRelation,ElementType::Source,ElementType::Target,ElementType::Cue,ElementType::Scope],
            ElementGroup::Structure => &[ElementType::Caption,ElementType::Cell,ElementType::Definition,ElementType::Division,ElementType::Entry,ElementType::Event,ElementType::Example,ElementType::Figure,ElementType::Head,ElementType::Hiddenword,ElementType::Label,ElementType::Linebreak,ElementType::List,ElementType::ListItem,ElementType::Note,ElementType::Paragraph,ElementType::Part,ElementType::Quote,ElementType::Reference,ElementType::Row,ElementType::Sentence,ElementType::Speech,ElementType::Table,ElementType::TableHead,ElementType::Term,ElementType::Text,ElementType::Utterance,ElementType::Whitespace,ElementType::Word],
            ElementGroup::Subtoken => &[ElementType::Morpheme,ElementType::Phoneme],
            ElementGroup::TextMarkup => &[ElementType::TextMarkupCorrection,ElementType::TextMarkupError,ElementType::TextMarkupGap,ElementType::TextMarkupString,ElementType::TextMarkupStyle,ElementType::Hyphbreak,ElementType::TextMarkupReference,ElementType::TextMarkupWhitespace,ElementType::TextMarkupHSpace,ElementType::TextMarkupLanguage],
            ElementGroup::Feature => &[ElementType::ActorFeature,ElementType::BegindatetimeFeature,ElementType::EnddatetimeFeature,ElementType::FunctionFeature,ElementType::HeadFeature,ElementType::LevelFeature,ElementType::ModalityFeature,ElementType::PolarityFeature,ElementType::StrengthFeature,ElementType::StyleFeature,ElementType::SynsetFeature,ElementType::TimeFeature,ElementType::ValueFeature,ElementType::FontFeature,ElementType::SizeFeature],
        }

        //leave the above line empty for foliaspec to detect the end!
    }

    ///Checks whether the specified element type is a member of this group
    pub fn contains(&self, elementtype: ElementType) -> bool {
        self.elementtypes().contains(&elementtype)
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash,EnumIter,Serialize,Deserialize)]
///Defines all annotation types (as part of the AnnotationType enumeration)
//foliaspec:annotationtype
//Defines all annotation types (as part of the AnnotationType enumeration)
pub enum AnnotationType { TEXT, TOKEN, DIVISION, PARAGRAPH, HEAD, LIST, FIGURE, WHITESPACE, LINEBREAK, SENTENCE, POS, LEMMA, DOMAIN, SENSE, SYNTAX, CHUNKING, ENTITY, CORRECTION, ERRORDETECTION, PHON, SUBJECTIVITY, MORPHOLOGICAL, EVENT, DEPENDENCY, TIMESEGMENT, GAP, QUOTE, NOTE, REFERENCE, RELATION, SPANRELATION, COREFERENCE, SEMROLE, METRIC, LANG, STRING, TABLE, STYLE, PART, UTTERANCE, ENTRY, TERM, DEFINITION, EXAMPLE, PHONOLOGICAL, PREDICATE, OBSERVATION, SENTIMENT, STATEMENT, ALTERNATIVE, RAWCONTENT, COMMENT, DESCRIPTION, HYPHENATION, HIDDENTOKEN, MODALITY, EXTERNAL, HSPACE }

impl AnnotationType {
    ///Maps annotation types to strings
    pub fn as_str(&self) -> &'static str {
        //foliaspec:annotationtype_string_map(self)
        //A mapping from annotation types to strings
        match self {
          AnnotationType::ALTERNATIVE => "alternative",
          AnnotationType::CHUNKING => "chunking",
          AnnotationType::COMMENT => "comment",
          AnnotationType::RAWCONTENT => "rawcontent",
          AnnotationType::COREFERENCE => "coreference",
          AnnotationType::CORRECTION => "correction",
          AnnotationType::DEFINITION => "definition",
          AnnotationType::DEPENDENCY => "dependency",
          AnnotationType::DESCRIPTION => "description",
          AnnotationType::DIVISION => "division",
          AnnotationType::DOMAIN => "domain",
          AnnotationType::ENTITY => "entity",
          AnnotationType::ENTRY => "entry",
          AnnotationType::ERRORDETECTION => "errordetection",
          AnnotationType::EVENT => "event",
          AnnotationType::EXAMPLE => "example",
          AnnotationType::EXTERNAL => "external",
          AnnotationType::FIGURE => "figure",
          AnnotationType::GAP => "gap",
          AnnotationType::HEAD => "head",
          AnnotationType::HIDDENTOKEN => "hiddentoken",
          AnnotationType::HYPHENATION => "hyphenation",
          AnnotationType::LANG => "lang",
          AnnotationType::LEMMA => "lemma",
          AnnotationType::LINEBREAK => "linebreak",
          AnnotationType::LIST => "list",
          AnnotationType::METRIC => "metric",
          AnnotationType::MODALITY => "modality",
          AnnotationType::MORPHOLOGICAL => "morphological",
          AnnotationType::NOTE => "note",
          AnnotationType::OBSERVATION => "observation",
          AnnotationType::PARAGRAPH => "paragraph",
          AnnotationType::PART => "part",
          AnnotationType::PHON => "phon",
          AnnotationType::PHONOLOGICAL => "phonological",
          AnnotationType::POS => "pos",
          AnnotationType::PREDICATE => "predicate",
          AnnotationType::QUOTE => "quote",
          AnnotationType::REFERENCE => "reference",
          AnnotationType::RELATION => "relation",
          AnnotationType::SEMROLE => "semrole",
          AnnotationType::SENSE => "sense",
          AnnotationType::SENTENCE => "sentence",
          AnnotationType::SENTIMENT => "sentiment",
          AnnotationType::SPANRELATION => "spanrelation",
          AnnotationType::STATEMENT => "statement",
          AnnotationType::STRING => "string",
          AnnotationType::SUBJECTIVITY => "subjectivity",
          AnnotationType::SYNTAX => "syntax",
          AnnotationType::TABLE => "table",
          AnnotationType::TERM => "term",
          AnnotationType::TEXT => "text",
          AnnotationType::HSPACE => "hspace",
          AnnotationType::STYLE => "style",
          AnnotationType::WHITESPACE => "whitespace",
          AnnotationType::TIMESEGMENT => "timesegment",
          AnnotationType::UTTERANCE => "utterance",
          AnnotationType::TOKEN => "token",
        }

    }

    pub fn from_str(s: &str) -> Option<Self> {
        //foliaspec:string_annotationtype_map(s)
        //A mapping from strings to annotation types
        match s {
            "alternative" => Some(AnnotationType::ALTERNATIVE),
            "chunking" => Some(AnnotationType::CHUNKING),
            "comment" => Some(AnnotationType::COMMENT),
            "rawcontent" => Some(AnnotationType::RAWCONTENT),
            "coreference" => Some(AnnotationType::COREFERENCE),
            "correction" => Some(AnnotationType::CORRECTION),
            "definition" => Some(AnnotationType::DEFINITION),
            "dependency" => Some(AnnotationType::DEPENDENCY),
            "description" => Some(AnnotationType::DESCRIPTION),
            "division" => Some(AnnotationType::DIVISION),
            "domain" => Some(AnnotationType::DOMAIN),
            "entity" => Some(AnnotationType::ENTITY),
            "entry" => Some(AnnotationType::ENTRY),
            "errordetection" => Some(AnnotationType::ERRORDETECTION),
            "event" => Some(AnnotationType::EVENT),
            "example" => Some(AnnotationType::EXAMPLE),
            "external" => Some(AnnotationType::EXTERNAL),
            "figure" => Some(AnnotationType::FIGURE),
            "gap" => Some(AnnotationType::GAP),
            "head" => Some(AnnotationType::HEAD),
            "hiddentoken" => Some(AnnotationType::HIDDENTOKEN),
            "hyphenation" => Some(AnnotationType::HYPHENATION),
            "lang" => Some(AnnotationType::LANG),
            "lemma" => Some(AnnotationType::LEMMA),
            "linebreak" => Some(AnnotationType::LINEBREAK),
            "list" => Some(AnnotationType::LIST),
            "metric" => Some(AnnotationType::METRIC),
            "modality" => Some(AnnotationType::MODALITY),
            "morphological" => Some(AnnotationType::MORPHOLOGICAL),
            "note" => Some(AnnotationType::NOTE),
            "observation" => Some(AnnotationType::OBSERVATION),
            "paragraph" => Some(AnnotationType::PARAGRAPH),
            "part" => Some(AnnotationType::PART),
            "phon" => Some(AnnotationType::PHON),
            "phonological" => Some(AnnotationType::PHONOLOGICAL),
            "pos" => Some(AnnotationType::POS),
            "predicate" => Some(AnnotationType::PREDICATE),
            "quote" => Some(AnnotationType::QUOTE),
            "reference" => Some(AnnotationType::REFERENCE),
            "relation" => Some(AnnotationType::RELATION),
            "semrole" => Some(AnnotationType::SEMROLE),
            "sense" => Some(AnnotationType::SENSE),
            "sentence" => Some(AnnotationType::SENTENCE),
            "sentiment" => Some(AnnotationType::SENTIMENT),
            "spanrelation" => Some(AnnotationType::SPANRELATION),
            "statement" => Some(AnnotationType::STATEMENT),
            "string" => Some(AnnotationType::STRING),
            "subjectivity" => Some(AnnotationType::SUBJECTIVITY),
            "syntax" => Some(AnnotationType::SYNTAX),
            "table" => Some(AnnotationType::TABLE),
            "term" => Some(AnnotationType::TERM),
            "text" => Some(AnnotationType::TEXT),
            "hspace" => Some(AnnotationType::HSPACE),
            "style" => Some(AnnotationType::STYLE),
            "whitespace" => Some(AnnotationType::WHITESPACE),
            "timesegment" => Some(AnnotationType::TIMESEGMENT),
            "utterance" => Some(AnnotationType::UTTERANCE),
            "token" => Some(AnnotationType::TOKEN),
            _ => None
        }

    }

    pub fn as_element_str(&self) -> &'static str {
        //foliaspec:annotationtype_xml_map(self)
        //A mapping from annotation types to xml tags (strings)
        match self {
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
          AnnotationType::EXTERNAL => "external",
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
          AnnotationType::MODALITY => "modality",
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
          AnnotationType::HSPACE => "t-hspace",
          AnnotationType::STYLE => "t-style",
          AnnotationType::TIMESEGMENT => "timesegment",
          AnnotationType::UTTERANCE => "utt",
          AnnotationType::WHITESPACE => "whitespace",
          AnnotationType::TOKEN => "w",
        }

    }

    ///Maps annotation types to their layer elements
    pub fn layertype(&self) -> Option<ElementType> {
        //foliaspec:annotationtype_layerclass_map(self)
        match self {
            AnnotationType::CHUNKING => Some(ElementType::ChunkingLayer),
            AnnotationType::COREFERENCE => Some(ElementType::CoreferenceLayer),
            AnnotationType::DEPENDENCY => Some(ElementType::DependenciesLayer),
            AnnotationType::ENTITY => Some(ElementType::EntitiesLayer),
            AnnotationType::MODALITY => Some(ElementType::ModalitiesLayer),
            AnnotationType::MORPHOLOGICAL => Some(ElementType::MorphologyLayer),
            AnnotationType::OBSERVATION => Some(ElementType::ObservationLayer),
            AnnotationType::PHONOLOGICAL => Some(ElementType::PhonologyLayer),
            AnnotationType::SEMROLE => Some(ElementType::SemanticRolesLayer),
            AnnotationType::SENTIMENT => Some(ElementType::SentimentLayer),
            AnnotationType::SPANRELATION => Some(ElementType::SpanRelationLayer),
            AnnotationType::STATEMENT => Some(ElementType::StatementLayer),
            AnnotationType::SYNTAX => Some(ElementType::SyntaxLayer),
            AnnotationType::TIMESEGMENT => Some(ElementType::TimingLayer),
            AnnotationType::PREDICATE => Some(ElementType::SemanticRolesLayer),
            _ => None
        }

        //leave the above line empty for foliaspec!
    }
}

impl Into<&str> for AnnotationType {
    fn into(self) -> &'static str {
        self.as_str()
    }
}

impl fmt::Display for AnnotationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}


impl AnnotationType {
    ///A mapping from annotation types to element types, based on the assumption that there is always only one primary element for an annotation type (and possible multiple secondary ones which are not included in this map,w)
    pub fn elementtype(&self) -> ElementType {
        //foliaspec:annotationtype_elementtype_map(self)
        //A mapping from annotation types to element types, based on the assumption that there is always only one primary element for an annotation type (and possible multiple secondary ones which are not included in this map,w)
        match self {
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
            AnnotationType::EXTERNAL => ElementType::External,
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
            AnnotationType::MODALITY => ElementType::Modality,
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
            AnnotationType::HSPACE => ElementType::TextMarkupHSpace,
            AnnotationType::STYLE => ElementType::TextMarkupStyle,
            AnnotationType::TIMESEGMENT => ElementType::TimeSegment,
            AnnotationType::UTTERANCE => ElementType::Utterance,
            AnnotationType::WHITESPACE => ElementType::Whitespace,
            AnnotationType::TOKEN => ElementType::Word,
        }

        //leave the above line empty for foliaspec!
    }
}

impl ElementType {

    pub fn annotationtype(&self) -> Option<AnnotationType> {
        //foliaspec:elementtype_annotationtype_map(self)
        match self {
            ElementType::Alternative => Some(AnnotationType::ALTERNATIVE),
            ElementType::AlternativeLayers => Some(AnnotationType::ALTERNATIVE),
            ElementType::Chunk => Some(AnnotationType::CHUNKING),
            ElementType::ChunkingLayer => Some(AnnotationType::CHUNKING),
            ElementType::Comment => Some(AnnotationType::COMMENT),
            ElementType::Content => Some(AnnotationType::RAWCONTENT),
            ElementType::CoreferenceChain => Some(AnnotationType::COREFERENCE),
            ElementType::CoreferenceLayer => Some(AnnotationType::COREFERENCE),
            ElementType::CoreferenceLink => Some(AnnotationType::COREFERENCE),
            ElementType::Correction => Some(AnnotationType::CORRECTION),
            ElementType::Current => Some(AnnotationType::CORRECTION),
            ElementType::Definition => Some(AnnotationType::DEFINITION),
            ElementType::DependenciesLayer => Some(AnnotationType::DEPENDENCY),
            ElementType::Dependency => Some(AnnotationType::DEPENDENCY),
            ElementType::Description => Some(AnnotationType::DESCRIPTION),
            ElementType::Division => Some(AnnotationType::DIVISION),
            ElementType::DomainAnnotation => Some(AnnotationType::DOMAIN),
            ElementType::EntitiesLayer => Some(AnnotationType::ENTITY),
            ElementType::Entity => Some(AnnotationType::ENTITY),
            ElementType::Entry => Some(AnnotationType::ENTRY),
            ElementType::ErrorDetection => Some(AnnotationType::ERRORDETECTION),
            ElementType::Event => Some(AnnotationType::EVENT),
            ElementType::Example => Some(AnnotationType::EXAMPLE),
            ElementType::External => Some(AnnotationType::EXTERNAL),
            ElementType::Figure => Some(AnnotationType::FIGURE),
            ElementType::Gap => Some(AnnotationType::GAP),
            ElementType::Head => Some(AnnotationType::HEAD),
            ElementType::Hiddenword => Some(AnnotationType::HIDDENTOKEN),
            ElementType::Hyphbreak => Some(AnnotationType::HYPHENATION),
            ElementType::LangAnnotation => Some(AnnotationType::LANG),
            ElementType::LemmaAnnotation => Some(AnnotationType::LEMMA),
            ElementType::Linebreak => Some(AnnotationType::LINEBREAK),
            ElementType::List => Some(AnnotationType::LIST),
            ElementType::Metric => Some(AnnotationType::METRIC),
            ElementType::ModalitiesLayer => Some(AnnotationType::MODALITY),
            ElementType::Modality => Some(AnnotationType::MODALITY),
            ElementType::Morpheme => Some(AnnotationType::MORPHOLOGICAL),
            ElementType::MorphologyLayer => Some(AnnotationType::MORPHOLOGICAL),
            ElementType::New => Some(AnnotationType::CORRECTION),
            ElementType::Note => Some(AnnotationType::NOTE),
            ElementType::Observation => Some(AnnotationType::OBSERVATION),
            ElementType::ObservationLayer => Some(AnnotationType::OBSERVATION),
            ElementType::Original => Some(AnnotationType::CORRECTION),
            ElementType::Paragraph => Some(AnnotationType::PARAGRAPH),
            ElementType::Part => Some(AnnotationType::PART),
            ElementType::PhonContent => Some(AnnotationType::PHON),
            ElementType::Phoneme => Some(AnnotationType::PHONOLOGICAL),
            ElementType::PhonologyLayer => Some(AnnotationType::PHONOLOGICAL),
            ElementType::PosAnnotation => Some(AnnotationType::POS),
            ElementType::Predicate => Some(AnnotationType::PREDICATE),
            ElementType::Quote => Some(AnnotationType::QUOTE),
            ElementType::Reference => Some(AnnotationType::REFERENCE),
            ElementType::Relation => Some(AnnotationType::RELATION),
            ElementType::SemanticRole => Some(AnnotationType::SEMROLE),
            ElementType::SemanticRolesLayer => Some(AnnotationType::SEMROLE),
            ElementType::SenseAnnotation => Some(AnnotationType::SENSE),
            ElementType::Sentence => Some(AnnotationType::SENTENCE),
            ElementType::Sentiment => Some(AnnotationType::SENTIMENT),
            ElementType::SentimentLayer => Some(AnnotationType::SENTIMENT),
            ElementType::SpanRelation => Some(AnnotationType::SPANRELATION),
            ElementType::SpanRelationLayer => Some(AnnotationType::SPANRELATION),
            ElementType::Statement => Some(AnnotationType::STATEMENT),
            ElementType::StatementLayer => Some(AnnotationType::STATEMENT),
            ElementType::String => Some(AnnotationType::STRING),
            ElementType::SubjectivityAnnotation => Some(AnnotationType::SUBJECTIVITY),
            ElementType::Suggestion => Some(AnnotationType::CORRECTION),
            ElementType::SyntacticUnit => Some(AnnotationType::SYNTAX),
            ElementType::SyntaxLayer => Some(AnnotationType::SYNTAX),
            ElementType::Table => Some(AnnotationType::TABLE),
            ElementType::Term => Some(AnnotationType::TERM),
            ElementType::TextContent => Some(AnnotationType::TEXT),
            ElementType::TextMarkupCorrection => Some(AnnotationType::CORRECTION),
            ElementType::TextMarkupError => Some(AnnotationType::ERRORDETECTION),
            ElementType::TextMarkupGap => Some(AnnotationType::GAP),
            ElementType::TextMarkupHSpace => Some(AnnotationType::HSPACE),
            ElementType::TextMarkupLanguage => Some(AnnotationType::LANG),
            ElementType::TextMarkupReference => Some(AnnotationType::REFERENCE),
            ElementType::TextMarkupString => Some(AnnotationType::STRING),
            ElementType::TextMarkupStyle => Some(AnnotationType::STYLE),
            ElementType::TextMarkupWhitespace => Some(AnnotationType::WHITESPACE),
            ElementType::TimeSegment => Some(AnnotationType::TIMESEGMENT),
            ElementType::TimingLayer => Some(AnnotationType::TIMESEGMENT),
            ElementType::Utterance => Some(AnnotationType::UTTERANCE),
            ElementType::Whitespace => Some(AnnotationType::WHITESPACE),
            ElementType::Word => Some(AnnotationType::TOKEN),
            _ => None
        }

        //leave the above line empty for foliaspec!
    }
}

impl ElementType {
    pub fn is_in_group(&self, group: ElementGroup) -> bool {
        group.contains(*self)
    }

    pub fn as_str(&self) -> &'static str {
        //foliaspec:elementtype_string_map(self)
        match self {
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
          ElementType::Cue => "cue",
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
          ElementType::FontFeature => "font",
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
          ElementType::ModalitiesLayer => "modalities",
          ElementType::Modality => "modality",
          ElementType::ModalityFeature => "mod",
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
          ElementType::Scope => "scope",
          ElementType::SemanticRole => "semrole",
          ElementType::SemanticRolesLayer => "semroles",
          ElementType::SenseAnnotation => "sense",
          ElementType::Sentence => "s",
          ElementType::Sentiment => "sentiment",
          ElementType::SentimentLayer => "sentiments",
          ElementType::SizeFeature => "size",
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
          ElementType::TextMarkupHSpace => "t-hspace",
          ElementType::TextMarkupLanguage => "t-lang",
          ElementType::TextMarkupReference => "t-ref",
          ElementType::TextMarkupString => "t-str",
          ElementType::TextMarkupStyle => "t-style",
          ElementType::TextMarkupWhitespace => "t-whitespace",
          ElementType::TimeFeature => "time",
          ElementType::TimeSegment => "timesegment",
          ElementType::TimingLayer => "timing",
          ElementType::Utterance => "utt",
          ElementType::ValueFeature => "value",
          ElementType::Whitespace => "whitespace",
          ElementType::Word => "w",
          ElementType::WordReference => "wref",
        }

        //leave the above line empty for foliaspec!
    }
}


impl Into<&str> for ElementType {
    fn into(self) -> &'static str {
        self.as_str()
    }
}


impl Into<ElementType> for AnnotationType {
    fn into(self) -> ElementType {
        self.elementtype()
    }
}


impl fmt::Display for ElementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Debug for ElementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for ElementType {
    type Err = FoliaError;

    fn from_str(tag: &str) -> Result<Self, Self::Err> {
        //foliaspec:string_elementtype_map(tag)
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
          "cue" =>  Ok(ElementType::Cue),
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
          "font" =>  Ok(ElementType::FontFeature),
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
          "modalities" =>  Ok(ElementType::ModalitiesLayer),
          "modality" =>  Ok(ElementType::Modality),
          "mod" =>  Ok(ElementType::ModalityFeature),
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
          "scope" =>  Ok(ElementType::Scope),
          "semrole" =>  Ok(ElementType::SemanticRole),
          "semroles" =>  Ok(ElementType::SemanticRolesLayer),
          "sense" =>  Ok(ElementType::SenseAnnotation),
          "s" =>  Ok(ElementType::Sentence),
          "sentiment" =>  Ok(ElementType::Sentiment),
          "sentiments" =>  Ok(ElementType::SentimentLayer),
          "size" =>  Ok(ElementType::SizeFeature),
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
          "t-hspace" =>  Ok(ElementType::TextMarkupHSpace),
          "t-lang" =>  Ok(ElementType::TextMarkupLanguage),
          "t-ref" =>  Ok(ElementType::TextMarkupReference),
          "t-str" =>  Ok(ElementType::TextMarkupString),
          "t-style" =>  Ok(ElementType::TextMarkupStyle),
          "t-whitespace" =>  Ok(ElementType::TextMarkupWhitespace),
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

        //leave the above line empty for foliaspec!
    }
}


#[derive(Debug,PartialEq,Clone)]
///This data structure represents a data item in the document tree.
pub enum DataType {
    ///Text item
    Text(String),
    ///A reference to an element
    Element(ElementKey),
    ///Comment item
    Comment(String),
    ///A reference to a span element (the reverse of wref, not expressed explicitly in the XML)
    SpanReference(ElementKey),
    ///A to-be-constructed element, this is a only a temporary value that allows nested
    ///construction, it will not actually appear in the document's data model
    AddElement(ElementData),
}

impl DataType {
    pub fn text(text: &str) -> DataType {
        DataType::Text(text.to_string())
    }
    pub fn comment(text: &str) -> DataType {
        DataType::Comment(text.to_string())
    }
}


#[derive(Clone,Copy)]
pub enum BodyType {
    Text,
    Speech
}


