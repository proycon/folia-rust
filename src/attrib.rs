use std::io::BufRead;
use std::io::BufReader;
use std::borrow::Cow;
use std::str::FromStr;
use std::string::ToString;
use std::convert::Into;
use std::fmt;

use quick_xml::Reader;
use quick_xml::events::Event;

use crate::error::*;
use crate::common::*;
use crate::types::*;
use crate::store::*;

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum AttribType { //not from foliaspec because we add more individual attributes that are not grouped together like in the specification
    ID, SET, CLASS, ANNOTATOR, ANNOTATORTYPE, CONFIDENCE, N, DATETIME, BEGINTIME, ENDTIME, SRC, SPEAKER, TEXTCLASS, METADATA, IDREF, SPACE, PROCESSOR, HREF, FORMAT, SUBSET
}

impl Into<&str> for AttribType {
    fn into(self) -> &'static str {
         match self {
            AttribType::ID => "xml:id",
            AttribType::SET => "set",
            AttribType::CLASS => "class",
            AttribType::ANNOTATOR => "annotator",
            AttribType::ANNOTATORTYPE => "annotatortype",
            AttribType::CONFIDENCE => "confidence",
            AttribType::N => "n",
            AttribType::DATETIME => "datetime",
            AttribType::BEGINTIME => "begintime",
            AttribType::ENDTIME => "endtime",
            AttribType::SRC => "src",
            AttribType::SPEAKER => "speaker",
            AttribType::TEXTCLASS => "textclass",
            AttribType::METADATA => "metadata",
            AttribType::IDREF => "id",
            AttribType::SPACE => "space",
            AttribType::PROCESSOR => "processor",
            AttribType::HREF => "href",
            AttribType::FORMAT => "format",
            AttribType::SUBSET => "subset"
        }
    }
}

impl fmt::Display for AttribType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone,PartialEq)]
pub enum Attribute {
    Id(String),
    Set(String),
    DeclarationRef(DecKey), //encoded form for (annotationtype,set)
    Class(String),
    ClassRef(ClassKey),
    Annotator(String),
    AnnotatorType(String),
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
    Space(bool),

    Processor(String),
    ProcessorRef(ProcKey), //encoded form
    Href(String),
    Format(String),
    Subset(String),
    SubsetRef(SubsetKey), //encoded form

}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(s) = self.as_str() {
            write!(f, "{}",  s )
        } else {
            write!(f, "{}",  self.to_string().expect("Attribute can not be displayed without decoding") )
        }
    }
}



impl Attribute {

    ///Can the attribute be decoded? Implies it is currently encoded (key instead of string)
    pub fn decodable(&self) -> bool {
        match self {
            Attribute::DeclarationRef(_) | Attribute::ClassRef(_) | Attribute::ProcessorRef(_) | Attribute::SubsetRef(_) => true,
            _ => false,
        }
    }

    ///Can the attribute be encoded? Implies it is currently decoded (string instead of key)
    pub fn encodable(&self) -> bool {
        match self {
            Attribute::Set(_) | Attribute::Class(_) | Attribute::Processor(_) | Attribute::Subset(_) => true,
            _ => false,
        }
    }

    pub fn as_str(&self) -> Result<&str,FoliaError> {
        match self {
            Attribute::Id(s) | Attribute::Set(s) | Attribute::Class(s) | Attribute::Annotator(s) |
            Attribute::AnnotatorType(s) | Attribute::N(s) | Attribute::DateTime(s) | Attribute::BeginTime(s) | Attribute::EndTime(s) |
            Attribute::Src(s) | Attribute::Speaker(s) | Attribute::Textclass(s) | Attribute::Metadata(s) | Attribute::Idref(s) |
            Attribute::Processor(s) | Attribute::Href(s) | Attribute::Format(s) | Attribute::Subset(s)
                => Ok(&s),
            Attribute::Space(b) => { if *b { Ok("yes") } else { Ok("no") } },
            _ =>  Err(FoliaError::TypeError("Attribute can't be cast as_str, use to_string() instead".to_string()))
        }
    }

    pub fn to_string(&self) -> Result<String,FoliaError> {
        match self {
            Attribute::Confidence(f) => Ok(f.to_string()),
            _ =>  {
                if let Ok(s) = self.as_str() {
                    Ok(s.to_string())
                } else {
                    Err(FoliaError::TypeError("Attribute can't be cast to_string() without decoding".to_string()))
                }
            }
        }
    }


    pub fn sametype(&self, other: &Attribute) -> bool {
        self.attribtype() == other.attribtype()
    }

