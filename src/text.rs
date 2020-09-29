use std::fmt;
use std::borrow::Cow;

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

#[derive(Clone)]
///Parameters for the text() method
pub struct TextParameters {
    pub set: Option<String>,
    pub textclass: Option<String>,
    pub strict: bool,
    pub retaintokenisation: bool,
}

impl<'a> Default for TextParameters {
    fn default() -> Self {
        Self {
            set: Some(DEFAULT_TEXT_SET.to_string()),
            textclass: Some("current".to_string()),
            strict: false,
            retaintokenisation: false,
        }
    }
}

impl TextParameters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn retaintokenisation(mut self, retaintokenisation: bool) -> Self {
        self.retaintokenisation = retaintokenisation;
        self
    }

    pub fn set(mut self, set: &str) -> Self {
        self.set = Some(set.to_string());
        self
    }

    pub fn textclass(mut self, textclass: &str) -> Self {
        self.textclass = Some(textclass.to_string());
        self
    }
}




impl<'a> Element<'a> {

    ///Returns the text content of a given element
    pub fn text_by_key(&self, set: DecKey, textclass: ClassKey, strict: bool, retaintokenisation: bool, previousdelimiter: Option<String>) -> Result<String,FoliaError> {
        let doc = self.document().ok_or(FoliaError::KeyError("Element has no associated document".to_string()))?;

        let properties = doc.props(self.elementtype());

        if properties.textcontainer {
            //we are a text container (<t> or markup or something)
            let mut text: String = String::new();
            for item in self.elementdata().data.iter()  {
                match item {
                    DataType::Text(item_text) => {
                        text += &item_text;
                    },
                    DataType::Element(element_key) => {
                        if let Some(element) = doc.get_element(*element_key) {
                            let properties = doc.props(element.elementtype());
                            if properties.printable {
                                if !text.is_empty() {
                                    if let Some(textdelimiter) = properties.textdelimiter {
                                        text += textdelimiter;
                                    }
                                }
                                let textpart = element.text_by_key(set,textclass,strict, retaintokenisation,None)?;
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
            let mut textcontent_element: Option<Element> = None;
            for element in self.elementdata().data.iter() {
                if let DataType::Element(element_key) = element {
                    if let Some(element) = doc.get_element(*element_key) {
                        if ElementGroup::Structure.contains(element.elementtype()) ||
                           element.elementtype() == ElementType::Correction ||
                           ElementGroup::Span.contains(element.elementtype()) {

                           if let Ok(textpart) = element.text_by_key(set,textclass,false, retaintokenisation, Some(delimiter.clone())) {
                               //delimiter will be buffered and only printed upon next iteration
                               text += &textpart;
                               if let Ok(s) = element.get_textdelimiter(retaintokenisation) {
                                   delimiter = s.to_string();
                               }
                           }
                        } else if element.elementtype() == ElementType::WordReference {
                            //follow the reference
                            if let Some(element) = element.resolve() {
                               if let Ok(textpart) = element.text_by_key(set,textclass,false, retaintokenisation, Some(delimiter.clone())) {
                                   //delimiter will be buffered and only printed upon next iteration
                                   text += &textpart;
                                   if let Ok(s) = element.get_textdelimiter(retaintokenisation) {
                                       delimiter = s.to_string();
                                   }
                               }
                            }
                        } else if element.elementtype() == ElementType::TextContent {
                            textcontent_element = Some(element);
                        }
                    }
                }
            }
            if text.is_empty() && textcontent_element.is_some() {
                if let Ok(parttext) = textcontent_element.unwrap().text_by_key(set,textclass,false,retaintokenisation, None) {
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
    pub fn get_textdelimiter(&self, retaintokenisation: bool) -> Result<Cow<str>,FoliaError> {
        let doc = self.document().ok_or(FoliaError::KeyError("Element has no associated document".to_string()))?;
        let properties =  doc.props(self.elementtype());
        if properties.textdelimiter.is_none() {
            //no text delimiter of itself, recurse into children to inherit delimiter
            for item in self.elementdata().data.iter().rev() {
                if let DataType::Element(element_key) = item {
                    if let Some(element) = doc.get_element(*element_key) {
                        //recurse
                        match element.get_textdelimiter(retaintokenisation) {
                            Ok(Cow::Borrowed(s)) => {
                                return Ok(Cow::Owned(s.to_owned()));
                            },
                            Ok(Cow::Owned(s)) => {
                                return Ok(Cow::Owned(s));
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                }
            }
            Ok(Cow::Borrowed(""))
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
                Ok(Cow::Borrowed(properties.textdelimiter.unwrap()))
            } else {
                Ok(Cow::Borrowed(""))
            }
        } else {
            Ok(Cow::Borrowed(properties.textdelimiter.unwrap()))
        }
    }

    ///Returns the text content of a given element
    ///This method takes string parameters for set and textclass, which can be set to None to
    ///fallback to the default text set and "current class".
    pub fn text(&self, textparameters: &TextParameters) -> Result<String,FoliaError> {
        let doc = self.document().ok_or(FoliaError::KeyError("Element has no associated document".to_string()))?;
        if let Some(dec_key) = doc.get_declaration_key_by_id(Declaration::index_id(AnnotationType::TEXT, &textparameters.set.as_ref().map(|s| s.as_str())).as_str()) {
            let class_key = doc.class_key(dec_key, &textparameters.textclass.as_ref().map(|s| s.as_str()).expect("unwrapping textclass")  )?;
            self.text_by_key(dec_key, class_key,textparameters.strict,textparameters.retaintokenisation, None)
        } else {
            Err(FoliaError::EncodeError(format!("No declaration for the specified text set ({})", textparameters.set.as_ref().map(|s| s.as_str()).expect("unwrapping set"))))
        }
    }
}




impl Document {
    ///Returns the text of the given element
    pub fn text_by_key(&self, element_key: ElementKey, set: DecKey, textclass: ClassKey, strict: bool, retaintokenisation: bool) -> Result<String,FoliaError> {
        if let Some(element) = self.get_element(element_key) {
            element.text_by_key(set, textclass, strict, retaintokenisation,None)
        } else {
            Err(FoliaError::KeyError(format!("No such element key: {}", element_key)))
        }
    }

    ///Returns the text of the given element
    pub fn text(&self, element_key: ElementKey, textparameters: &TextParameters) -> Result<String,FoliaError> {
        if let Some(element) = self.get_element(element_key) {
            element.text(textparameters)
        } else {
            Err(FoliaError::KeyError(format!("No such element key: {}", element_key)))
        }
    }
}
