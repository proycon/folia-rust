use std::collections::HashMap;

use crate::common::*;
use crate::element::*;
use crate::store::*;

pub type ProcIntId = u16;
pub type DecIntId = u16;

pub struct Declaration {
    pub annotationtype: AnnotationType,
    pub set: Option<String>,
    pub processors: Vec<ProcIntId>
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
    index: HashMap<String,ProcIntId>
}

impl Store<Declaration,DecIntId> for DeclarationStore {
    fn items_mut(&mut self) -> &mut Vec<Option<Box<Declaration>>> {
        &mut self.items
    }
    fn index_mut(&mut self) -> &mut HashMap<String,DecIntId> {
        &mut self.index
    }

    fn items(&self) -> &Vec<Option<Box<Declaration>>> {
        &self.items
    }
    fn index(&self) -> &HashMap<String,DecIntId> {
        &self.index
    }
}

pub struct ProvenanceStore {
    items: Vec<Option<Box<Processor>>>, //heap-allocated
    index: HashMap<String,ProcIntId>,
    pub chain: Vec<ProcIntId>,
}

impl Store<Processor,ProcIntId> for ProvenanceStore {
    fn items_mut(&mut self) -> &mut Vec<Option<Box<Processor>>> {
        &mut self.items
    }
    fn index_mut(&mut self) -> &mut HashMap<String,ProcIntId> {
        &mut self.index
    }

    fn items(&self) -> &Vec<Option<Box<Processor>>> {
        &self.items
    }
    fn index(&self) -> &HashMap<String,ProcIntId> {
        &self.index
    }
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
    pub processors: Vec<ProcIntId>,
    pub src: String,
    pub format: String,
    pub resourcelink: String,
    pub parent: Option<ProcIntId>,
    pub metadata: Metadata,
}

impl MaybeIdentifiable for Processor {
    fn id(&self) -> Option<String> {
        Some(self.id.clone())
    }
}

pub struct Metadata {
    pub data: HashMap<String,String>
}
