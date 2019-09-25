use std::collections::HashMap;
use std::borrow::Cow;

use crate::common::*;
use crate::types::*;
use crate::error::*;
use crate::element::*;
use crate::store::*;
use crate::document::*;
use crate::specification::*;

///Holds and owns all elements and the index to them. The store serves as an abstraction used by Documents
pub struct ElementStore {
    pub(crate) items: Vec<Option<Box<ElementData>>>, //heap-allocated
    pub(crate) index: HashMap<String,ElementKey>,

    ///An extra field to hold the document root as DataType. It's practically
    ///always DataType::Element(0) and exists primarily
    ///so a borrow can be taken in a similar way as one can can borrow from the data vector
    ///inside elements. Used by the select iterator.
    pub(crate) root: DataType,

    ///An ``ElementStore`` holds a copy of the FoLiA specification. Duplicating this for each
    ///element store causes some duplication when holding multiple documents (or stores) in memory, but the specification itself contains mostly
    ///references to static strings and arrays contained within the library, and therefore only loaded once.
    pub specification: Specification,
}

impl Default for ElementStore {
    fn default() -> Self {
        ElementStore {
            items: vec![],
            index: HashMap::new(),
            root: DataType::Element(0),
            specification: Specification::default()
        }
    }
}


