use std::collections::HashMap;

use crate::common::*;
use crate::element::*;

///Holds and owns all items, the index to them and their declarations. The store serves as an abstraction used by Documents

pub trait MaybeIdentifiable {
    fn id(&self) -> Option<String> {
        None
    }
}

pub trait Store<T> where T: MaybeIdentifiable {
    fn items_mut(&mut self) -> &mut Vec<Option<Box<T>>>;
    fn index_mut(&mut self) -> &mut HashMap<String,IntId>;

    fn items(&self) -> &Vec<Option<Box<T>>>;
    fn index(&self) -> &HashMap<String,IntId>;

    fn add(&mut self, item: T) -> IntId {
        let id = item.id();
        let boxed = Box::new(item);
        self.items_mut().push( Some(boxed) );
        let intid = self.items().len() - 1;
        if let Some(id) = id {
            self.index_mut().insert(id,intid);
        }
        intid
    }

    fn is_empty(&self) -> bool {
        self.items().is_empty()
    }

    fn len(&self) -> usize {
        self.items().len()
    }

    ///Retrieves an element from the store
    fn get(&self, intid: IntId) -> Option<&Box<T>> {
        if let Some(intid) = self.items().get(intid) { //-> Option<&Option<Box<T>>>
            intid.as_ref()
        } else {
            None
        }
    }

    ///Retrieves an element from the store
    fn get_mut(&mut self, intid: IntId) -> Option<&mut Box<T>> {
        if let Some(intid) = self.items_mut().get_mut(intid) { //-> Option<&Option<Box<T>>>
            intid.as_mut()
        } else {
            None
        }
    }

    ///Resolve an ID to an Internal ID using the index
    fn id(&self, id: &str) -> Option<IntId> {
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

