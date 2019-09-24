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
use crate::query::*;

///The selector defines matching criteria for a SelectIterator
///It is constructed from a ``Query`` given a document, i.e. it is the encoded
///form of a query.
#[derive(Default,Clone)]
pub struct Selector {
    pub action: Action,
    pub datatypes: Vec<DataTypeSelector>, //if left empty, matches anything
    pub elementtype: Cmp<ElementType>,
    pub elementgroup: Cmp<ElementGroup>,
    pub contexttype: Cmp<ElementType>, //only used for features
    pub set: Cmp<DecKey>,
    pub class: Cmp<ClassKey>,
    pub processor: Cmp<ProcKey>,
    pub subset: Cmp<SubsetKey>,
    pub confidence: Cmp<f64>,
    pub next: Option<Box<Selector>>
}




impl Selector {
    ///Builds a new selector given a query and a document (effectively encoding the query into a
    ///selector for the specified document)
    pub fn from_query(document: &Document, query: &Query) -> Result<Self,FoliaError> {
        let mut selector = Selector::default();
        selector.elementtype = query.elementtype.clone();
        selector.elementgroup = query.elementgroup.clone();
        selector.contexttype = query.contexttype.clone();
        selector.datatypes = vec![DataTypeSelector::Elements];
        //if we have subsets, we use contexttype instead of elementtype (because elementtype will
        //always be ElementType::feature)
        let elementtype_source: &Cmp<ElementType> = match &query.subset {
            Cmp::Some | Cmp::Is(_) =>  &query.contexttype,
            _ => &query.elementtype,
        };
        selector.set = match &query.set {
            Cmp::Is(set) => {
                //encode the set from the query, given the document, if this fails then the set is
                //unmatchable
                let mut result: Cmp<DecKey> = Cmp::Unmatchable; //will try to falsify this
                if let Cmp::Is(elementtype) = elementtype_source {
                    if let Some(annotationtype) = elementtype.annotationtype() {
                        if let Some(deckey) = document.get_declaration_key_by_id(&Declaration::index_id(annotationtype,&Some(set.as_str()))) {
                            result = Cmp::Is(deckey);
                        }
                    }
                }
                result
            },
            Cmp::Any => Cmp::Any,
            Cmp::Some => Cmp::Some,
            Cmp::None => {
                //even though set is None, we obtain the associated declaration
                let mut result: Cmp<DecKey> = Cmp::Unmatchable; //will try to falsify this
                if let Cmp::Is(elementtype) = elementtype_source {
                    if let Some(annotationtype) = elementtype.annotationtype() {
                        if let Some(deckey) = document.get_declaration_key_by_id(&Declaration::index_id(annotationtype,&None)) {
                            result = Cmp::Is(deckey);
                        }
                    }
                }
                result
            },
            Cmp::Unmatchable => Cmp::Unmatchable,
        };
        //println!("{:?} -> {:?}",query.set,selector.set); //DEBUG
        selector.subset = match &query.subset {
            Cmp::Is(subset) => {
                let mut result: Cmp<SubsetKey> = Cmp::Unmatchable; //will try to falsify this
                if let Cmp::Is(deckey) = selector.set {
                    if let Some(declaration) = document.get_declaration(deckey) {
                        if let Some(subset_key) = declaration.subset_key(subset.as_str()) {
                            result = Cmp::Is(subset_key);
                        }
                    }
                } else {
                    return Err(FoliaError::QueryError(format!("Selector::from_query() can't match on a subset without a contexttype and a set, Add a .contextype() and .set() call. (selector.contexttype={:?}, selector.set={:?})",selector.contexttype, selector.set) ));
                }
                result
            },
            Cmp::Any => Cmp::Any,
            Cmp::Some => Cmp::Some,
            Cmp::None => Cmp::None,
            Cmp::Unmatchable => Cmp::Unmatchable,
        };
        selector.class = match &query.class {
            Cmp::Is(class) => {
                let mut result: Cmp<ClassKey> = Cmp::Unmatchable; //will try to falsify this
                if let Cmp::Is(deckey) = selector.set {
                    if let Some(declaration) = document.get_declaration(deckey) {
                        match selector.subset {
                            Cmp::Is(_) | Cmp::Some =>  {
                                //we have a subset, so we assume the class is a subclass and encode it as such
                                if let Some(class_key) = declaration.subclass_key(class.as_str()) {
                                    result = Cmp::Is(class_key);
                                }
                            },
                            _ =>  {
                                //normal class
                                if let Some(class_key) = declaration.class_key(class.as_str()) {
                                    result = Cmp::Is(class_key);
                                }
                            }
                        }
                    }
                }
                result
            },
            Cmp::Any => Cmp::Any,
            Cmp::Some => Cmp::Some,
            Cmp::None => Cmp::None,
            Cmp::Unmatchable => Cmp::Unmatchable,
        };
        selector.processor = match &query.processor {
            Cmp::Is(processor_id) => {
                let mut result: Cmp<ProcKey> = Cmp::Unmatchable; //will try to falsify this
                if let Some(processor_key) = document.get_processor_key_by_id(processor_id.as_str()) {
                    result = Cmp::Is(processor_key);
                }
                result
            },
            Cmp::Any => Cmp::Any,
            Cmp::Some => Cmp::Some,
            Cmp::None => Cmp::None,
            Cmp::Unmatchable => Cmp::Unmatchable,
        };
        Ok(selector)
    }

