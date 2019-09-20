use std::path::{Path};
use std::io::BufRead;
use std::io::BufReader;
use std::io::Cursor;
use std::fs::File;
use std::str;
use std::str::FromStr;
use std::borrow::Cow;
use std::string::ToString;

use quick_xml::{Reader,Writer};
use quick_xml::events::{Event,BytesStart,BytesEnd,BytesText};

use crate::common::*;
use crate::types::*;
use crate::error::*;
use crate::element::*;
use crate::attrib::*;
use crate::store::*;
use crate::elementstore::*;
use crate::metadata::*;
use crate::select::*;
use crate::serialiser::*;
use crate::parser::*;
use crate::specification::*;

pub struct Document<'a> {
    pub id: String,
    ///The FoLiA version of the document
    pub version: String,
    pub filename: Option<String>,
    ///The element store holds and owns all elements in a document
    pub elementstore: ElementStore<'a>,
    ///The provenance store holds and owns all processors and a representation of the  provenance chain
    pub provenancestore: ProvenanceStore,
    ///The declaration store holds all annotation declarations
    pub declarationstore: DeclarationStore,
    ///Metadata consists of a simple key/value store (or a reference to external metadata)
    pub metadata: Metadata,
}



impl<'a> Document<'a> {
    ///Create a new FoLiA document from scratch
    pub fn new(id: &str, bodytype: BodyType) -> Result<Document<'a>, FoliaError> {
        let mut document = Document {
            id: id.to_string(),
            filename: None,
            version: FOLIAVERSION.to_string(),
            elementstore: ElementStore::default(),
            provenancestore:  ProvenanceStore::default(),
            declarationstore: DeclarationStore::default(),
            metadata: Metadata::default(),
        };
        let mut body = match bodytype {
            BodyType::Text => FoliaElement::new(ElementType::Text),
            BodyType::Speech => FoliaElement::new(ElementType::Speech),
        };
        body = document.encode(body)?;
        assert!(body.is_encoded());
        document.add(body)?;
        Ok(document)
    }

    ///Load a FoliA document from file. Invokes the XML parser and loads it all into memory.
    pub fn from_file(filename: &str) -> Result<Document<'a>, FoliaError> {
        let mut reader = Reader::from_file(Path::new(filename))?;
        reader.trim_text(true);
        let mut doc = Document::parse(&mut reader)?;
        //associate the filename with the document
        doc.filename = Some(filename.to_string());
        Ok(doc)
    }

    ///Load a FoliA document from XML string representation, loading it all into memory.
    pub fn from_str(data: &str) -> Result<Document<'a>, FoliaError> {
        let mut reader = Reader::from_str(data);
        reader.trim_text(true);
        Document::parse(&mut reader)
    }





    ///Returns the ID of the document
    pub fn id(&self) -> &str { &self.id }

    ///Returns the filename associated with this document (i.e. the file from which it was loaded)
    pub fn filename(&self) -> Option<&str> { self.filename.as_ref().map(String::as_str) } //String::as_str equals  |x| &**x


    pub fn textelement_encode(&'a self, element_key: ElementKey, set: Option<&str>, textclass: Option<&str>) -> Option<&'a FoliaElement<'a>> {
        let set: &str = if let Some(set) = set {
            set
        } else {
            DEFAULT_TEXT_SET
        };
        let textclass: &str = if let Some(textclass) = textclass {
            textclass
        } else {
            "current"
        };
        let selector = Selector::new_encode(&self, ElementType::TextContent, SelectorValue::Some(set), SelectorValue::Some(textclass));
        for element in self.select_elements(element_key, selector, false)  {
            return Some(element.element);
        }
        None
    }




    ///Get properties from the specification (a shortcut)
    pub fn props(&self, elementtype: ElementType) -> &Properties {
        self.elementstore.specification.get(elementtype)
    }

    //************** Methods providing easy access to IntoStore ********************

    ///Add an element to the document (but the element will be an orphan unless it is the very
    ///first one, you may want to use ``add_element_to`` instead)
    pub fn add_element(&'a mut self, element: FoliaElement<'a>) -> Result<ElementKey, FoliaError> {
        <Self as IntoStore<FoliaElement<'a>,ElementKey>>::add(self, element)
    }

    ///Add a declaration. It is strongly recommended to use ``declare()`` instead
    ///because this one adds a declaration without any checks.
    ///Returns the key.
    pub fn add_declaration(&'a mut self, declaration: Declaration) -> Result<DecKey, FoliaError> {
        <Self as IntoStore<'a,Declaration,DecKey>>::add(self, declaration)
    }

    ///Add an processor the document (but the processor will be an orphan and not in the processor
    ///chain!). You may want to use ``add_processor()`` instead to add to the provenance chain or
    ///``add_subprocessor()`` to add a processor as a subprocessor.
    pub fn add_provenance(&'a mut self, processor: Processor) -> Result<ProcKey, FoliaError> {
        <Self as IntoStore<'a,Processor,ProcKey>>::add(self, processor)
    }

    //************** Higher-order methods for adding things ********************

    ///Adds an element as a child of another, this is a higher-level function that/
    ///takes care of adding and attaching for you.
    pub fn add_element_to(&'a mut self, parent_key: ElementKey, mut element: FoliaElement<'a>) -> Result<ElementKey, FoliaError> {
        element = <Self as IntoStore<'a,FoliaElement<'a>,ElementKey>>::encode(self, element)?;
        self.elementstore.add_to(parent_key, element)
    }

    ///Add an element to the provenance chain
    ///Returns the key
    pub fn add_processor(&mut self, processor: Processor) -> Result<ProcKey, FoliaError> {
        self.provenancestore.add_to_chain(processor)
    }

    ///Add a processor as a subprocessor
    ///Returns the key
    pub fn add_subprocessor(&mut self, parent_processor: ProcKey, processor: Processor) -> Result<ProcKey, FoliaError> {
        self.provenancestore.add_to(parent_processor, processor)
    }

    ///Add a declaration. Returns the key. If the declaration already exists it simply returns the
    ///key of the existing one.
    pub fn declare(&'a mut self, annotationtype: AnnotationType, set: &Option<String>, alias: &Option<String>) -> Result<DecKey,FoliaError> {
        //first we simply check the index
        if let Some(found_key) = self.declarationstore.id_to_key(DeclarationStore::index_id(annotationtype, &set.as_ref().map(String::as_str)  ).as_str()) {
            return Ok(found_key);
        }

        //If not found, we search for a default
        if let Some(default_key) = self.declarationstore.get_default_key(annotationtype) {
            if let Some(declaration) = self.declarationstore.get(default_key) {
                if set.is_some() {
                    //there is an explicit set defined, only return the default if the sets are not
                    //in conflict
                    if let Some(declared_set) = &declaration.set {
                        if *declared_set == *set.as_ref().unwrap() {
                            return Ok(default_key);
                        }
                    }
                } else {
                    //no set defined, that means we inherit the default set
                    return Ok(default_key);
                }
            }
        }

        //if we reach this point we have no defaults and add a new declaration
        let added_key = self.add_declaration(Declaration::new(annotationtype, set.clone(), alias.clone()))?;
        Ok(added_key)
    }

    //************** Methods providing easy access to FromStore ****************
    pub fn get_element(&'a self, key: ElementKey) -> Option<&FoliaElement<'a>> {
        <Self as FromStore<ElementKey,FoliaElement<'a>>>::get(self, key)
    }
    pub fn get_element_by_id(&'a self, id: &str) -> Option<&FoliaElement<'a>> {
        <Self as FromStore<ElementKey,FoliaElement<'a>>>::get_by_id(self, id)
    }
    pub fn get_mut_element(&'a mut self, key: ElementKey) -> Option<&mut FoliaElement<'a>> {
        <Self as FromStore<ElementKey,FoliaElement<'a>>>::get_mut(self, key)
    }
    pub fn get_mut_element_by_id(&'a mut self, id: &str) -> Option<&mut FoliaElement<'a>> {
        <Self as FromStore<ElementKey,FoliaElement<'a>>>::get_mut_by_id(self, id)
    }
    pub fn get_declaration(&'a self, key: DecKey) -> Option<&'a Declaration> {
        <Self as FromStore<DecKey,Declaration>>::get(self, key)
    }
    pub fn get_declaration_by_id(&'a self, id: &str) -> Option<&'a Declaration> {
        <Self as FromStore<DecKey,Declaration>>::get_by_id(self, id)
    }
    pub fn get_mut_declaration(&'a mut self, key: DecKey) -> Option<&'a mut Declaration> {
        <Self as FromStore<DecKey,Declaration>>::get_mut(self, key)
    }
    pub fn get_mut_declaration_by_id(&'a mut self, id: &str) -> Option<&'a mut Declaration> {
        <Self as FromStore<DecKey,Declaration>>::get_mut_by_id(self, id)
    }
    pub fn get_processor(&'a self, key: ProcKey) -> Option<&'a Processor> {
        <Self as FromStore<ProcKey,Processor>>::get(self, key)
    }
    pub fn get_processor_by_id(&'a self, id: &str) -> Option<&'a Processor> {
        <Self as FromStore<ProcKey,Processor>>::get_by_id(self, id)
    }
    pub fn get_mut_processor(&'a mut self, key: ProcKey) -> Option<&'a mut Processor> {
        <Self as FromStore<ProcKey,Processor>>::get_mut(self, key)
    }
    pub fn get_mut_processor_by_id(&'a mut self, id: &str) -> Option<&'a mut Processor> {
        <Self as FromStore<ProcKey,Processor>>::get_mut_by_id(self, id)
    }



}

impl<'a> FromStore<'a,ElementKey, FoliaElement<'a>> for Document<'a> {
    fn store(&self) -> &dyn Store<FoliaElement<'a>,ElementKey> {
        &self.elementstore
    }
    fn store_mut(&mut self) -> &mut dyn Store<FoliaElement<'a>,ElementKey> {
        &mut self.elementstore
    }
}


impl<'a> FromStore<'a,DecKey, Declaration> for Document<'a> {
    fn store(&self) -> &dyn Store<Declaration,DecKey> {
        &self.declarationstore
    }
    fn store_mut (&mut self) -> &mut dyn Store<Declaration,DecKey> {
        &mut self.declarationstore
    }
}

impl<'a> FromStore<'a,ProcKey, Processor> for Document<'a> {
    fn store(&self) -> &dyn Store<Processor,ProcKey> {
        &self.provenancestore
    }
    fn store_mut(&mut self) -> &mut dyn Store<Processor,ProcKey> {
        &mut self.provenancestore
    }
}

impl<'a> IntoStore<'a,FoliaElement<'a>,ElementKey> for Document<'a> {
    ///Actively encode element for storage, this encodes attributes that need to be encoded (such as set,class,processor), and adds them to their respective stores.
    ///It does not handle relations between elements (data/children and parent)
    ///nor does it add the element itself to the store
    ///to the store).
    fn encode(&mut self, mut element: FoliaElement<'a>) -> Result<FoliaElement<'a>, FoliaError> {
        if element.is_encoded() {
            //already encoded, nothing to do
            return Ok(element);
        }

        let mut enc_attribs: EncodedAttributes = EncodedAttributes::default();

        //encode the element for storage
        let set = element.attrib(AttribType::SET);

        if let Some(annotationtype) = element.elementtype.annotationtype() {
            //Declare the element (either declares anew or just resolves the to the right
            //declaration.
            let deckey = self.declare(annotationtype, &set.map(|x| x.value().into_owned() ), &None)?;
            enc_attribs.declaration = Some(deckey);

            if let Some(class) = element.attrib(AttribType::CLASS) {
                if let Attribute::Class(class) = class {
                    if let Ok(class_key) = self.declarationstore.add_class(deckey, class) {
                        enc_attribs.class = Some(class_key);
                    }
                }
            }

            if let Some(declaration) = self.get_declaration(deckey) {
                enc_attribs.processor = declaration.default_processor() //returns an Option, may be overriden later if a specific processor is et
            }
        }

        if let Some(processor) = element.attrib(AttribType::PROCESSOR) {
            let processor_id: &str  = &processor.value();

            if let Some(processor_key) = self.provenancestore.id_to_key(processor_id) {
                enc_attribs.processor = Some(processor_key); //overrides the earlier-set default (if any)
            }
        }

        //remove encoded attributes
        element.attribs.retain(|a| match a {
            Attribute::Set(_) | Attribute::Class(_) | Attribute::Processor(_) => false,
            _ => true
        });

        element.set_enc_attribs(Some(enc_attribs));

        Ok(element)
    }
}

impl<'a> IntoStore<'a,Declaration,DecKey> for Document<'a> {
}

impl<'a> IntoStore<'a,Processor,ProcKey> for Document<'a> {
}
