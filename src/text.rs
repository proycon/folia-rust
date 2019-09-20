use crate::common::*;
use crate::types::*;
use crate::element::*;
use crate::error::*;
use crate::attrib::*;
use crate::elementstore::*;
use crate::store::*;
use crate::metadata::*;
use crate::select::*;
use crate::document::*;


impl<'a> FoliaElement<'a> {

    ///Returns the text content of a given element
    pub fn text(&'a self, set: DecKey, textclass: ClassKey, strict: bool, retaintokenisation: bool, previousdelimiter: Option<String>) -> Result<String,FoliaError> {
        let doc = self.doc.ok_or(FoliaError::DetachedError("Element is not attached to anything (orphaned)".to_string()))?;
        let key = self.key().ok_or(FoliaError::KeyError("Element has no key".to_string()))?;

        let properties = doc.props(self.elementtype);

        if properties.textcontainer {
            //we are a text container (<t> or markup or something)
            let mut text: String = String::new();
            for item in self.data.iter()  {
                match item {
                    DataType::Text(item_text) => {
                        text += &item_text;
                    },
                    DataType::Element(element_key) => {
                        if let Some(element) = doc.elementstore.get(*element_key) {
                            let properties = doc.props(element.elementtype);
                            if properties.printable {
                                if !text.is_empty() {
                                    if let Some(textdelimiter) = properties.textdelimiter {
                                        text += textdelimiter;
                                    }
                                }
                                let textpart = element.text(set,textclass,strict, retaintokenisation,None)?;
                                text += &textpart;
                            }
                        }
                    },
                    _ => {},
                }
            }
            Ok(text)
        } else if !properties.printable || properties.hidden {
            Err(FoliaError::NoTextError("No such text".to_string()))
        } else {
            //Get text from children first
            let mut delimiter: String = String::new();
            let mut text: String = String::new();
            let mut textcontent_element: Option<&FoliaElement> = None;
            for element in self.data.iter() {
                if let DataType::Element(element_key) = element {
                    if let Some(element) = doc.elementstore.get(*element_key) {
                        if ElementGroup::Structure.contains(element.elementtype) ||
                           element.elementtype == ElementType::Correction ||
                           ElementGroup::Span.contains(element.elementtype) {

                           if let Ok(textpart) = element.text(set,textclass,false, retaintokenisation, Some(delimiter.clone())) {
                               //delimiter will be buffered and only printed upon next iteration
                               text += &textpart;
                               let delimiter_ref = element.get_textdelimiter(retaintokenisation)?;
                               delimiter = delimiter_ref.to_string();
                           }
                        } else if element.elementtype == ElementType::TextContent {
                            textcontent_element = Some(element);
                        }
                    }
                }
            }
            if text.is_empty() && textcontent_element.is_some() {
                if let Ok(parttext) = textcontent_element.unwrap().text(set,textclass,false,retaintokenisation, None) {
                    text = parttext
                }
            }

            if !text.is_empty() && previousdelimiter.is_some() {
                text = previousdelimiter.unwrap() + text.as_str();
            }

            if !text.is_empty() {
                Ok(text)
            } else {
                Err(FoliaError::NoTextError("No such text".to_string()))
            }
        }
    }

    ///Returns the text delimiter for this element
    pub fn get_textdelimiter(&'a self, retaintokenisation: bool) -> Result<&'a str,FoliaError> {
        let doc = self.doc.ok_or(FoliaError::DetachedError("Element is not attached to anything (orphaned)".to_string()))?;
        let properties =  doc.props(self.elementtype);
        if properties.textdelimiter.is_none() {
            //no text delimiter of itself, recurse into children to inherit delimiter
            for item in self.data.iter().rev() {
                if let DataType::Element(element_key) = item {
                }
            }
            Ok("")
        } else if properties.optional_attribs.contains(&AttribType::SPACE) {
            let space: bool = retaintokenisation || match self.attrib(AttribType::SPACE) {
                Some(Attribute::Space(space)) => {
                    *space
                },
                _ => {
                    true
                }
            };
            if space {
                Ok(properties.textdelimiter.unwrap())
            } else {
                Ok("")
            }
        } else {
            Ok(properties.textdelimiter.unwrap())
        }
    }

    ///Returns the text content of a given element
    ///This method takes string parameters for set and textclass, which can be set to None to
    ///fallback to the default text set and "current class".
    pub fn text_encode(&'a self, set: Option<&str>, textclass: Option<&str>, strict: bool, retaintokenisation: bool) -> Result<String,FoliaError> {
        let doc = self.doc.ok_or(FoliaError::DetachedError("Element is not attached to anything (orphaned)".to_string()))?;
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
        if let Some(dec_key) = doc.declarationstore.id_to_key(DeclarationStore::index_id(AnnotationType::TOKEN, &Some(set)).as_str()) {
            let class_key = doc.declarationstore.encode_class(dec_key, textclass)?;
            self.text(dec_key, class_key,strict,retaintokenisation, None)
        } else {
            Err(FoliaError::EncodeError("No declaration for the specified text set/class".to_string()))
        }
    }
}

impl<'a> Document<'a> {
    ///Returns the text of the given element
    pub fn text(&self, element_key: ElementKey, set: DecKey, textclass: ClassKey, strict: bool, retaintokenisation: bool) -> Result<String,FoliaError> {
        if let Some(element) = self.elementstore.get(element_key) {
            element.text(set, textclass, strict, retaintokenisation,None)
        } else {
            Err(FoliaError::KeyError(format!("No such element key: {}", element_key)))
        }
    }

    ///Returns the text of the given element
    pub fn text_encode(&self, element_key: ElementKey, set: Option<&str>, textclass: Option<&str>, strict: bool, retaintokenisation: bool) -> Result<String,FoliaError> {
        if let Some(element) = self.elementstore.get(element_key) {
            element.text_encode(set, textclass, strict, retaintokenisation)
        } else {
            Err(FoliaError::KeyError(format!("No such element key: {}", element_key)))
        }
    }
}
