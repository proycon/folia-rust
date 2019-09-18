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
            FoliaError::XmlError(ref _err) => "XML Error",
            FoliaError::ParseError(ref err) => err,
            FoliaError::SerialisationError(ref err) => err,
            FoliaError::ValidationError(ref err) => err,
            FoliaError::InternalError(ref err) => err,
            FoliaError::EncodeError(ref err) => err,
            FoliaError::KeyError(ref err) => err,
            FoliaError::IndexError => "invalid index",
        }
    }

    fn cause(&self)  -> Option<&Error> {
        match *self {
            FoliaError::IoError(ref err) => Some(err as &Error),
            FoliaError::XmlError(ref _err) => None,
            FoliaError::ParseError(ref _err) => None, //TODO
            FoliaError::SerialisationError(ref _err) => None, //TODO
            FoliaError::ValidationError(ref _err) => None, //TODO
            FoliaError::InternalError(ref _err) => None, //TODO
            FoliaError::EncodeError(ref _err) => None, //TODO
            FoliaError::KeyError(ref _err) => None, //TODO
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
            FoliaError::SerialisationError(ref err) => fmt::Display::fmt(err, f),
            FoliaError::ValidationError(ref err) => fmt::Display::fmt(err, f),
            FoliaError::InternalError(ref err) => fmt::Display::fmt(err, f),
            FoliaError::EncodeError(ref err) => fmt::Display::fmt(err, f),
            FoliaError::KeyError(ref err) => fmt::Display::fmt(err, f),
            FoliaError::IndexError => fmt::Display::fmt("invalid index", f),
        }
    }
}
