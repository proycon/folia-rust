#![allow(unused_imports,dead_code)] //TODO: remove later!
extern crate quick_xml;

extern crate strum;
#[macro_use]
extern crate strum_macros;



pub mod common;
pub mod types;
pub mod error;
pub mod attrib;
pub mod element;
pub mod store;
pub mod elementstore;
pub mod metadata;
pub mod select;
pub mod document;
pub mod parser;
pub mod serialiser;
pub mod properties;


pub use common::*;
pub use types::*;
pub use error::*;
pub use document::*;
pub use element::*;
pub use store::*;
pub use elementstore::*;
pub use attrib::*;
pub use select::*;
pub use properties::*;




