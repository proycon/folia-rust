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
use crate::metadata::*;
use crate::store::*;

#[derive(Debug,Copy,Clone,PartialEq)]
///Attribute Type
pub enum AttribType { //not from foliaspec because we add more individual attributes that are not grouped together like in the specification
    NONE, ID, SET, CLASS, ANNOTATOR, ANNOTATORTYPE, CONFIDENCE, N, DATETIME, BEGINTIME, ENDTIME, SRC, SPEAKER, TEXTCLASS, METADATA, IDREF, SPACE, PROCESSOR, HREF, FORMAT, SUBSET, TEXT, TYPE, AUTH, OFFSET, REF, ORIGINAL, LINENR, PAGENR, NEWPAGE, XLINKTYPE
}

impl Into<&str> for AttribType {
    ///Get a string representation of the attribute type
    fn into(self) -> &'static str {
         match self {
            AttribType::NONE => "NONE", //should not be serialised, need to check in advance
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
            AttribType::HREF => "xlink:href",
            AttribType::FORMAT => "format",
            AttribType::SUBSET => "subset",
            AttribType::TEXT => "t",
            AttribType::TYPE => "type",
            AttribType::AUTH => "auth",
            AttribType::OFFSET => "offset",
            AttribType::REF => "ref",
            AttribType::LINENR => "linenr",
            AttribType::PAGENR => "pagenr",
            AttribType::NEWPAGE => "newpage",
            AttribType::ORIGINAL => "original",
            AttribType::XLINKTYPE => "xlink:type",
        }
    }
}

impl fmt::Display for AttribType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone,PartialEq)]
///This type hold attributes (including the attribute value)
pub enum Attribute {
    Ignore,
    Id(String),
    Set(String),
    DeclarationRef(DecKey), //encoded form for (annotationtype,set)
    Class(String),
    ClassRef(ClassKey),
    Annotator(String),
    AnnotatorType(ProcessorType),
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
    Text(String),
    Type(String), //used by references
    Auth(String), //for backward compatibility
    Offset(u16),
    Ref(String),
    Original(String), //used by t-correction
    LineNr(u16), //used by linebreak
    PageNr(String), //used by linebreak
    NewPage(bool), //used by linebreak
    XLinkType(String),

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

    ///Get the attribute value as a ``&str``, note that this does only works
    ///for attributes from which a string slice can be borrowed. Use ``to_string()`` for
    ///the others.
    pub fn as_str(&self) -> Result<&str,FoliaError> {
        match self {
            Attribute::Id(s) | Attribute::Set(s) | Attribute::Class(s) | Attribute::Annotator(s) |
            Attribute::N(s) | Attribute::DateTime(s) | Attribute::BeginTime(s) | Attribute::EndTime(s) |
            Attribute::Src(s) | Attribute::Speaker(s) | Attribute::Textclass(s) | Attribute::Metadata(s) | Attribute::Idref(s) |
            Attribute::Processor(s) | Attribute::Href(s) | Attribute::Format(s) | Attribute::Subset(s) | Attribute::Text(s)| Attribute::Type(s) | Attribute::Ref(s) | Attribute::Original(s) | Attribute::Auth(s) | Attribute::XLinkType(s) | Attribute::PageNr(s)
                => Ok(&s),
            Attribute::AnnotatorType(t) => Ok(t.as_str()),
            Attribute::Space(b) => { if *b { Ok("yes") } else { Ok("no") } },
            Attribute::NewPage(b) => { if *b { Ok("yes") } else { Ok("no") } },
            Attribute::Ignore => Err(FoliaError::TypeError("Ignore attribute can't be serialised".to_string())),
            _ =>  {
                let attribtype: &str  = self.attribtype().into();
                Err(FoliaError::TypeError(format!("Attribute {} can't be cast as_str, use to_string() instead", attribtype)))
            }
        }
    }

