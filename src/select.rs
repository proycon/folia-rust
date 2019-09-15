use crate::common::*;
use crate::types::*;
use crate::error::*;
use crate::element::*;
use crate::document::*;
use crate::attrib::*;
use crate::metadata::*;
use crate::store::*;
use crate::elementstore::*;



#[derive(Debug,Clone,Default)]
pub struct Selector {
    pub typeselector: TypeSelector,
    pub setselector: SetSelector,
    pub classselector: ClassSelector,
    pub next: Option<Box<Selector>>
}

impl Selector {

    pub fn new(typeselector: TypeSelector, setselector: SetSelector, classselector: ClassSelector) -> Self {
        Selector { typeselector: typeselector, setselector: setselector, classselector: classselector, next: None }
    }

    ///Sets the selector
    pub fn with(mut self, document: &Document, elementtype: ElementType, set: SelectorValue, class: SelectorValue) -> Self {
        self.typeselector = TypeSelector::SomeElement(elementtype);
        if let Some(annotationtype) = elementtype.annotationtype() {
            self.setselector = match set {
                SelectorValue::Some(set) => {
                    let id = DeclarationStore::index_id(annotationtype, &Some(set));
                    //add a set filter,
                    if let Some(dec_key) = document.declarationstore.id_to_key(id.as_str()) {
                        SetSelector::SomeSet(dec_key)
                    } else {
                        SetSelector::Unmatchable
                    }
                },
                SelectorValue::Any => SetSelector::AnySet,
                SelectorValue::None => SetSelector::NoSet,
            };
            self.classselector = match class {
                SelectorValue::Some(class) => {
                    //add a class filter
                    let mut result = ClassSelector::Unmatchable;
                    if let SetSelector::SomeSet(dec_key) = self.setselector {
                        if let Some(declaration) = document.declarationstore.get(dec_key) {
                            if let Some(classes) = declaration.classes {
                                if let Some(class_key) = classes.id_to_key(class) {
                                      result = ClassSelector::SomeClass(class_key);
                                }
                            }
                        }
                    }
                    result
                },
                SelectorValue::Any =>  ClassSelector::AnyClass,
                SelectorValue::None => ClassSelector::NoClass,
            }
        }
        self
    }


    pub fn and(mut self, selector: Selector) -> Self {
        self.next = Some(Box::new(selector));
        self
    }

    pub fn matchable(&self) -> bool {
        self.typeselector != TypeSelector::Unmatchable &&
        self.setselector != SetSelector::Unmatchable &&
        self.classselector != ClassSelector::Unmatchable
    }

    pub fn matches(&self, store: &ElementStore, item: &DataType) -> bool {
        //we attempt to falsify the match
        let matches = match item {
            DataType::Element(key) => {
                if let TypeSelector::Text | TypeSelector::Comment  = self.typeselector {
                    false
                } else if let Some(element) = store.get(*key) {
                    match &self.setselector {
                         SetSelector::SomeSet(set) => {
                             if let Some(set2) = element.set_key() {
                                 *set == set2
                             } else {
                                 false
                             }
                         },
                         SetSelector::NoSet => {
                             element.set_key().is_none()
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
        if let Some(next) = &self.next {
            matches || next.matches(store, item)
        } else {
            matches
        }
    }
}


#[derive(Debug,Clone)]
pub enum SelectorValue<'a> {
    Some(&'a str),
    Any,
    None,
}

impl<'a> Default for SelectorValue<'a> {
    fn default() -> Self {
        SelectorValue::Any
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum SetSelector {
    SomeSet(DecKey),
    AnySet,
    NoSet,
    Unmatchable,
}

impl Default for SetSelector {
    fn default() -> Self { SetSelector::AnySet }
}



#[derive(Debug,Clone,PartialEq)]
pub enum ClassSelector {
    SomeClass(ClassKey),
    AnyClass,
    NoClass,
    Unmatchable,
}

impl Default for ClassSelector {
    fn default() -> Self { ClassSelector::AnyClass }
}



#[derive(Debug,Clone,PartialEq)]
pub enum TypeSelector {
    SomeElement(ElementType),
    AnyElement,
    AnyType,
    Text,
    Comment,
    Unmatchable,
}

impl Default for TypeSelector {
    fn default() -> Self { TypeSelector::AnyType }
}

///Implements a depth-first search
pub struct SelectIterator<'a> {
    store: &'a ElementStore,
    selector: Selector,
    ///The current stack, containing the element and cursor within that element
    stack: Vec<(ElementKey,usize)>,
    iteration: usize,
}

impl<'a> SelectIterator<'a> {
    pub fn new(store: &'a ElementStore, selector: Selector, key: ElementKey) -> SelectIterator<'a> {
        SelectIterator {
            store: store,
            selector: selector,
            stack: vec![(key,0)],
            iteration: 0,
        }
    }

}

#[derive(Debug)]
pub struct SelectItem<'a> {
    pub data: &'a DataType,
    pub parent_key: ElementKey,
    pub cursor: usize,
    pub depth: usize,
}


impl<'a> Iterator for SelectIterator<'a> {
    type Item = SelectItem<'a>; //Returns the DataTyp, the Parent IntID, the

    fn next(&mut self) -> Option<Self::Item> {
        self.iteration += 1;
        if self.iteration == 1 {
            if !self.selector.matchable() {
                //no need to iterate, selector already knows it is not matchable
                return None;
            }
        }
        if let Some((key,cursor)) = self.stack.pop() {
            if let Some(parent) = self.store.get(key) {
                if let Some(item) = parent.get(cursor) {
                    //increment the cursor and push back to the stack
                    self.stack.push((key, cursor+1));
                    let current_depth = self.stack.len();

                    //we have an element, push to stack so we descend into its on next iteraton
                    if let DataType::Element(key) = item {
                        self.stack.push((*key,0));
                    };

                    //return the current one
                    if self.selector.matches(self.store, item) {
                        Some(SelectItem { data: item, parent_key: key, cursor: cursor, depth: current_depth})
                    } else {
                        self.next() //recurse
                    }
                } else {
                    //child does not exist (cursor out of bounds), no panic, this indicates we are done
                    //with this element and move back up the hierarchy (stack stays popped )

                    self.next() //recurse
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
    fn select(&'a self, key: ElementKey, selector: Selector, recursive: bool) -> SelectIterator<'a>;
}

impl<'a> Select<'a> for ElementStore {
    fn select(&'a self, key: ElementKey, selector: Selector, recursive: bool) -> SelectIterator<'a> {
        SelectIterator::new(self, selector, key)
    }
}
