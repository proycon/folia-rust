use std::collections::HashMap;

use crate::common::*;
use crate::element::*;

pub type ProcessorIntId = usize;

pub struct Declaration {
    pub annotationtype: AnnotationType,
    pub set: Option<String>,
    pub processors: Vec<ProcessorIntId>
}

pub struct DeclarationStore {

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
