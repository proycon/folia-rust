use std::collections::HashMap;
use std::fmt;
use std::borrow::Cow;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::NaiveDateTime;
use rand::prelude::*;

use crate::common::*;
use crate::error::*;
use crate::types::*;
use crate::store::*;
use crate::document::*;


///Represent a declaration for a particular annotation type, a set (optional), and associated with
///zero or more annotators or processors. Also holds and owns
///the class store for any classes in that set.
#[derive(Clone)]
pub struct Declaration {
    pub key: Option<DecKey>,
    pub annotationtype: AnnotationType,
    pub set: Option<String>,
    pub alias: Option<String>,
    pub format: Option<String>,
    pub processors: Vec<ProcKey>,
    pub classes: Option<ClassStore>,
    pub subsets: Option<SubsetStore>,
    pub subclasses: Option<ClassStore> //aggregates classes from all subsets
}

impl Declaration {
    ///Creates a new declaration, which can for instance be passed to ``Document.add_declaration()``.
    pub fn new(annotationtype: AnnotationType, set: Option<String>, alias: Option<String>, format: Option<String>) -> Declaration {
        Declaration { annotationtype: annotationtype, set: set, alias: alias, processors: vec![] , format: format, classes: None, key: None, subclasses: None, subsets: None }
    }

    ///Returns the key of default processor, if any
    pub fn default_processor(&self) -> Option<ProcKey> {
        if self.processors.len() == 1 {
            self.processors.get(0).map(|x| x.to_owned())
        } else {
            None
        }
    }

    ///Create a id to use with the index
    pub fn index_id(annotationtype: AnnotationType, set: &Option<&str>) -> String {
        if let Some(set) = set {
            format!("{}/{}", annotationtype, set)
        } else {
            format!("{}", annotationtype)
        }
    }

    pub fn get_class(&self, class_key: ClassKey) -> Option<&str> {
        if let Some(class_store) = &self.classes {
            class_store.get(class_key)
        } else {
            None
        }
    }

    pub fn get_subset(&self, subset_key: SubsetKey) -> Option<&str> {
        if let Some(subset_store) = &self.subsets {
            subset_store.get(subset_key)
        } else {
            None
        }
    }

    ///get a feature by key
    pub fn get_subclass(&self, subclass_key: ClassKey) -> Option<&str> {
        if let Some(subclasses) = &self.subclasses {
            subclasses.get(subclass_key)
        } else {
            None
        }
    }


    ///Encode a class, adding it to the class store if needed, returning the existing one if
    ///already present. For an immutable variant, see ``get_class_key()``
    pub fn add_class(&mut self, class: Cow<str>) -> Result<ClassKey,FoliaError> {
        if self.classes.is_none() {
            self.classes = Some(ClassStore::default());
        }
        if let Some(class_key) = self.classes.as_ref().unwrap().get_key(&class) {
            Ok(class_key)
        } else {
            self.classes.as_mut().unwrap().add(class)
        }
    }

    ///Encode a subset, adding it to the subset store if needed, returning the existing one if
    ///already present. For an immutable variant, see ``get_subset_key()``
    pub fn add_subset(&mut self, subset: Cow<str>) -> Result<SubsetKey,FoliaError> {
        if self.subsets.is_none() {
            self.subsets = Some(SubsetStore::default());
        }
        if let Some(subset_key) = self.subsets.as_ref().unwrap().get_key(&subset) {
            Ok(subset_key)
        } else {
            self.subsets.as_mut().unwrap().add(subset)
        }
    }

    ///Encode a subclass, adding it to the subclass store if needed, returning the existing one if
    ///already present. For an immutable variant, see ``get_subclass_key()``
    pub fn add_subclass(&mut self, subclass: Cow<str>) -> Result<ClassKey,FoliaError> {
        if self.subclasses.is_none() {
            self.subclasses = Some(ClassStore::default());
        }
        if let Some(subclass_key) = self.subclasses.as_ref().unwrap().get_key(&subclass) {
            Ok(subclass_key)
        } else {
            self.subclasses.as_mut().unwrap().add(subclass)
        }
    }


    ///Encode a class, assumes it already exists. If not, use ``add_class()`` instead.
    pub fn class_key(&self, class: &str) -> Option<ClassKey> {
        if let Some(class_store) = &self.classes {
            if let Some(class_key) = class_store.get_key(class) {
                return Some(class_key);
            }
        }
        None
    }

    ///Encode a subset, assumes it already exists. If not, use ``add_subset()`` instead.
    pub fn subset_key(&self, subset: &str) -> Option<SubsetKey> {
        if let Some(subset_store) = &self.subsets {
            if let Some(subset_key) = subset_store.get_key(subset) {
                return Some(subset_key);
            }
        }
        None
    }

    ///Encode a subclass, assumes it already exists. If not, use ``add_subclass()`` instead.
    pub fn subclass_key(&self, subclass: &str) -> Option<ClassKey> {
        if let Some(subclass_store) = &self.subclasses {
            if let Some(subclass_key) = subclass_store.get_key(subclass) {
                return Some(subclass_key);
            }
        }
        None
    }
}