    ///Sets the selector to also yield Folia Elements (you usually don't need this as it's the
    ///default or already implied in Selector construction)
    pub fn with_elements(mut self) -> Self {
        self.datatypes.push(DataTypeSelector::Elements);
        self
    }

    ///Sets the selector to also yield XML text
    pub fn with_text(mut self) -> Self {
        self.datatypes.push(DataTypeSelector::Text);
        self
    }

    ///Sets the selector to also yield XML comments
    pub fn with_comments(mut self) -> Self {
        self.datatypes.push(DataTypeSelector::Comments);
        self
    }

    ///Constrains the selector by element type
    pub fn element(mut self, value: Cmp<ElementType>) -> Self {
        self.elementtype = value;
        self
    }

    ///Creates a selector on elements
    pub fn elements() -> Self {
        let mut selector = Selector::default();
        selector.datatypes = vec![DataTypeSelector::Elements];
        selector
    }

    ///Creates a selector on all data (alias for Selector::default())
    pub fn all_data() -> Self {
        Selector::default()
    }

    ///Constrains the selector by element group
    pub fn elementgroup(mut self, value: Cmp<ElementGroup>) -> Self {
        self.elementgroup = value;
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
        self.set != Cmp::Unmatchable &&
        self.class != Cmp::Unmatchable &&
        self.processor != Cmp::Unmatchable
    }

    ///Tests if the selector matches against the specified data item, given an element store.
    ///There is no need to invoke this directly if you use a ``SelectIterator``.
    pub fn matches(&self, document: &Document, item: &DataType) -> bool {
        //we attempt to falsify the match
        let matches = match item {
            DataType::Element(key) => {
                if !self.datatypes.is_empty() && !self.datatypes.contains(&DataTypeSelector::Elements) {
                    false
                } else if let Some(element) = document.get_element(*key) {
                    let mut matches = match self.elementgroup {
                        Cmp::Is(elementgroup) => elementgroup.contains(element.elementtype()),
                        Cmp::Any | Cmp::Some => true,
                        Cmp::None | Cmp::Unmatchable => false,
                    };
                    matches &&
                    self.elementtype.matches(Some(&element.elementtype())) &&
                    self.set.matches(element.declaration_key().as_ref()) &&
                    self.subset.matches(element.subset_key().as_ref()) &&
                    self.class.matches(element.class_key().as_ref()) &&
                    self.processor.matches(element.processor_key().as_ref())
                } else {
                    //element does not exist, can never match
                    false
                }
            },
            DataType::Text(_) => self.datatypes.contains(&DataTypeSelector::Text),
            DataType::Comment(_) => self.datatypes.contains(&DataTypeSelector::Comments)
        };
        if let Some(next) = &self.next {
            matches || next.matches(document, item)
        } else {
            matches
        }
    }
}




#[derive(Debug,Clone,PartialEq)]
pub enum DataTypeSelector {
    Elements,
    Text,
    Comments,
}


