use std::ops::Deref;

use crate::common::*;
use crate::types::*;
use crate::error::*;
use crate::element::*;
use crate::document::*;
use crate::attrib::*;
use crate::metadata::*;
use crate::store::*;
use crate::elementstore::*;



///The selector defines matching criteria for a SelectIterator
#[derive(Debug,Clone,Default)]
pub struct Selector {
    pub typeselector: TypeSelector,
    pub setselector: SetSelector,
    pub classselector: ClassSelector,
    pub next: Option<Box<Selector>>
}


impl Selector {

    ///Creates a new selector given its subcomponents, a type selector, a set selector, and a class
    ///selector. Note that set and class refer to encoded values here. Use ``new_encode()``, if you want
    ///to create a selector with decoded values (strings), which will take care of encoding them
    ///for you.
    pub fn new(typeselector: TypeSelector, setselector: SetSelector, classselector: ClassSelector) -> Self {
        Selector { typeselector: typeselector, setselector: setselector, classselector: classselector, next: None }
    }


    ///Creates a new selector given its subcomponents, a type selector, a set selector, and a class
    ///selector. This variant actively encodes the set and class you specify.
    pub fn new_encode(document: &Document, elementtype: ElementType, set: SelectorValue, class: SelectorValue) -> Self {
        Selector::default().encode(document, elementtype, set, class)
    }


