use std::collections::HashMap;
use std::fmt;
use std::borrow::Cow;

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
    pub processors: Vec<ProcKey>,
    pub classes: Option<ClassStore>,
    pub features: Option<HashMap<SubsetKey,ClassStore>>
}

impl Declaration {
    ///Creates a new declaration, which can for instance be passed to ``Document.add_declaration()``.
    pub fn new(annotationtype: AnnotationType, set: Option<String>, alias: Option<String>) -> Declaration {
        Declaration { annotationtype: annotationtype, set: set, alias: alias, processors: vec![] , classes: None, key: None, features: None }
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
           class_store.get(class_key).map(|s| s.as_str())
        } else {
            None
        }
    }

    pub fn get_feature(&self, subset_key: SubsetKey, class_key: ClassKey) -> Option<&str> {
        if let Some(features) = &self.features {
            if let Some(class_store) = features.get(&subset_key) {
               return class_store.get(class_key).map(|s| s.as_str());
            }
        }
        None
    }

    ///Encode a class, adding it to the class store if needed, returning the existing one if
    ///already present. For an immutable variant, see ``get_class_key()``
    pub fn add_class(&mut self, class: &Class) -> Result<ClassKey,FoliaError> {
        if self.classes.is_none() {
            self.classes = Some(ClassStore::default());
        }
        if let Some(class_key) = self.classes.as_ref().unwrap().id_to_key(class) {
            Ok(class_key)
        } else {
            let class_key = self.classes.as_ref().unwrap().get_key(class);
            if let Some(class_key) = class_key {
                Ok(class_key)
            } else {
                self.classes.as_mut().unwrap().add(class.to_owned())
            }
        }
    }


    ///Encode a class, assumes it already exists. If not, use ``add_class()`` instead.
    pub fn get_class_key(&self, class: &str) -> Result<ClassKey,FoliaError> {
        if let Some(class_store) = &self.classes {
            if let Some(class_key) = class_store.id_to_key(class) {
                Ok(class_key)
            } else {
                let class = class.to_string();
                let class_key = class_store.get_key(&class);
                if let Some(class_key) = class_key {
                    Ok(class_key)
                } else {
                    Err(FoliaError::KeyError("[encode_class()] Class does not exist".to_string()))
                }
            }
        } else {
            Err(FoliaError::KeyError("[encode_class()] Class does not exist (empty class store)".to_string()))
        }
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
    items: Vec<Option<Box<Class>>>, //heap-allocated
    index: HashMap<Class,ClassKey>
}


impl Store<Class,ClassKey> for ClassStore {

    fn items_mut(&mut self) -> &mut Vec<Option<Box<Class>>> {
        &mut self.items
    }
    fn index_mut(&mut self) -> &mut HashMap<Class,ClassKey> {
        &mut self.index
    }

    fn items(&self) -> &Vec<Option<Box<Class>>> {
        &self.items
    }
    fn index(&self) -> &HashMap<Class,ClassKey> {
        &self.index
    }
    fn iter(&self) -> std::slice::Iter<Option<Box<Class>>> {
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
    pub fn add_class(&mut self, dec_key: DecKey, class: &Class) -> Result<ClassKey,FoliaError> {
        if let Some(declaration) = self.get_mut_declaration(dec_key) {
            declaration.add_class(class)
        } else {
            Err(FoliaError::KeyError(format!("[get_class_store()] No such declaration ({})", dec_key)))
        }
    }


    ///Encode a class, assumes it already exists. If not, use ``add_class()`` instead.
    pub fn get_class_key(&self, dec_key: DecKey, class: &str) -> Result<ClassKey,FoliaError> {
        if let Some(declaration) = self.get_declaration(dec_key) {
            declaration.get_class_key(class)
        } else {
            Err(FoliaError::KeyError(format!("[get_class_store()] No such declaration ({})", dec_key)))
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
    pub begindatetime: String,
    pub enddatetime: String,
    pub processors: Vec<ProcKey>,
    pub src: String,
    pub format: String,
    pub resourcelink: String,
    pub parent: Option<ProcKey>,
    pub metadata: Metadata,
    pub key: Option<ProcKey>
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