impl Storable<DecKey> for Declaration {
    fn maybe_id(&self) -> Option<Cow<str>> {
        //let set_str: &str = &self.set.as_ref().expect("unwrapping set str");
        Some(Cow::from(Declaration::index_id(self.annotationtype,&self.set.as_ref().map(String::as_str))))
    }

    ///Returns the key of the current element
    fn key(&self) -> Option<DecKey> {
        self.key
    }

    ///Sets the key of the current element
    fn assign_key(&mut self, key: DecKey) {
        self.key = Some(key);
    }
}

impl Storable<ClassKey> for Class {
    fn maybe_id(&self) -> Option<Cow<str>> {
        Some(Cow::from(self))
    }
}

#[derive(Default,Clone)]
///The declaration store holds all classes that occur (e.g. in a document for a given set and
///annotation type). There are multiple class stores, which are owned by their respective ``Declaration`` (for a given set and
///annotation type).
pub struct ClassStore {
    items: Vec<Option<String>>, //heap-allocated
    index: HashMap<String,ClassKey>
}


impl StringStore<ClassKey> for ClassStore {

    fn items_mut(&mut self) -> &mut Vec<Option<String>> {
        &mut self.items
    }
    fn index_mut(&mut self) -> &mut HashMap<String,ClassKey> {
        &mut self.index
    }

    fn items(&self) -> &Vec<Option<String>> {
        &self.items
    }
    fn index(&self) -> &HashMap<String,ClassKey> {
        &self.index
    }
    fn iter(&self) -> std::slice::Iter<Option<String>> {
        self.items.iter()
    }
}


#[derive(Default,Clone)]
///The declaration store holds all classes that occur (e.g. in a document for a given set and
///annotation type). There are multiple class stores, which are owned by their respective ``Declaration`` (for a given set and
///annotation type).
pub struct SubsetStore {
    items: Vec<Option<String>>, //heap-allocated
    index: HashMap<String,SubsetKey>
}


impl StringStore<SubsetKey> for SubsetStore {

    fn items_mut(&mut self) -> &mut Vec<Option<String>> {
        &mut self.items
    }
    fn index_mut(&mut self) -> &mut HashMap<String,SubsetKey> {
        &mut self.index
    }

    fn items(&self) -> &Vec<Option<String>> {
        &self.items
    }
    fn index(&self) -> &HashMap<String,SubsetKey> {
        &self.index
    }
    fn iter(&self) -> std::slice::Iter<Option<String>> {
        self.items.iter()
    }
}

#[derive(Default)]
///The declaration store holds all declarations (e.g. for a document)
pub struct DeclarationStore {
    pub(crate) items: Vec<Option<Box<Declaration>>>, //heap-allocated
    pub(crate) index: HashMap<String,DecKey>,
}

impl DeclarationStore {


    ///Returns a vector of boolean, indicating if the declaration is a default or not. Can be
    ///indexed with DecKey
    pub fn default_mask(&self) -> Vec<bool> {
        let mut typecount: HashMap<AnnotationType,usize> = HashMap::new();
        for declaration in self.items.iter() {
            if let Some(declaration) = declaration {
                if let Some(count) = typecount.get_mut(&declaration.annotationtype) {
                    *count += 1;
                } else {
                    typecount.insert(declaration.annotationtype, 1);
                }
            }
        }
        let mut mask: Vec<bool> = Vec::with_capacity(self.items.len());
        for declaration in self.items.iter() {
            if let Some(declaration) = declaration {
                mask.push( typecount.get(&declaration.annotationtype) == Some(&1) );
            } else {
                mask.push(false);
            }
        }
        mask
    }

    ///Retrieves the key for the default annotation for the given annotationtype (if there is a
    ///default)
    pub fn get_default_key(&self, annotationtype: AnnotationType) -> Option<DecKey> {
        let matches: Vec<usize> = self.items.iter().enumerate().filter_map(|(index, declaration)|  {
            if let Some(declaration) = declaration {
                if declaration.annotationtype  == annotationtype {
                    Some(index)
                } else {
                    None
                }
            } else {
                None
            }
        }).collect();
        if matches.len() == 1 {
            Some(matches[0] as DecKey)
        } else {
            None
        }
    }

}

impl Document {

    ///Encode a class, adding it to the class store if needed, returning the existing one if
    ///already present
    pub fn add_class(&mut self, dec_key: DecKey, class: &String) -> Result<ClassKey,FoliaError> {
        if let Some(declaration) = self.get_mut_declaration(dec_key) {
            declaration.add_class(Cow::Borrowed(class.as_str()))
        } else {
            Err(FoliaError::KeyError(format!("[add_class()] No such declaration ({})", dec_key)))
        }
    }


    ///Encode a class, assumes it already exists. If not, use ``add_class()`` instead.
    pub fn class_key(&self, dec_key: DecKey, class: &str) -> Result<ClassKey,FoliaError> {
        if let Some(declaration) = self.get_declaration(dec_key) {
            declaration.class_key(class).ok_or(FoliaError::KeyError(format!("[class_key()] No such class ({}) for the given declaration", class)))
        } else {
            Err(FoliaError::KeyError(format!("[class_key()] No such declaration ({})", dec_key)))
        }
    }

}