    ///Encodes a selector
    pub fn encode(mut self, document: &Document, elementtype: ElementType, set: SelectorValue, class: SelectorValue) -> Self {
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
                            if let Some(classes) = &declaration.classes {
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

    ///Add another selector, the resulting selection will then consist of the union
    ///of the selectors. This can be chained multiple times.
    pub fn and(mut self, selector: Selector) -> Self {
        self.next = Some(Box::new(selector));
        self
    }


    ///The selector determines whether it is matchable in the encoding stage, when references are
    ///made to sets or classes that don't exist in the document, then it is unmatchable and there
    ///is no sense in actually performing any matching.
    pub fn matchable(&self) -> bool {
        self.typeselector != TypeSelector::Unmatchable &&
        self.setselector != SetSelector::Unmatchable &&
        self.classselector != ClassSelector::Unmatchable
    }

    ///Tests if the selector matches against the specified data item, given an element store.
    ///There is no need to invoke this directly if you use a ``SelectIterator``.
    pub fn matches(&self, store: &ElementStore, item: &DataType) -> bool {
        //we attempt to falsify the match
        let matches = match item {
            DataType::Element(key) => {
                if let TypeSelector::Text | TypeSelector::Comment  = self.typeselector {
                    false
                } else if let Some(element) = store.get(*key) {
                    let typematch: bool = match &self.typeselector {
                        TypeSelector::SomeElement(refelementtype) => {
                            element.elementtype == *refelementtype
                        },
                        TypeSelector::SomeElementGroup(elementgroup) => {
                            elementgroup.contains(element.elementtype)
                        },
                        TypeSelector::AnyElement => true,
                        TypeSelector::AnyType => true,
                        TypeSelector::Unmatchable => false,
                        TypeSelector::Comment => false,
                        TypeSelector::Text => false,
                    };
                    if typematch {
                        let setmatch: bool = match &self.setselector {
                             SetSelector::SomeSet(refset) => {
                                 if let Some(set) = element.declaration_key() {
                                     set == *refset
                                 } else {
                                     false
                                 }
                             },
                             SetSelector::NoSet => element.declaration_key().is_none(),
                             SetSelector::AnySet => true,
                             SetSelector::Unmatchable => false,
                        };
                        if setmatch {
                            let classmatch: bool = match &self.classselector {
                                ClassSelector::SomeClass(refclass) => {
                                    if let Some(class) = element.class_key() {
                                        class == *refclass
                                    } else {
                                        false
                                    }
                                },
                                ClassSelector::NoClass => element.class_key().is_none(),
                                ClassSelector::AnyClass => true,
                                ClassSelector::Unmatchable => false,
                            };
                            classmatch
                        } else {
                            false
                        }
                    } else {
                        false
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


///Represents a selector value prior to encoding; such as a particular FoLiA set or FoLiA class as
///a string. This SelectorValue can then be passed when creating selectors with ``with()`` or ``new_with()``, which
///will take care of encoding the set/class.
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

///Specifies what set to select (where the set is already encoded as a key)
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



///Specifies what class to select (where the class is already encoded as a key)
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
    SomeElementGroup(ElementGroup),
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

///Iterator over data items (elements, text, comments, i.e. a ``DataType``).
///This implements a depth-first search.
pub struct SelectIterator<'a> {
    ///The element store to draw elements from
    pub store: &'a ElementStore<'a>,
    ///The selector to apply to test for matching data items
    pub selector: Selector,
    ///Apply the selector recursively (depth-first search) or not (plain linear search)
    pub recursive: bool,

    ///The current stack, containing the element and cursor within that element
    pub(crate) stack: Vec<(ElementKey,usize)>,
    pub(crate) iteration: usize,
}

impl<'a> SelectIterator<'a> {
    ///Creates a new ``SelectIterator``. This is usually not invoked directly but through a
    ///``selects()`` method (provided by the ``Select`` trait) which is implement by for instance a ``Document`` or an ``ElementStore``.
    pub fn new(store: &'a ElementStore, selector: Selector, key: ElementKey, recursive: bool) -> SelectIterator<'a> {
        SelectIterator {
            store: store,
            selector: selector,
            recursive: recursive,
            stack: vec![(key,0)],
            iteration: 0,
        }
    }

    ///Returns the selector component of the iterator
    pub fn selector(&self) -> &Selector {
        &self.selector
    }

}

///The ``Item`` returned by a ``SelectIterator``. It dereferences into ``&DataType``>
#[derive(Debug)]
pub struct SelectItem<'a> {
    pub data: &'a DataType,
    pub parent_key: ElementKey,
    pub cursor: usize,
    pub depth: usize,
}

impl<'a> Deref for SelectItem<'a> {
    type Target = DataType;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}


impl<'a> Iterator for SelectIterator<'a> {
    type Item = SelectItem<'a>; //Returns the DataType, the Parent IntID, the cursor and the depth

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
                    if self.recursive {
                        if let DataType::Element(key) = item {
                            self.stack.push((*key,0));
                        };
                    }

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

///This trait is for collections for which a ``SelectIterator`` can be created to iterate over data
///items contained in it.
pub trait Select<'a> {
    fn select(&'a self, key: ElementKey, selector: Selector, recursive: bool) -> SelectIterator<'a>;
}


impl<'a> Select<'a> for ElementStore<'a> {
    ///Returns a ``SelectIterator`` that can be used to iterate over data items under the element specified by
    ///``key``.
    fn select(&'a self, key: ElementKey, selector: Selector, recursive: bool) -> SelectIterator<'a> {
        SelectIterator::new(self, selector, key, recursive)
    }
}

impl<'a> Select<'a> for Document<'a> {
    ///Returns a ``SelectIterator`` that can be used to iterate over data items under the element
    ///specified by ``key``. The ``SelectIterator`` implements a depth-first-search (if recursion
    ///is enabled). This is the primary means of iterating over anything in the document.
    fn select(&'a self, key: ElementKey, selector: Selector, recursive: bool) -> SelectIterator<'a> {
        SelectIterator::new(&self.elementstore, selector, key, recursive)
    }
}

///This is a higher-level iterator that iterates over elements only (i.e. not over text, comments,
///etc). It is implemented as a wrapper around ``SelectIterator`` and is identical in many regards. However, this iterator returns
///``SelectElementsItem``, which dereferences directly to ``&FoliaElement``.
pub struct SelectElementsIterator<'a> {
    iterator: SelectIterator<'a>
}

impl<'a> SelectElementsIterator<'a> {
    pub fn new(store: &'a ElementStore, selector: Selector, key: ElementKey, recursive: bool) -> SelectElementsIterator<'a> {
        SelectElementsIterator {
            iterator: SelectIterator::new(&store, selector, key, recursive)
        }
    }

    pub fn selector(&self) -> &Selector {
        &self.iterator.selector
    }

}

///The Item returned by SelectElementsIterator, this dereferences directly to ``&FoliaElement``
pub struct SelectElementsItem<'a> {
    pub element: &'a FoliaElement<'a>,
}

impl<'a> Deref for SelectElementsItem<'a> {
    type Target = FoliaElement<'a>;

    fn deref(&self) -> &Self::Target {
        self.element
    }
}


impl<'a> Iterator for SelectElementsIterator<'a> {
    type Item = SelectElementsItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let selectitem = self.iterator.next();
        if let Some(selectitem) = selectitem {
            match *selectitem {
                DataType::Element(key) => {
                    let element = self.iterator.store.get(key).expect("Getting key from elementstore for SelectElementsIterator");
                    Some(Self::Item { element: &**element })
                },
                _ => {
                    //recurse
                    self.next()
                }
            }
        } else {
            None
        }
    }

}

///This trait is for collections for which a ``SelectElementsIterator`` can be created to iterate over data
///items contained in it.
pub trait SelectElements<'a> {
    fn select_elements(&'a self, key: ElementKey, selector: Selector, recursive: bool) -> SelectElementsIterator<'a>;
}

impl<'a> SelectElements<'a> for ElementStore<'a> {
    ///Returns a ``SelectElementsIterator`` that can be used to iterate over elements under the element
    ///specified by ``key``. The ``SelectElementsIterator`` implements a depth-first-search (if recursion
    ///is enabled).
    fn select_elements(&'a self, key: ElementKey, selector: Selector, recursive: bool) -> SelectElementsIterator<'a> {
        SelectElementsIterator::new(&self, selector, key, recursive)
    }
}

impl<'a> SelectElements<'a> for Document<'a> {
    ///Returns a ``SelectElementsIterator`` that can be used to iterate over elements under the element
    ///specified by ``key``. The ``SelectElementsIterator`` implements a depth-first-search (if recursion
    ///is enabled).
    fn select_elements(&'a self, key: ElementKey, selector: Selector, recursive: bool) -> SelectElementsIterator<'a> {
        SelectElementsIterator::new(&self.elementstore, selector, key, recursive)
    }
}
