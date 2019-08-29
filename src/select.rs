use crate::common::*;
use crate::error::*;
use crate::element::*;
use crate::attrib::*;
use crate::elementstore::*;

#[derive(Debug,Clone)]
pub struct Selector<'a> {
    pub elementtype: TypeSelector,
    pub set: SetSelector<'a>,
    pub recursive: bool,
}

#[derive(Debug,Copy,Clone)]
pub enum SetSelector<'a> {
    SomeSet(&'a str),
    AnySet,
    NoSet
}

#[derive(Debug,Clone)]
pub enum TypeSelector {
    SomeType(ElementType),
    MultiType(Vec<ElementType>),
    AnyType,
}

pub struct SelectIterator<'a> {
    store: &'a ElementStore,
    selector: Selector<'a>,
    element: &'a FoliaElement,
    ///The current stack, containing the element and cursor within that element
    stack: Vec<(IntId,usize)>,
}

impl<'a> Iterator for SelectIterator<'a> {
    type Item = &'a DataType;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((intid,cursor)) = self.stack.last().map(|(x,y)| (*x,*y)) {
            if let Some(element) = self.store.get(intid) {
                let item = element.get(cursor);
                //compute the next element
                match item {
                    Some(DataType::Element(intid)) => {
                        self.stack.push((*intid,0));
                    },
                    Some(x) => {
                        //temporarily pop the last element of the stack
                        self.stack.pop();
                        //increment the cursor
                        let cursor = cursor + 1;
                        //push it back if the cursor does not exceed the length
                        if cursor < element.len() {
                            self.stack.push((intid, cursor));
                        }
                    }
                    None => {
                        unreachable!();
                    }
                };
                //return the current one
                item
            } else {
                unreachable!();
            }
        } else {
            None
        }
    }

}

pub trait Select {
    fn select(&self, selector: Selector) -> SelectIterator;
}