    pub fn attribtype(&self) -> AttribType {
        match self {
            Attribute::Id(_) => AttribType::ID,
            Attribute::Set(_) => AttribType::SET,
            Attribute::DeclarationRef(_) => AttribType::SET,
            Attribute::Class(_) => AttribType::CLASS,
            Attribute::ClassRef(_) => AttribType::CLASS,
            Attribute::Annotator(_) => AttribType::ANNOTATOR,
            Attribute::AnnotatorType(_) => AttribType::ANNOTATORTYPE,
            Attribute::Confidence(_) => AttribType::CONFIDENCE,
            Attribute::N(_) => AttribType::N,
            Attribute::DateTime(_) => AttribType::DATETIME,
            Attribute::BeginTime(_) => AttribType::BEGINTIME,
            Attribute::EndTime(_) => AttribType::ENDTIME,
            Attribute::Src(_) => AttribType::SRC,
            Attribute::Speaker(_) => AttribType::SPEAKER,
            Attribute::Textclass(_) => AttribType::TEXTCLASS,
            Attribute::Metadata(_) => AttribType::METADATA,
            Attribute::Idref(_) => AttribType::IDREF,
            Attribute::Space(_) => AttribType::SPACE,
            Attribute::Processor(_) => AttribType::PROCESSOR,
            Attribute::ProcessorRef(_) => AttribType::PROCESSOR,
            Attribute::Href(_) => AttribType::HREF,
            Attribute::Format(_) => AttribType::FORMAT,
            Attribute::Subset(_) => AttribType::SUBSET,
            Attribute::SubsetRef(_) => AttribType::SUBSET,
        }
    }

    ///The attribute type class is a generalisation of the attrib type, some inter-dependent attrib types are part
    ///of the same attribute type class, which themselves are just a subset of the attribute types
    ///and are used in the required_attribs and optional_attribs properties.
    pub fn attribtypeclass(&self) -> AttribType {
        let attribtype = self.attribtype();
        match attribtype {
            AttribType::SET => AttribType::CLASS,
            AttribType::PROCESSOR => AttribType::ANNOTATOR,
            AttribType::ANNOTATORTYPE => AttribType::ANNOTATOR,
            _  => attribtype,
        }
    }

    ///Parse an XML attribute into a FoLiA Attribute
    pub fn parse<R: BufRead>(reader: &Reader<R>, attrib: &quick_xml::events::attributes::Attribute) -> Result<Attribute,FoliaError> {
        if let Ok(value) = attrib.unescape_and_decode_value(&reader) {
            match attrib.key {
                b"xml:id" => {
                    Ok(Attribute::Id(value))
                },
                b"set" => {
                    Ok(Attribute::Set(value))
                },
                b"class" => {
                    Ok(Attribute::Class(value))
                },
                b"processor" => {
                    Ok(Attribute::Processor(value))
                },
                b"annotator" => {
                    Ok(Attribute::Annotator(value))
                },
                b"annotatortype" => {
                    Ok(Attribute::AnnotatorType(value))
                },
                b"subset" => {
                    Ok(Attribute::Subset(value))
                },
                b"xlink:format" => {
                    Ok(Attribute::Format(value))
                },
                b"xlink:href" => {
                    Ok(Attribute::Href(value))
                },
                b"speaker" => {
                    Ok(Attribute::Speaker(value))
                },
                b"src" => {
                    Ok(Attribute::Src(value))
                },
                b"n" => {
                    Ok(Attribute::N(value))
                },
                b"datetime" => {
                    Ok(Attribute::DateTime(value))
                },
                b"begintime" => {
                    Ok(Attribute::BeginTime(value))
                },
                b"endtime" => {
                    Ok(Attribute::EndTime(value))
                },
                b"textclass" => {
                    Ok(Attribute::Textclass(value))
                },
                b"metadata" => {
                    Ok(Attribute::Metadata(value))
                },
                b"id" => {
                    Ok(Attribute::Idref(value))
                },
                b"confidence" => {
                    if let Ok(value) = f64::from_str(&value) {
                        Ok(Attribute::Confidence(value))
                    } else {
                        Err(FoliaError::ParseError(format!("Invalid confidence value: '{}'", value)))
                    }
                },
                b"space" => {
                    match value.as_str() {
                        "yes" | "true" => Ok(Attribute::Space(true)),
                        "no" | "false" => Ok(Attribute::Space(false)),
                        _ => Err(FoliaError::ParseError(format!("Invalid space value: '{}'", value)))
                    }
                },
                _ => {
                    //TODO: handle feature/subset attributes
                    Err(FoliaError::ParseError(format!("Unknown attribute: '{}'", std::str::from_utf8(attrib.key).expect("unable to parse attribute name"))))
                }
            }
        } else {
            Err(FoliaError::ParseError("Unable to parse attribute value (invalid utf-8?)".to_string()))
        }
    }
}

