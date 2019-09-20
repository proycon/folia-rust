use std::collections::HashMap;
use std::borrow::Cow;

use crate::common::*;
use crate::types::*;
use crate::error::*;
use crate::element::*;
use crate::store::*;
use crate::document::*;
use crate::specification::*;

///Holds and owns all elements, the index to them and their declarations. The store serves as an abstraction used by Documents
#[derive(Default)]
pub struct ElementStore {
    pub(crate) items: Vec<Option<Box<FoliaElement>>>, //heap-allocated
    pub(crate) index: HashMap<String,ElementKey>,

    ///An ``ElementStore`` holds a copy of the FoLiA specification. Duplicating this for each
    ///element store causes some duplicitity, but the specification itself contains mostly
    ///references to static strings and arrays contained within the library, and therefore only loaded once.
    pub specification: Specification,
}


