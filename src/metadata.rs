use std::collections::HashMap;

use crate::common::*;
use crate::types::*;
use crate::element::*;
use crate::store::*;


pub struct Declaration {
    pub annotationtype: AnnotationType,
    pub set: Option<String>,
    pub processors: Vec<ProcKey>
}

impl Declaration {
    pub fn new(annotationtype: AnnotationType, set: Option<String>) -> Declaration {
        Declaration { annotationtype: annotationtype, set: set, processors: vec![] }
    }
}

impl CheckEncoded for Declaration { }

impl CheckEncoded for String { }

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
pub struct ClassStore {
    items: Vec<Option<Box<String>>>, //heap-allocated
    index: HashMap<String,ClassKey>
}

impl MaybeIdentifiable for String {
    fn id(&self) -> Option<String> {
        Some(self.to_string())
    }
}


impl Store<String,ClassKey> for ClassStore {
    fn items_mut(&mut self) -> &mut Vec<Option<Box<String>>> {
        &mut self.items
    }
    fn index_mut(&mut self) -> &mut HashMap<String,ClassKey> {
        &mut self.index
    }

    fn items(&self) -> &Vec<Option<Box<String>>> {
        &self.items
    }
    fn index(&self) -> &HashMap<String,ClassKey> {
        &self.index
    }
}


#[derive(Default)]
pub struct DeclarationStore {
    items: Vec<Option<Box<Declaration>>>, //heap-allocated
    index: HashMap<String,DecKey>,
    classes: Option<ClassStore>
}

impl Store<Declaration,DecKey> for DeclarationStore {
    fn items_mut(&mut self) -> &mut Vec<Option<Box<Declaration>>> {
        &mut self.items
    }
    fn index_mut(&mut self) -> &mut HashMap<String,DecKey> {
        &mut self.index
    }

    fn items(&self) -> &Vec<Option<Box<Declaration>>> {
        &self.items
    }
    fn index(&self) -> &HashMap<String,DecKey> {
        &self.index
    }
}

#[derive(Default)]
pub struct ProvenanceStore {
    items: Vec<Option<Box<Processor>>>, //heap-allocated
    index: HashMap<String,ProcKey>,
    pub chain: Vec<ProcKey>,
}

impl Store<Processor,ProcKey> for ProvenanceStore {
    fn items_mut(&mut self) -> &mut Vec<Option<Box<Processor>>> {
        &mut self.items
    }
    fn index_mut(&mut self) -> &mut HashMap<String,ProcKey> {
        &mut self.index
    }

    fn items(&self) -> &Vec<Option<Box<Processor>>> {
        &self.items
    }
    fn index(&self) -> &HashMap<String,ProcKey> {
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

impl Default for ProcessorType {
    fn default() -> Self { ProcessorType::Auto }
}

#[derive(Default)]
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
    pub processors: Vec<ProcKey>,
    pub src: String,
    pub format: String,
    pub resourcelink: String,
    pub parent: Option<ProcKey>,
    pub metadata: Metadata,
}

impl CheckEncoded for Processor { }

impl MaybeIdentifiable for Processor {
    fn id(&self) -> Option<String> {
        Some(self.id.clone())
    }
}

#[derive(Default)]
pub struct Metadata {
    pub data: HashMap<String,String>
}
