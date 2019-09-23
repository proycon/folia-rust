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


#[derive(Default,Clone)]
pub struct Query {
    pub action: Action,
    pub elementtype: Cmp<ElementType>,
    pub elementgroup: Cmp<ElementGroup>,
    pub set: Cmp<String>,
    pub class: Cmp<String>,
    pub processor: Cmp<String>,
    pub confidence: Cmp<f64>
}

#[derive(Clone,PartialEq)]
pub enum Cmp<T> {
    ///Any includes None, unlike Some
    Any,
    Is(T),
//    IsNot(T),  //add later
   ///Some does not include None, unlike Any
    Some,
    None,
    Unmatchable,
}

impl<T> Default for Cmp<T> {
    fn default() -> Cmp<T> {
        Cmp::Any
    }
}

impl<T>  Cmp<T> where T: PartialEq {
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
    pub fn element(mut self, value: Cmp<ElementType>) -> Self {
        self.elementtype = value;
        self
    }

    pub fn elementgroup(mut self, value: Cmp<ElementGroup>) -> Self {
        self.elementgroup = value;
        self
    }

    pub fn set(mut self, value: Cmp<String>) -> Self {
        self.set = value;
        self
    }

    pub fn class(mut self, value: Cmp<String>) -> Self {
        self.class = value;
        self
    }

    pub fn processor(mut self, value: Cmp<String>) -> Self {
        self.processor = value;
        self
    }

    pub fn confidence(mut self, value: Cmp<f64>) -> Self {
        self.confidence = value;
        self
    }

    pub fn select() -> Self {
        Self::default()
    }
}