///Iterator over data items (elements, text, comments, i.e. a ``DataType``).
///This implements a depth-first search.
pub struct SelectIterator<'a> {
    ///The element store to draw elements from
    pub document: &'a Document,
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
    pub fn new(document: &'a Document, selector: Selector, key: ElementKey, recursive: bool) -> SelectIterator<'a> {
        SelectIterator {
            document: document,
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
            if let Some(parent) = self.document.get_elementdata(key) {
                if let Some(item) = parent.get_data_at(cursor) {
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
                    if self.selector.matches(self.document, item) {
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
pub trait SelectData<'a> {
    fn select_data_by_key(&'a self, key: ElementKey, selector: Selector, recursive: bool) -> SelectIterator<'a>;
    fn select_data(&'a self, selector: Selector, recursive: bool) -> SelectIterator<'a>;
}



impl<'a> SelectData<'a> for Document {
    ///Returns a ``SelectIterator`` that can be used to iterate over data items under the element
    ///specified by ``key``. The ``SelectIterator`` implements a depth-first-search (if recursion
    ///is enabled). This is the primary means of iterating over anything in the document.
    fn select_data_by_key(&'a self, key: ElementKey, selector: Selector, recursive: bool) -> SelectIterator<'a> {
        SelectIterator::new(&self, selector, key, recursive)
    }

    ///Returns a ``SelectIterator`` that can be used to iterate over data items under the element
    ///specified by ``key``. The ``SelectIterator`` implements a depth-first-search (if recursion
    ///is enabled). This is the primary means of iterating over anything in the document.
    fn select_data(&'a self, selector: Selector, recursive: bool) -> SelectIterator<'a> {
        SelectIterator::new(&self, selector, 0, recursive)
    }
}

impl<'a> SelectData<'a> for Element<'a> {
    ///Returns a ``SelectIterator`` that can be used to iterate over data items under the element
    ///specified by ``key``. The ``SelectIterator`` implements a depth-first-search (if recursion
    ///is enabled). This is the primary means of iterating over anything in the document.
    fn select_data_by_key(&'a self, key: ElementKey, selector: Selector, recursive: bool) -> SelectIterator<'a> {
        SelectIterator::new(self.document().expect("Obtaining document for element (will fail on orphans!)"), selector, key, recursive)
    }

    ///Returns a ``SelectIterator`` that can be used to iterate over data items under the element
    ///specified by ``key``. The ``SelectIterator`` implements a depth-first-search (if recursion
    ///is enabled). This is the primary means of iterating over anything in the document.
    fn select_data(&'a self, selector: Selector, recursive: bool) -> SelectIterator<'a> {
        SelectIterator::new(self.document().expect("Obtaining document for element (will fail on orphans!)"), selector, self.key().expect("Obtaining key for element (will fail on oprhans)"), recursive)
    }
}

///This is a higher-level iterator that iterates over elements only (i.e. not over text, comments,
///etc). It is implemented as a wrapper around ``SelectIterator`` and is identical in many regards. However, this iterator returns
///``SelectElementsItem``, which dereferences directly to ``&ElementData``.
pub struct SelectElementsIterator<'a> {
    iterator: SelectIterator<'a>
}

impl<'a> SelectElementsIterator<'a> {
    pub fn new(document: &'a Document, selector: Selector, key: ElementKey, recursive: bool) -> SelectElementsIterator<'a> {
        SelectElementsIterator {
            iterator: SelectIterator::new(document, selector, key, recursive)
        }
    }

    pub fn selector(&self) -> &Selector {
        &self.iterator.selector
    }

}

///The Item returned by SelectElementsIterator, this dereferences directly to ``&ElementData``
pub struct SelectElementsItem<'a> {
    pub element: Element<'a>,
}

impl<'a> Deref for SelectElementsItem<'a> {
    type Target = Element<'a>;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}


impl<'a> Iterator for SelectElementsIterator<'a> {
    type Item = SelectElementsItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let selectitem = self.iterator.next();
        if let Some(selectitem) = selectitem {
            match *selectitem {
                DataType::Element(key) => {
                    let element = self.iterator.document.get_element(key).expect("Getting key from elementstore for SelectElementsIterator");
                    Some(Self::Item { element: element })
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
    fn select_by_key(&'a self, key: ElementKey, selector: Selector, recursive: bool) -> SelectElementsIterator<'a>;
    fn select(&'a self, selector: Selector, recursive: bool) -> SelectElementsIterator<'a>;
}

impl<'a> SelectElements<'a> for Document {
    ///Returns a ``SelectElementsIterator`` that can be used to iterate over elements under the element
    ///specified by ``key``. The ``SelectElementsIterator`` implements a depth-first-search (if recursion
    ///is enabled).
    fn select(&'a self, selector: Selector, recursive: bool) -> SelectElementsIterator<'a> {
        SelectElementsIterator::new(&self, selector, 0, recursive)
    }

    ///Returns a ``SelectElementsIterator`` that can be used to iterate over elements under the element
    ///specified by ``key``. The ``SelectElementsIterator`` implements a depth-first-search (if recursion
    ///is enabled).
    fn select_by_key(&'a self, key: ElementKey, selector: Selector, recursive: bool) -> SelectElementsIterator<'a> {
        SelectElementsIterator::new(&self, selector, key, recursive)
    }
}

impl<'a> SelectElements<'a> for Element<'a> {
    ///Returns a ``SelectElementsIterator`` that can be used to iterate over elements under the element
    ///specified by ``key``. The ``SelectElementsIterator`` implements a depth-first-search (if recursion
    ///is enabled).
    fn select(&'a self, selector: Selector, recursive: bool) -> SelectElementsIterator<'a> {
        SelectElementsIterator::new(self.document().expect("Obtaining document for element (will fail on orphans!)"), selector, self.key().expect("Obtaining key for element (will fail on orphans)"), recursive)
    }

    ///Returns a ``SelectElementsIterator`` that can be used to iterate over elements under the element
    ///specified by ``key``. The ``SelectElementsIterator`` implements a depth-first-search (if recursion
    ///is enabled).
    fn select_by_key(&'a self, key: ElementKey, selector: Selector, recursive: bool) -> SelectElementsIterator<'a> {
        SelectElementsIterator::new(self.document().expect("Obtaining document for element (will fail on orphans!)"), selector, key, recursive)
    }

}