#[derive(Default)]
pub struct ProvenanceStore {
    pub(crate) items: Vec<Option<Box<Processor>>>, //heap-allocated
    pub(crate) index: HashMap<String,ProcKey>,
    pub chain: Vec<ProcKey>,
}



#[derive(Debug,PartialEq,Clone,Copy)]
///Represents the type of a processor
pub enum ProcessorType {
    Auto,
    Manual,
    Generator,
    DataSource,
}

impl ProcessorType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProcessorType::Auto => "auto",
            ProcessorType::Manual => "manual",
            ProcessorType::Generator => "generator",
            ProcessorType::DataSource => "datasource",
        }
    }
}

impl Default for ProcessorType {
    fn default() -> Self { ProcessorType::Auto }

}

impl fmt::Display for ProcessorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Default,Clone)]
///Represents a processor
pub struct Processor {
    pub id: String,
    pub name: String,
    pub processortype: ProcessorType,
    pub version: String,
    pub folia_version: String,
    pub document_version: String,
    pub command: String,
    pub host: String,
    pub user: String,
    pub begindatetime: Option<NaiveDateTime>,
    pub enddatetime: Option<NaiveDateTime>,
    pub processors: Vec<ProcKey>,
    pub src: String,
    pub format: String,
    pub resourcelink: String,
    pub parent: Option<ProcKey>,
    pub metadata: Metadata,
    pub key: Option<ProcKey>
}

impl Processor {
    ///Instantiates a new processor
    pub fn new(name: String) -> Processor {
        let mut randomidstring: Vec<u8> = Vec::new();
        for _ in 0..16 {
            randomidstring.push(rand::random::<u8>());
        }
        Processor {
            id: format!("proc.{}.{}",name, hex::encode(randomidstring)),
            name: name,
            processortype: ProcessorType::default(),
            version: "".to_string(),
            folia_version: "".to_string(),
            document_version: "".to_string(),
            command: "".to_string(),
            host: "".to_string(),
            user: "".to_string(),
            begindatetime: None,
            enddatetime: None,
            processors: vec!(),
            src: "".to_string(),
            format: "".to_string(),
            resourcelink: "".to_string(),
            parent: None,
            metadata: Metadata::default(),
            key: None,
        }
    }

    //builder patterns
    pub fn with_type(mut self, processortype: ProcessorType) -> Processor {
        self.processortype = processortype;
        self
    }
    pub fn with_id(mut self, value: String) -> Processor {
        self.id = value;
        self
    }
    pub fn with_version(mut self, value: String) -> Processor {
        self.version = value;
        self
    }
    pub fn with_folia_version(mut self, value: String) -> Processor {
        self.folia_version = value;
        self
    }
    pub fn with_document_version(mut self, value: String) -> Processor {
        self.document_version = value;
        self
    }
    pub fn with_command(mut self, value: String) -> Processor {
        self.command = value;
        self
    }
    pub fn with_host(mut self, value: String) -> Processor {
        self.host = value;
        self
    }
    pub fn with_user(mut self, value: String) -> Processor {
        self.user = value;
        self
    }
    pub fn with_begindatetime(mut self, dt: NaiveDateTime) -> Processor {
        self.begindatetime = Some(dt);
        self
    }
    pub fn with_enddatetime(mut self, dt: NaiveDateTime) -> Processor {
        self.enddatetime = Some(dt);
        self
    }
    pub fn with_src(mut self, value: String) -> Processor {
        self.src = value;
        self
    }
    pub fn with_format(mut self, value: String) -> Processor {
        self.format = value;
        self
    }
    pub fn with_resourcelink(mut self, value: String) -> Processor {
        self.resourcelink = value;
        self
    }
    ///attempts to automatically fill the processor's fields based on the environment
    pub fn autofill(self) -> Processor {
        let host: String = env::var("HOST").unwrap_or_default();
        let user: String = env::var("USER").unwrap_or_default();
        let command: String = env::args().collect();
        let begindatetime: NaiveDateTime = NaiveDateTime::from_timestamp(
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("Unable to get time").as_secs() as i64, 0
        );
        self.with_host(host).with_begindatetime(begindatetime).with_folia_version(FOLIAVERSION.to_string()).with_user(user).with_command(command)
    }
}

impl Storable<ProcKey> for Processor {
    ///Returns the key of the current processor
    fn key(&self) -> Option<ProcKey> {
        self.key
    }

    ///Sets the key of the current processor
    fn assign_key(&mut self, key: ProcKey) {
        self.key = Some(key);
    }

    fn maybe_id(&self) -> Option<Cow<str>> {
        Some(Cow::from(&self.id))
    }
}

#[derive(Default,Clone)]
///A key/value store (``data``) containing arbitrary metadata (FoLiA native metadata)
///Instead of using the key/value store, it may also refer to an external metadata source
///(``src``).
pub struct Metadata {
    pub data: HashMap<String,String>,
    pub src: Option<String>,
    pub metadatatype: Option<String>,
}


