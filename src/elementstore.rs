use std::collections::HashMap;

use crate::common::*;
use crate::types::*;
use crate::element::*;
use crate::store::*;

///Holds and owns all elements, the index to them and their declarations. The store serves as an abstraction used by Documents
#[derive(Default)]
pub struct ElementStore {
    items: Vec<Option<Box<FoliaElement>>>, //heap-allocated
    index: HashMap<String,IntId>
}

impl Store<FoliaElement,IntId> for ElementStore {
    fn items_mut(&mut self) -> &mut Vec<Option<Box<FoliaElement>>> {
        &mut self.items
    }
    fn index_mut(&mut self) -> &mut HashMap<String,IntId> {
        &mut self.index
    }

    fn items(&self) -> &Vec<Option<Box<FoliaElement>>> {
        &self.items
    }
    fn index(&self) -> &HashMap<String,IntId> {
        &self.index
    }
}

impl ElementStore {
    ///Adds an element as a child on another, this is a higher-level function that/
    ///takes care of adding and attaching for you.
    pub fn add_to(&mut self, parent_intid: IntId, child: FoliaElement) -> Result<IntId,FoliaError> {
        let child_intid = self.add(child)?;
        self.attach(parent_intid, child_intid);
        child_intid
    }

    ///Adds the child element to the parent element, automatically takes care
    ///of removing the old parent (if any).
    pub fn attach(&mut self, parent_intid: IntId, child_intid: IntId) -> Result<(),FoliaError> {
        //ensure the parent exists
        if !self.get(parent_intid).is_some() {
            return Err(FoliaError::InternalError(format!("Parent does not exist: {}", parent_intid)));
        };

        let oldparent_intid = if let Some(child) = self.get_mut(child_intid) {
            //add the new parent and return the old parent
            let tmp = child.get_parent();
            child.set_parent(Some(parent_intid));
            tmp
        } else {
            //child does not exist
            return Err(FoliaError::InternalError(format!("Child does not exist: {}", child_intid)));
        };

        if let Some(parent) = self.get_mut(parent_intid) {
            parent.push(DataType::Element(child_intid));
        }

        if let Some(oldparent_intid) = oldparent_intid {
            //detach child from the old parent
            if let Some(oldparent) = self.get_mut(oldparent_intid) {
                if let Some(index) = oldparent.index(&DataType::Element(child_intid)) {
                    oldparent.remove(index);
                }
            }
        }
        Ok(())
    }

    ///Removes the child from the parent, orphaning it, does NOT remove the element entirely
    pub fn detach(&mut self, child_intid: IntId) -> Result<(),FoliaError> {
        let oldparent_intid = if let Some(child) = self.get_mut(child_intid) {
            //add the new parent and return the old parent
            let tmp = child.get_parent();
            child.set_parent(None);
            tmp
        } else {
            //child does not exist
            return Err(FoliaError::InternalError(format!("Child does not exist: {}", child_intid)));
        };

        if let Some(oldparent_intid) = oldparent_intid {
            //detach child from the old parent
            if let Some(oldparent) = self.get_mut(oldparent_intid) {
                if let Some(index) = oldparent.index(&DataType::Element(child_intid)) {
                    oldparent.remove(index);
                }
            }
        }
        Ok(())
    }
}

