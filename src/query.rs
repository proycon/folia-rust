use std::fmt::Debug;

use crate::common::*;
use crate::types::*;
use crate::error::*;
use crate::element::*;
use crate::document::*;
use crate::attrib::*;
use crate::metadata::*;
use crate::store::*;
use crate::elementstore::*;


#[derive(Clone)]
pub enum Action {
    Select
}

impl Default for Action {
    fn default() -> Action {
        Action::Select
    }
}


///The query defines a search query on a FoLiA document
///with various matching criteria. It is turned into a ``Selector``, the encoded variant, when given a document.
#[derive(Default,Clone)]
pub struct Query {
    pub action: Action,
    pub elementtype: Cmp<ElementType>,
    pub elementgroup: Cmp<ElementGroup>,
    pub contexttype: Cmp<ElementType>, //needed for features
    pub set: Cmp<String>,
    pub class: Cmp<String>,
    pub processor: Cmp<String>,
    pub subset: Cmp<String>,
    pub annotator: Cmp<String>,
    pub annotatortype: Cmp<ProcessorType>,
    pub confidence: Cmp<f64>
}

#[derive(Clone,PartialEq,Debug)]
///A comparison structure, used in building queries
pub enum Cmp<T> where T: Debug {
    ///Any includes None, unlike Some
    Any,
    Is(T),
//    IsNot(T),  //add later
   ///Some does not include None, unlike Any
    Some,
    None,
    Unmatchable,
}

impl<T> Default for Cmp<T> where T: Debug {
    fn default() -> Cmp<T> {
        Cmp::Any
    }
}

impl<T>  Cmp<T> where T: PartialEq, T: Debug {
    ///Tests the comparison against another
    pub fn matches(&self, other: Option<&T>) -> bool {
        match self {
            Cmp::Any => true,
            Cmp::Is(value) => {
                if let Some(refvalue) = other {
                    value == refvalue
                } else {
                    false
                }
            },
            Cmp::None => other.is_none(),
            Cmp::Some => other.is_some(),
            Cmp::Unmatchable => false,
        }
    }
}


impl Query {
    ///Add a matching criterion on element type
    pub fn element(mut self, value: Cmp<ElementType>) -> Self {
        self.elementtype = value;
        self
    }

    ///Add a matching criterion on element type for the context
    pub fn contexttype(mut self, value: Cmp<ElementType>) -> Self {
        self.contexttype = value;
        self
    }

    ///Add a matching criterion on element group
    pub fn elementgroup(mut self, value: Cmp<ElementGroup>) -> Self {
        self.elementgroup = value;
        self
    }

    ///Add a matching criterion on FoLiA set
    pub fn set(mut self, value: Cmp<String>) -> Self {
        self.set = value;
        self
    }

    ///Add a matching criterion on FoLiA class
    pub fn class(mut self, value: Cmp<String>) -> Self {
        self.class = value;
        self
    }

    ///Add a matching criterion on processor
    pub fn processor(mut self, value: Cmp<String>) -> Self {
        self.processor = value;
        self
    }

    ///Add a matching criterion on annotator (i.e. the name of a processor)
    pub fn annotator(mut self, value: Cmp<String>) -> Self {
        self.annotator = value;
        self
    }

    ///Add a matching criterion on annotator type (i.e. the type of a processor)
    pub fn annotatortype(mut self, value: Cmp<ProcessorType>) -> Self {
        self.annotatortype = value;
        self
    }

    ///Add a matching criterion on a FoLiA subset
    pub fn subset(mut self, value: Cmp<String>) -> Self {
        self.subset = value;
        self
    }


    ///Add a matching criterion on confidence value
    pub fn confidence(mut self, value: Cmp<f64>) -> Self {
        self.confidence = value;
        self
    }

    ///Creates an empty (all matching) select query
    pub fn select() -> Self {
        Self::default()
    }
}
