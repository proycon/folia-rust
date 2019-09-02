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



pub trait Store<T,K> where T: MaybeIdentifiable,
                           K: TryFrom<usize> + Copy + Debug,
                           usize: std::convert::TryFrom<K>,
                           <usize as std::convert::TryFrom<K>>::Error : std::fmt::Debug {
    fn items_mut(&mut self) -> &mut Vec<Option<Box<T>>>;
    fn index_mut(&mut self) -> &mut HashMap<String,K>;

    fn items(&self) -> &Vec<Option<Box<T>>>;
    fn index(&self) -> &HashMap<String,K>;

    fn add(&mut self, mut item: T) -> Result<K,FoliaError> {
        let id = &item.id();
        if let Some(id) = id {
            //check if the ID already exists, if so, we re-use the existing entry and have nothing
            //else to do (the item stays unchanged)
            if let Some(key) = self.index().get(id) {
                return Ok(*key);
            }
        }

        //add the item anew
        let boxed = Box::new(item);
        self.items_mut().push( Some(boxed) );
        if let Ok(key) = K::try_from(self.items().len() - 1) {
            if let Some(id) = id {
                self.index_mut().insert(id.to_string(),key);
            }
            Ok(key)
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
    fn get(&self, key: K) -> Option<&Box<T>> {
        if let Some(key) = self.items().get(usize::try_from(key).expect("conversion to usize")) { //-> Option<&Option<Box<T>>>
            key.as_ref()
        } else {
            None
        }
    }

    ///Retrieves an element from the store
    fn get_mut(&mut self, key: K) -> Option<&mut Box<T>> {
        if let Some(key) = self.items_mut().get_mut(usize::try_from(key).expect("conversion to usize")) { //-> Option<&Option<Box<T>>>
            key.as_mut()
        } else {
            None
        }
    }

    ///Resolve an ID to an Key using the index
    fn key(&self, key: &str) -> Option<K> {
        self.index().get(key).map( |&key| key )
    }

    fn get_by_key(&self, key: &str) -> Option<&Box<T>> {
        self.key(key).map( |key| {
            self.get(key)
        }).map(|o| o.unwrap())
    }

    fn get_mut_by_key(&mut self, key: &str) -> Option<&mut Box<T>> {
        self.key(key).map( move |key| {
            self.get_mut(key)
        }).map(|o| o.unwrap())
    }
}

