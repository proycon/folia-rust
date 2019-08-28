extern crate quick_xml;

pub mod error;
pub mod attrib;
pub mod element;
pub mod document;

pub use error::*;
pub use document::*;
pub use element::*;
pub use attrib::*;

const NSFOLIA: &[u8] = b"http://ilk.uvt.nl/folia";


