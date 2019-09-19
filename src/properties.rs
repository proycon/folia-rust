use crate::common::*;
use crate::types::*;


pub enum AcceptedData {
    AcceptElementType(ElementType),
    AcceptElementGroup(ElementGroup),
}


pub struct Properties {
    pub xmltag: String,
    pub annotationtype: AnnotationType,
    pub accepted_data: [AcceptedData],
    pub required_data: [AcceptedData],
    pub required_attribs: [AttribType],
    pub optional_attribs: [AttribType],
    pub occurrences: u32, //How often can this element occur under the parent? (0 = unlimited)
    pub occurrences_per_set: u32, //How often can a particular element+set combination occur under the parent (0 = unlimited)
    pub textdelimiter: Option<String>, //Delimiter to use when dynamically gathering text
    pub printable: bool, //Is this element printable? (i.e. can the text() method be called?)
    pub speakable: bool, //Is this element phonetically representablly? (i.e. can the phon() method be called?)
    pub hidden: bool, //Is this element hidden? (only applies to Hiddenword for now)
    pub xlink: bool, //Can the element carry xlink references?
    pub textcontainer: bool, //Does the element directly take textual content (e.g. TextContent (t) is a textcontainer)
    pub phoncontainer: bool, //Does the element directly take phonetic content (e.g. PhonContent (ph) is a phoncontainer)
    pub subset: Option<String>, //used for Feature subclasses
    pub auth: bool, //The default authoritative state for this element
    pub primaryelement: bool, //Is this the primary element for the advertised annotation type?
    pub auto_generate_id: bool, //Automatically generate an ID if none was provided?
    pub setonly: bool, //States that the element may take a set property only, and not a class property
    pub wrefable: bool //Indicates whether this element is referable as a token/word (applies only to a very select few elements, such as w, morpheme, and phoneme)
}

impl Properties {
    pub fn new(elementtype: ElementType) -> Self {
        //foliaspec:setelementproperties:elementtype

    }
}
