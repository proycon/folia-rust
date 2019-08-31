use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Debug;

use crate::common::*;
use crate::types::*;
use crate::error::*;
use crate::element::*;
use crate::document::*;

///Holds and owns all items, the index to them and their declarations. The store serves as an abstraction used by Documents

pub trait MaybeIdentifiable {
    fn id(&self) -> Option<String> {
        None
    }
}




pub trait Store<T,I> where T: MaybeIdentifiable, I: TryFrom<usize> + Into<usize> + Debug {
    fn items_mut(&mut self) -> &mut Vec<Option<Box<T>>>;
    fn index_mut(&mut self) -> &mut HashMap<String,I>;

    fn items(&self) -> &Vec<Option<Box<T>>>;
    fn index(&self) -> &HashMap<String,I>;

    fn add(&mut self, mut item: T) -> Result<I,FoliaError> {
        let id = item.id();
        if let Some(id) = id {
            //check if the ID already exists, if so, we re-use the existing entry and have nothing
            //else to do (the item stays unchanged)
            if let Some(intid) = self.index().get(&id) {
                return Ok(*intid);
            }
        }

        //add the item anew
        let boxed = Box::new(item);
        self.items_mut().push( Some(boxed) );
        if let Ok(intid) = I::try_from(self.items().len() - 1) {
            if let Some(id) = id {
                self.index_mut().insert(id,intid);
            }
            Ok(intid)
        } else {
            Err(FoliaError::InternalError(format!("Store.add(). Index out of bounds (e.g. integer overflow)")))
        }
    }

    fn is_empty(&self) -> bool {
        self.items().is_empty()
    }

    fn len(&self) -> usize {
        self.items().len()
    }

    ///Retrieves an element from the store
    fn get(&self, intid: I) -> Option<&Box<T>> {
        if let Some(intid) = self.items().get(intid.into()) { //-> Option<&Option<Box<T>>>
            intid.as_ref()
        } else {
            None
        }
    }

    ///Retrieves an element from the store
    fn get_mut(&mut self, intid: I) -> Option<&mut Box<T>> {
        if let Some(intid) = self.items_mut().get_mut(intid.into()) { //-> Option<&Option<Box<T>>>
            intid.as_mut()
        } else {
            None
        }
    }

    ///Resolve an ID to an Internal ID using the index
    fn id(&self, id: &str) -> Option<I> {
        self.index().get(id).map( |&intid| intid )
    }

    fn get_by_id(&self, id: &str) -> Option<&Box<T>> {
        self.id(id).map( |intid| {
            self.get(intid)
        }).map(|o| o.unwrap())
    }

    fn get_mut_by_id(&mut self, id: &str) -> Option<&mut Box<T>> {
        self.id(id).map( move |intid| {
            self.get_mut(intid)
        }).map(|o| o.unwrap())
    }
}