    ///Convert the attribute value to string. This does not work for encoded attributes (i.e.
    ///attributes that refer to a certain key), those need
    ///explicit decoding first.
    pub fn to_string(&self) -> Result<String,FoliaError> {
        match self {
            Attribute::Confidence(f) => Ok(f.to_string()),
            Attribute::Offset(n) => Ok(n.to_string()),
            Attribute::LineNr(n) => Ok(n.to_string()),
            Attribute::Ignore => Err(FoliaError::TypeError("Ignore attribute can't be serialised".to_string())),
            _ =>  {
                if let Ok(s) = self.as_str() {
                    Ok(s.to_string())
                } else {
                    let attribtype: &str  = self.attribtype().into();
                    Err(FoliaError::TypeError(format!("Attribute {} can't be cast to_string() without decoding", attribtype)))
                }
            },
        }
    }


    ///Tests if two attributes are the same type (does not take their values into account)
    pub fn sametype(&self, other: &Attribute) -> bool {
        self.attribtype() == other.attribtype()
    }

    ///Get the type of the attribute
    pub fn attribtype(&self) -> AttribType {
        match self {
            Attribute::Ignore => AttribType::NONE,
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
            Attribute::XLinkType(_) => AttribType::XLINKTYPE,
            Attribute::Subset(_) => AttribType::SUBSET,
            Attribute::SubsetRef(_) => AttribType::SUBSET,
            Attribute::Text(_) => AttribType::TEXT,
            Attribute::Type(_) => AttribType::TYPE,
            Attribute::Offset(_) => AttribType::OFFSET,
            Attribute::Ref(_) => AttribType::REF,
            Attribute::Auth(_) => AttribType::AUTH,
            Attribute::Original(_) => AttribType::ORIGINAL,
            Attribute::LineNr(_) => AttribType::LINENR,
            Attribute::PageNr(_) => AttribType::PAGENR,
            Attribute::NewPage(_) => AttribType::NEWPAGE
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
                    match value.as_str() {
                        "auto" => Ok(Attribute::AnnotatorType(ProcessorType::Auto)),
                        "manual" => Ok(Attribute::AnnotatorType(ProcessorType::Manual)),
                        "generator" => Ok(Attribute::AnnotatorType(ProcessorType::Generator)),
                        "datasource" => Ok(Attribute::AnnotatorType(ProcessorType::DataSource)),
                        other => Err(FoliaError::ParseError(format!("Invalid value for annotatortype: {}", other)))
                    }
                },
                b"subset" => {
                    Ok(Attribute::Subset(value))
                },
                b"format" => {
                    Ok(Attribute::Format(value))
                },
                b"xlink:href" => {
                    Ok(Attribute::Href(value))
                },
                b"xlink:type" => {
                    Ok(Attribute::XLinkType(value))
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
                b"t" => {
                    Ok(Attribute::Text(value))
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
                b"type" => {
                    Ok(Attribute::Type(value))
                },
                b"auth" => {
                    Ok(Attribute::Auth(value))
                },
                b"original" => {
                    Ok(Attribute::Original(value))
                },
                b"pagenr" => {
                    Ok(Attribute::PageNr(value))
                },
                b"offset" => {
                    if let Ok(value) = u16::from_str(&value) {
                        Ok(Attribute::Offset(value))
                    } else {
                        Err(FoliaError::ParseError(format!("Invalid offset value: '{}'", value)))
                    }
                },
                b"linenr" => {
                    if let Ok(value) = u16::from_str(&value) {
                        Ok(Attribute::LineNr(value))
                    } else {
                        Err(FoliaError::ParseError(format!("Invalid line number value: '{}'", value)))
                    }
                },
                b"newpage" => {
                    match value.as_str() {
                        "yes" | "true" => Ok(Attribute::NewPage(true)),
                        "no" | "false" => Ok(Attribute::NewPage(false)),
                        _ => Err(FoliaError::ParseError(format!("Invalid newpage value: '{}'", value)))
                    }
                },
                b"ref" => {
                    Ok(Attribute::Ref(value))
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
                attrib_key => {
                    if attrib_key.contains(&58) { //58 is a colon, we assume alien namespaces and ignore it
                        Ok(Attribute::Ignore)
                    } else {
                        Err(FoliaError::ParseError(format!("Unknown attribute: '{}'", std::str::from_utf8(attrib.key).expect("unable to parse attribute name"))))
                    }
                }
            }
        } else {
            Err(FoliaError::ParseError("Unable to parse attribute value (invalid utf-8?)".to_string()))
        }
    }
}

