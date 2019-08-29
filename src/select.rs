use crate::common::*;
use crate::error::*;
use crate::element::*;
use crate::attrib::*;
use crate::elementstore::*;

#[derive(Debug,Clone)]
pub struct Selector {
    pub typeselector: TypeSelector,
    pub setselector: SetSelector,
    pub next: Option<Box<Selector>>
}

impl Selector {
    pub fn new(typeselector: TypeSelector, setselector: SetSelector) -> Selector {
        Selector { typeselector: typeselector, setselector: setselector, next: None }
    }

    pub fn and(mut self, selector: Selector) -> Self {
        self.next = Some(Box::new(selector));
        self
    }

    pub fn matches(&self, store: &ElementStore, item: &DataType) -> bool {
        //we attempt to falsify the match
        let matches = match item {
            DataType::Element(intid) => {
                if let TypeSelector::Text | TypeSelector::Comment  = self.typeselector {
                    false
                } else if let Some(element) = store.get(*intid) {
                    match self.setselector {
                         SetSelector::SomeSet(set) => {
                             if let Some(set2) = element.set() {
                                 set == set2
                             } else {
                                 false
                             }
                         },
                         SetSelector::NoSet => {
                             element.set().is_none()
                         },
                         SetSelector::AnySet => true,
                    }
                } else {
                    //element does not exist, can never match
                    false
                }
            },
            DataType::Text(_) => {
                if let TypeSelector::AnyType | TypeSelector::Text  = self.typeselector {
                    true
                } else {
                    false
                }
            },
            DataType::Comment(_) => {
                if let TypeSelector::AnyType | TypeSelector::Comment  = self.typeselector {
                    true
                } else {
                    false
                }
            }
        };
        if let Some(next) = self.next {
            matches || next.matches(store, item)
        } else {
            matches
        }
    }
}



#[derive(Debug,Clone)]
pub enum SetSelector {
    SomeSet(String),
    AnySet,
    NoSet
}

#[derive(Debug,Clone)]
pub enum TypeSelector {
    SomeElement(ElementType),
    AnyElement,
    AnyType,
    Text,
    Comment,
}

pub struct SelectIterator<'a> {
    store: &'a ElementStore,
    selector: Selector,
    ///The current stack, containing the element and cursor within that element
    stack: Vec<(IntId,usize)>,
}

impl<'a> SelectIterator<'a> {
    pub fn new(store: &'a ElementStore, selector: Selector, intid: IntId) -> SelectIterator<'a> {
        SelectIterator {
            store: store,
            selector: selector,
            stack: vec![(intid,0)]
        }
    }

}

impl<'a> Iterator for SelectIterator<'a> {
    type Item = &'a DataType;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((intid,cursor)) = self.stack.last().map(|(x,y)| (*x,*y)) {
            if let Some(parent) = self.store.get(intid) {
                if let Some(item) = parent.get(cursor) {
                    //compute the next element
                    match item {
                        DataType::Element(intid) => {
                            self.stack.push((*intid,0));
                        },
                        _ => {
                            //temporarily pop the last element of the stack
                            self.stack.pop();
                            //increment the cursor
                            let cursor = cursor + 1;
                            //push it back if the cursor does not exceed the length
                            if cursor < parent.len() {
                                self.stack.push((intid, cursor));
                            }
                        }
                    };
                    //return the current one
                    if self.selector.matches(self.store, item) {
                        Some(item)
                    } else {
                        self.next()
                    }
                } else {
                    unreachable!("selector cursor index out of bounds")
                }
            } else {
                unreachable!("selector tried to get an element which no longer exists")
            }
        } else {
            //stack is empty, we are done (None stops iteration)
            None
        }
    }

}

pub trait Select<'a> {
    fn select(&'a self, intid: IntId, selector: Selector, recursive: bool) -> SelectIterator<'a>;
}

impl<'a> Select<'a> for ElementStore {
    fn select(&'a self, intid: IntId, selector: Selector, recursive: bool) -> SelectIterator<'a> {
        SelectIterator::new(self, selector, intid)
    }
}
