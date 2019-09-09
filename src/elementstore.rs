use std::collections::HashMap;

use crate::common::*;
use crate::types::*;
use crate::error::*;
use crate::element::*;
use crate::store::*;

///Holds and owns all elements, the index to them and their declarations. The store serves as an abstraction used by Documents
#[derive(Default)]
pub struct ElementStore {
    items: Vec<Option<Box<FoliaElement>>>, //heap-allocated
    index: HashMap<String,ElementKey>
}

impl Store<FoliaElement,ElementKey> for ElementStore {
    fn items_mut(&mut self) -> &mut Vec<Option<Box<FoliaElement>>> {
        &mut self.items
    }
    fn index_mut(&mut self) -> &mut HashMap<String,ElementKey> {
        &mut self.index
    }

    fn items(&self) -> &Vec<Option<Box<FoliaElement>>> {
        &self.items
    }
    fn index(&self) -> &HashMap<String,ElementKey> {
        &self.index
    }

}

impl ElementStore {
    ///Adds an element as a child of another, this is a higher-level function that/
    ///takes care of adding and attaching for you.
    pub fn add_to(&mut self, parent_key: ElementKey, child: FoliaElement) -> Result<ElementKey,FoliaError> {
        let child_key = self.add(child);
        if let Ok(child_key) = child_key {
            self.attach(parent_key, child_key)?;
        }
        child_key
    }

    ///Adds the child element to the parent element, automatically takes care
    ///of removing the old parent (if any).
    pub fn attach(&mut self, parent_key: ElementKey, child_key: ElementKey) -> Result<(),FoliaError> {
        //ensure the parent exists
        if !self.get(parent_key).is_some() {
            return Err(FoliaError::InternalError(format!("Parent does not exist: {}", parent_key)));
        };

        let oldparent_key = if let Some(child) = self.get_mut(child_key) {
            //add the new parent and return the old parent
            let tmp = child.get_parent();
            child.set_parent(Some(parent_key));
            tmp
        } else {
            //child does not exist
            return Err(FoliaError::InternalError(format!("Child does not exist: {}", child_key)));
        };

        if let Some(parent) = self.get_mut(parent_key) {
            parent.push(DataType::Element(child_key));
        }

        if let Some(oldparent_key) = oldparent_key {
            //detach child from the old parent
            if let Some(oldparent) = self.get_mut(oldparent_key) {
                if let Some(index) = oldparent.index(&DataType::Element(child_key)) {
                    oldparent.remove(index);
                }
            }
        }
        Ok(())
    }

    ///Removes the child from the parent, orphaning it, does NOT remove the element entirely
    pub fn detach(&mut self, child_key: ElementKey) -> Result<(),FoliaError> {
        let oldparent_key = if let Some(child) = self.get_mut(child_key) {
            //add the new parent and return the old parent
            let tmp = child.get_parent();
            child.set_parent(None);
            tmp
        } else {
            //child does not exist
            return Err(FoliaError::InternalError(format!("Child does not exist: {}", child_key)));
        };

        if let Some(oldparent_key) = oldparent_key {
            //detach child from the old parent
            if let Some(oldparent) = self.get_mut(oldparent_key) {
                if let Some(index) = oldparent.index(&DataType::Element(child_key)) {
                    oldparent.remove(index);
                }
            }
        }
        Ok(())
    }
}

