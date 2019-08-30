use std::collections::HashMap;

use crate::common::*;
use crate::element::*;
use crate::store::*;

pub type ProcessorIntId = usize;

pub struct Declaration {
    pub annotationtype: AnnotationType,
    pub set: Option<String>,
    pub processors: Vec<ProcessorIntId>
}

impl MaybeIdentifiable for Declaration {
    fn id(&self) -> Option<String> {
        if let Some(set) = &self.set {
            Some(format!("{}/{}", self.annotationtype, set))
        } else {
            Some(format!("{}", self.annotationtype))
        }
    }
}

#[derive(Default)]
pub struct DeclarationStore {
    items: Vec<Option<Box<Declaration>>>, //heap-allocated
    index: HashMap<String,IntId>
}

impl Store<Declaration> for DeclarationStore {
    fn items_mut(&mut self) -> &mut Vec<Option<Box<Declaration>>> {
        &mut self.items
    }
    fn index_mut(&mut self) -> &mut HashMap<String,IntId> {
        &mut self.index
    }

    fn items(&self) -> &Vec<Option<Box<Declaration>>> {
        &self.items
    }
    fn index(&self) -> &HashMap<String,IntId> {
        &self.index
    }
}

pub struct ProvenanceStore {
    pub data: Vec<Processor>,
    pub chain: Vec<ProcessorIntId>,
    pub index: HashMap<String,ProcessorIntId>
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub enum ProcessorType {
    Auto,
    Manual,
    Generator,
    DataSource,
}

pub struct Processor {
    pub id: String,
    pub processortype: ProcessorType,
    pub version: String,
    pub folia_version: String,
    pub document_version: String,
    pub command: String,
    pub host: String,
    pub user: String,
    pub begindatetime: String,
    pub enddatetime: String,
    pub processors: Vec<ProcessorIntId>,
    pub src: String,
    pub format: String,
    pub resourcelink: String,
    pub parent: Option<ProcessorIntId>,
    pub metadata: Metadata,
}

pub struct Metadata {
    pub data: HashMap<String,String>
}
