use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::borrow::{Cow,ToOwned,Borrow};
use std::ops::Deref;

use crate::common::*;
use crate::types::*;
use crate::error::*;
use crate::element::*;
use crate::document::*;


///This trait needs to be implemented on  items that are storable in a ``Store``. It is a very lax trait where storable elements *MAY BE* identifiable and *MAY BE* storing their own key (the default implementation does neither)
pub trait Storable<K> {
    fn maybe_id(&self) -> Option<Cow<str>> {
        None
    }

    fn is_encoded(&self) -> bool {
        true
    }

    ///Get the key of the current item (if supported by the item)
    fn key(&self) -> Option<K> {
        None
    }

    ///Set the key of the current item (if supported by the item)
    fn set_key(&mut self, key: K) {
        //does nothing by default, override in implementations
    }
}


///Holds and owns all items, the index to them and their declarations. The store serves as an abstraction used by Documents
pub trait Store<T,K> where T: Storable<K>,
                           K: TryFrom<usize> + Copy + Debug,
                           usize: std::convert::TryFrom<K>,
                           <usize as std::convert::TryFrom<K>>::Error : std::fmt::Debug {

    fn items_mut(&mut self) -> &mut Vec<Option<Box<T>>>;
    fn index_mut(&mut self) -> &mut HashMap<String,K>;

    fn items(&self) -> &Vec<Option<Box<T>>>;
    fn iter(&self) -> std::slice::Iter<Option<Box<T>>>;
    fn index(&self) -> &HashMap<String,K>;

    ///Add a new item to the store (takes ownership)
    fn add(&mut self, item: T) -> Result<K,FoliaError> {
        if !item.is_encoded() {
            return Err(FoliaError::InternalError(format!("Store.add(). Item is not properly encoded")));
        }
        if let Some(key) = self.get_key(&item) {
            return Ok(key);
        }

        //Get the ID fo the item (if any)
        let id: Option<String> = item.maybe_id().map(|x| x.to_owned().to_string());

        //add the item anew
        let mut boxed = Box::new(item);
        if let Ok(key) = K::try_from(self.items().len()) {
            boxed.set_key(key); //set the key so the item knows it's own key (if supported)
            self.items_mut().push( Some(boxed) );
            if let Some(id) = id {
                self.index_mut().insert(id,key);
            }
            Ok(key)
        } else {
            Err(FoliaError::InternalError(format!("Store.add(). Index out of bounds (e.g. integer overflow)")))
        }
    }

    ///Checks if an item is already in the store and returns the key if so, only works for
    ///identifiable items (so it's guaranteed O(1))
    fn get_key(&self, item: &T) -> Option<K> {
        let id: &Option<Cow<str>> = &item.maybe_id();
        if let Some(id) = id.as_ref() {
            let id: &str = id; //coerce the Cow<str> into &str
            self.index().get(id).map(|k| k.to_owned())
        } else {
            //not identifiable
            None
        }
    }

    fn is_empty(&self) -> bool {
        self.items().is_empty()
    }

    ///Returns the number of items in the store (including items that were removed)
    fn len(&self) -> usize {
        self.items().len()
    }

    ///Retrieves an element from the store
    fn get(&self, key: K) -> Option<&Box<T>> {
        if let Some(item) = self.items().get(usize::try_from(key).expect("conversion to usize")) { //-> Option<&Option<Box<T>>>
            item.as_ref()
        } else {
            None
        }
    }

    ///Retrieves an element from the store
    fn get_mut(&mut self, key: K) -> Option<&mut Box<T>> {
        if let Some(item) = self.items_mut().get_mut(usize::try_from(key).expect("conversion to usize")) { //-> Option<&Option<Box<T>>>
            item.as_mut()
        } else {
            None
        }
    }

    ///Resolve an ID to a Key using the index
    fn id_to_key(&self, id: &str) -> Option<K> {
        self.index().get(id).map( |&key| key )
    }

    ///Get by key, where key is still a string to be resolved. Shortcut function calling key() and
    ///get()
    fn get_by_id(&self, id: &str) -> Option<&Box<T>> {
        self.id_to_key(id).map( |key| {
            self.get(key)
        }).map(|o| o.unwrap())
    }

    ///Get (mutably) by key, where key is still a string to be resolved. Shortcut function calling
    ///key() and get_mut()
    fn get_mut_by_id(&mut self, id: &str) -> Option<&mut Box<T>> {
        self.id_to_key(id).map( move |key| {
            self.get_mut(key)
        }).map(|o| o.unwrap())
    }
}

