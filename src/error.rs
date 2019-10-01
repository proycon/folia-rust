use std::error::Error;
use std::fmt;
use std::io;


// ------------------------------ ERROR DEFINITIONS & IMPLEMENTATIONS -------------------------------------------------------------
//
#[derive(Debug)]
pub enum FoliaError {
    ///Indicates problems with the input/output (disk full? file not found?)
    IoError(io::Error),
    ///Parse errors indicate invalid XML
    XmlError(quick_xml::Error),
    ///Parse errors occur if there is a problem while parsing the input and is often indicative of
    ///invalidly formatted input.
    ParseError(String),
    ///Serialisation errors occur if there is a problem during serialisation
    SerialisationError(String),
    ///Validation errors occur when the FoLiA is not consistent, they are a level beyond Parse
    ///Errors
    ValidationError(String),
    ///Internal errors should never occur
    InternalError(String),
    EncodeError(String),
    KeyError(String),
    QueryError(String),
    TypeError(String),
    NoTextError(String),
    IndexError,
}

impl From<FoliaError> for String {
    fn from(error: FoliaError) -> String {
        match error {
            FoliaError::IoError(err) => format!("{}",err),
            FoliaError::XmlError(err) => format!("{}",err),
            FoliaError::ParseError(err) |
            FoliaError::SerialisationError(err) |
            FoliaError::ValidationError(err) |
            FoliaError::InternalError(err) |
            FoliaError::EncodeError(err) |
            FoliaError::NoTextError(err) |
            FoliaError::QueryError(err) |
            FoliaError::TypeError(err) |
            FoliaError::KeyError(err) => {
                err
            },
            FoliaError::IndexError => "".to_string(),
        }
    }
}

impl FoliaError {
    pub fn add_parseerror(msg: &str) -> Box<dyn FnOnce(FoliaError) -> FoliaError> {
        Self::add_parseerror_string(msg.to_string())
    }

    pub fn add_parseerror_string(mut msg: String) -> Box<dyn FnOnce(FoliaError) -> FoliaError> {
        Box::new( move |err| {
            msg += format!(" -> {}",err).as_str();
            FoliaError::ParseError(msg)
        })
    }
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
            FoliaError::ParseError(ref err) => "Parse Error",
            FoliaError::SerialisationError(ref err) => "Serialisation Error",
            FoliaError::ValidationError(ref err) => "Validation Error",
            FoliaError::InternalError(ref err) => "Internal Error",
            FoliaError::EncodeError(ref err) => "Encode Error",
            FoliaError::KeyError(ref err) => "Key Error",
            FoliaError::QueryError(ref err) => "Query Error",
            FoliaError::TypeError(ref err) => "Type Error",
            FoliaError::NoTextError(ref err) => "No Text Error",
            FoliaError::IndexError => "invalid index",
        }
    }

    fn cause(&self)  -> Option<&Error> {
        match *self {
            FoliaError::IoError(ref err) => Some(err as &Error),
            FoliaError::XmlError(ref _err) => None,
            FoliaError::ParseError(ref _err) => None,
            FoliaError::SerialisationError(ref _err) => None,
            FoliaError::ValidationError(ref _err) => None,
            FoliaError::InternalError(ref _err) => None,
            FoliaError::EncodeError(ref _err) => None,
            FoliaError::KeyError(ref _err) => None,
            FoliaError::QueryError(ref _err) => None,
            FoliaError::TypeError(ref _err) => None,
            FoliaError::NoTextError(ref _err) => None,
            FoliaError::IndexError => None,
        }
    }
}

impl fmt::Display for FoliaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FoliaError::IoError(ref err) => fmt::Display::fmt(err, f),
            FoliaError::XmlError(ref err) => fmt::Display::fmt(err, f),
            FoliaError::ParseError(ref err) |
            FoliaError::SerialisationError(ref err) |
            FoliaError::ValidationError(ref err) |
            FoliaError::InternalError(ref err) |
            FoliaError::EncodeError(ref err) |
            FoliaError::NoTextError(ref err) |
            FoliaError::QueryError(ref err) |
            FoliaError::TypeError(ref err) |
            FoliaError::KeyError(ref err) => {
                write!(f, "[{}] {}", self.description(),  err)
            }
            FoliaError::IndexError => fmt::Display::fmt("invalid index", f),
        }
    }
}

