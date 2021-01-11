#[macro_use]
extern crate matches;

use std::str;
use folia::*;

const EXAMPLE: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<FoLiA xmlns="http://ilk.uvt.nl/folia" version="2.0" xml:id="example">
  <metadata type="native">
      <annotations>
          <token-annotation set="https://raw.githubusercontent.com/LanguageMachines/uctodata/master/setdefinitions/tokconfig-eng.foliaset.ttl">
             <annotator processor="p1" />
          </token-annotation>
          <text-annotation>
             <annotator processor="p1" />
          </text-annotation>
          <sentence-annotation>
             <annotator processor="p1" />
          </sentence-annotation>
          <paragraph-annotation>
             <annotator processor="p1" />
          </paragraph-annotation>
          <pos-annotation set="adhoc">
             <annotator processor="p2" />
          </pos-annotation>
          <chunking-annotation set="shallowsyntaxset">
             <annotator processor="p2" />
          </chunking-annotation>
      </annotations>
      <provenance>
         <processor xml:id="p1" name="proycon" type="manual" />
         <processor xml:id="p2" name="proycon" type="manual" />
      </provenance>
      <meta id="language">eng</meta>
  </metadata>
  <text xml:id="example.text">
    <p xml:id="example.p.1">
      <s xml:id="example.p.1.s.1">
         <w xml:id="example.p.1.s.1.w.1" class="WORD">
            <t>Hello</t>
         </w>
         <w xml:id="example.p.1.s.1.w.2" class="WORD" space="no">
            <t>world</t>
         </w>
         <w xml:id="example.p.1.s.1.w.3" class="PUNCTUATION">
            <t>!</t>
         </w>
      </s>
      <s xml:id="example.p.1.s.2">
         <w xml:id="example.p.1.s.2.w.1" class="WORD">
            <t>This</t>
         </w>
         <w xml:id="example.p.1.s.2.w.2" class="WORD">
            <t>is</t>
         </w>
         <w xml:id="example.p.1.s.2.w.3" class="WORD">
            <t>an</t>
         </w>
         <w xml:id="example.p.1.s.2.w.4" class="WORD">
            <t>example</t>
            <pos class="noun">
                <feat subset="number" class="singular" />
            </pos>
         </w>
         <w xml:id="example.p.1.s.2.w.5" class="WORD">
            <t>&amp;</t>
         </w>
         <w xml:id="example.p.1.s.2.w.6" class="WORD">
            <t>a</t>
         </w>
         <w xml:id="example.p.1.s.2.w.7" class="WORD" space="no">
            <t>test</t>
         </w>
         <w xml:id="example.p.1.s.2.w.8" class="PUNCTUATION">
            <t>.</t>
         </w>
         <chunking>
            <chunk xml:id="example.p.1.s.2.chunk.1" class="np">
                <wref id="example.p.1.s.2.w.3" />
                <wref id="example.p.1.s.2.w.4" />
            </chunk>
         </chunking>
      </s>
    </p>
  </text>
</FoLiA>"#;

const EXAMPLE_DEP: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<FoLiA xmlns="http://ilk.uvt.nl/folia" version="2.0" xml:id="example">
  <metadata>
      <annotations>
          <token-annotation set="https://raw.githubusercontent.com/LanguageMachines/uctodata/master/setdefinitions/tokconfig-eng.foliaset.ttl">
			 <annotator processor="p1" />
		  </token-annotation>
          <text-annotation>
			 <annotator processor="p1" />
          </text-annotation>
          <sentence-annotation>
			 <annotator processor="p1" />
          </sentence-annotation>
          <paragraph-annotation>
			 <annotator processor="p1" />
          </paragraph-annotation>
          <dependency-annotation set="alpino-dependencies"> <!-- an ad-hoc set -->
			 <annotator processor="p2" />
		  </dependency-annotation>
          <syntax-annotation set="alpino-constituents"> <!-- an ad-hoc set -->
			 <annotator processor="p2" />
		  </syntax-annotation>
      </annotations>
      <provenance>
         <processor xml:id="p1" name="proycon" type="manual" />
         <processor xml:id="p2" name="alpino" />
      </provenance>
  </metadata>
  <text xml:id="example.text">
    <p xml:id="example.p.1">
        <s xml:id="example.p.1.s.1">
          <t>De man begroette hem.</t>
          <w xml:id="example.p.1.s.1.w.1"><t>De</t></w>
          <w xml:id="example.p.1.s.1.w.2"><t>man</t></w>
          <w xml:id="example.p.1.s.1.w.3"><t>begroette</t></w>
          <w xml:id="example.p.1.s.1.w.4" space="no"><t>hem</t></w>
          <w xml:id="example.p.1.s.1.w.5"><t>.</t></w>
          <dependencies>
            <dependency xml:id="example.p.1.s.1.dependency.1" class="su">
                <hd>
                    <wref id="example.p.1.s.1.w.3" t="begroette"/>
                </hd>
                <dep>
                    <wref id="example.p.1.s.1.w.2" t="man" />
                </dep>
            </dependency>
            <dependency xml:id="example.p.1.s.1.dependency.3" class="obj1">
                <hd>
                    <wref id="example.p.1.s.1.w.3" t="begroette"/>
                </hd>
                <dep>
                    <wref id="example.p.1.s.1.w.4" t="hem" />
                </dep>
            </dependency>
            <dependency xml:id="example.p.1.s.1.dependency.2" class="det">
                <hd>
                   <wref id="example.p.1.s.1.w.2" t="man" />
                </hd>
                <dep>
                    <wref id="example.p.1.s.1.w.1" t="De" />
                </dep>
            </dependency>
          </dependencies>
          <syntax>
            <su xml:id="example.p.1.s.1.su.1" class="top">
                <su xml:id="example.p.1.s.1.su.1_1" class="smain">
                    <su xml:id="example.p.1.s.1.su.1_1_1" class="np">
                        <su xml:id="example.p.1.s.1.su.1_1_1_1" class="top">
                            <wref id="example.p.1.s.1.w.1" t="De" />
                        </su>
                        <su xml:id="example.p.1.s.1.su.1_1_1_2" class="top">
                            <wref id="example.p.1.s.1.w.2" t="man" />
                        </su>
                    </su>
                    <su xml:id="example.p.1.s.1.su.1_1_2" class="verb">
                        <wref id="example.p.1.s.1.w.3" t="begroette" />
                    </su>
                    <su xml:id="example.p.1.s.1.su.1_1_3" class="pron">
                        <wref id="example.p.1.s.1.w.4" t="hem" />
                    </su>
                </su>
                <su xml:id="example.p.1.s.1.su.1_2" class="punct">
                    <wref id="example.p.1.s.1.w.5" t="." />
                </su>
            </su>
          </syntax>
        </s>
    </p>
  </text>
</FoLiA>"#;

const EXAMPLE_MARKUP: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<FoLiA xmlns="http://ilk.uvt.nl/folia" version="2.0" xml:id="example">
  <metadata type="native">
      <annotations>
          <text-annotation>
             <annotator processor="p1" />
          </text-annotation>
          <sentence-annotation>
             <annotator processor="p1" />
          </sentence-annotation>
          <paragraph-annotation>
             <annotator processor="p1" />
          </paragraph-annotation>
          <style-annotation set="adhoc">
             <annotator processor="p1" />
          </style-annotation>
      </annotations>
      <provenance>
         <processor xml:id="p1" name="proycon" type="manual" />
      </provenance>
      <meta id="language">eng</meta>
  </metadata>
  <text xml:id="example.text">
    <p xml:id="example.p.1">
      <s xml:id="example.p.1.s.1">
        <t>Hello <t-style class="bold">world</t-style>! How are <t-style class="italics">you</t-style> today?</t>
      </s>
    </p>
  </text>
</FoLiA>"#;

#[test]
fn test001_instantiate() {
    match Document::new("example", DocumentProperties::default()) {
        Ok(doc) => {
            assert_eq!(doc.id(), "example");
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}



#[test]
fn test002_add_element_to() {
    match Document::new("example", DocumentProperties::default()) {
        Ok(mut doc) => {
            let root: ElementKey = 0;
            let sentence = doc.add_element_to(root,
                                              ElementData::new(ElementType::Sentence)
                                              .with_attrib(Attribute::Id("s.1".to_string()))
                                              ).expect("Adding sentence");

            let word1 = doc.add_element_to(sentence,
                               ElementData::new(ElementType::Word)
                              .with_attrib(Attribute::Id("word.1".to_string()))
                              .with_text("hello".to_string())
                              ).expect("Adding word 1");

            let word2 = doc.add_element_to(sentence,
                               ElementData::new(ElementType::Word)
                               .with_attrib(Attribute::Id("word.2".to_string()))
                               .with_text("world".to_string())
                              ).expect("Adding word 2");

            //sanity tests after adding:
            assert_eq!(doc.get_element_key_by_id("word.1"), Some(word1));

            if let Some(element) = doc.get_element(word2) {
                assert_eq!(element.id(), Some("word.2"));
                assert_eq!(element.text(&TextParameters::default()).expect("text"),"world");
                assert_eq!(element.parent_key(), Some(sentence));
            } else {
                assert!(false, format!("Element could not be retrieved"));
            }
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test002b_annotate_structure() {
    match Document::new("example", DocumentProperties::default()) {
        Ok(mut doc) => {
            let root: ElementKey = 0;
            let sentence = doc.annotate(root,
                                        ElementData::new(ElementType::Sentence)
                                        .with_attrib(Attribute::Id("s.1".to_string()))
                                        ).expect("Adding sentence");

            let word1 = doc.annotate(sentence,
                         ElementData::new(ElementType::Word)
                         .with_attrib(Attribute::Id("word.1".to_string()))
                         .with_text("hello".to_string())
                        ).expect("Adding word 1");

            let word2 = doc.annotate(sentence,
                         ElementData::new(ElementType::Word)
                         .with_attrib(Attribute::Id("word.2".to_string()))
                         .with_text("world".to_string())
                        ).expect("Adding word 2");

            //sanity tests after adding:
            assert_eq!(doc.get_element_key_by_id("word.1"), Some(word1));

            if let Some(element) = doc.get_element(word2) {
                assert_eq!(element.id(), Some("word.2"));
                assert_eq!(element.text(&TextParameters::default()).expect("text"),"world");
                assert_eq!(element.parent_key(), Some(sentence));
            } else {
                assert!(false, format!("Element could not be retrieved"));
            }
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test002c_annotate_invalid() {
    match Document::new("example", DocumentProperties::default()) {
        Ok(mut doc) => {
            let root: ElementKey = 0;
            let sentence = doc.annotate(root,
                                            ElementData::new(ElementType::Sentence)
                                                                .with_attrib(Attribute::Id("s.1".to_string())) ).expect("Obtaining sentence");
            let result = doc.annotate(sentence, ElementData::new(ElementType::Speech));
            assert!(match result {
                Err(FoliaError::ValidationError(_)) => true,
                _ => false
            });
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test002c_annotate_span_fromstructure() {
    match Document::new("example", DocumentProperties::default()) {
        Ok(mut doc) => {
            let root: ElementKey = 0;
            let sentence = doc.annotate(root,
                                            ElementData::new(ElementType::Sentence)
                                                                .with_attrib(Attribute::Id("s.1".to_string())) ).expect("Obtaining sentence");
            doc.annotate(sentence,
                         ElementData::new(ElementType::Word)
                         .with_attrib(Attribute::Id("word.1".to_string()))
                         .with_text("hello".to_string())
                        ).expect("Adding word 1");

            doc.annotate(sentence,
                         ElementData::new(ElementType::Word)
                         .with_attrib(Attribute::Id("word.2".to_string()))
                         .with_text("world".to_string())
                        ).expect("Adding word 2");

            //a layer doesn't exist yet at this stage and will be created automatically:
            doc.annotate(sentence,
                         ElementData::new(ElementType::Entity)
                         .with_attrib(Attribute::Set("adhoc".to_string()))
                         .with_attrib(Attribute::Class("greeting".to_string()))
                         .with_span(&[ "word.1", "word.2" ])
            ).expect("adding entity");

            //add another one, the same layer should be re-used:
            doc.annotate(sentence,
                         ElementData::new(ElementType::Entity)
                         .with_attrib(Attribute::Set("adhoc".to_string()))
                         .with_attrib(Attribute::Class("greeting2".to_string()))
                         .with_span(&[ "word.1", "word.2" ])
            ).expect("adding second entity");

            //DEBUG
            let xml = doc.xml(0,4).unwrap();
            println!("{}", str::from_utf8(&xml).unwrap());
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test003_parse() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("decoding utf-8"), DocumentProperties::default()) {
        Ok(doc) => {
            assert_eq!(doc.id(), "example", "ID check");
            assert_eq!(doc.provenancestore.chain.len(), 2, "Sanity check of provenance chain (count only)");
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}


#[test]
fn test004_get_word_from_index() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("decoding utf-8"), DocumentProperties::default()) {
        Ok(doc) => {
            if let Some(_word) = doc.get_element_by_id("example.p.1.s.1.w.1") {
                assert!(true);
            } else {
                assert!(false, "unable to get word");
            }
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test005_decode() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("decoding utf-8"), DocumentProperties::default()) {
        Ok(doc) => {
            if let Some(word) = doc.get_element_by_id("example.p.1.s.1.w.1") {
                assert_eq!(word.set().expect("Unwrapping set"), "https://raw.githubusercontent.com/LanguageMachines/uctodata/master/setdefinitions/tokconfig-eng.foliaset.ttl");
                assert_eq!(word.class().expect("Unwrapping class"), "WORD");
                assert_eq!(word.processor().expect("Unwrapping processor"), "p1");
                assert_eq!(word.annotator().expect("Unwrapping annotator"), "proycon");
                assert_eq!(word.annotatortype().expect("Unwrapping annotator"), ProcessorType::Manual);
            } else {
                assert!(false, "Word not found");
            }
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test006a_serialise_all_unchecked() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            match doc.xml(0,4) {
                Ok(xml) => {
                    println!("{}",str::from_utf8(&xml).expect("conversion from utf-8"));
                },
                Err(err) => {
                    assert!(false, format!("Serialisation failed with error: {}",err));
                }
            }
        }
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test006a_serialise_word() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            if let Some(word) = doc.get_element_by_id("example.p.1.s.1.w.1") {
                match word.xml(4) {
                    Ok(xml) => assert_eq!(xml, "<w xml:id=\"example.p.1.s.1.w.1\" class=\"WORD\">\n<t>Hello</t>\n</w>"),
                    //TODO: there's a bug in the underlying library regarding intendation this should eventually be: Ok(xml) => assert_eq!(xml, "<w xml:id=\"example.p.1.s.1.w.1\" class=\"WORD\">\n    <t>Hello</t>\n</w>"),
                    Err(err) => assert!(false, format!("Serialisation failed with error: {}",err))
                }
            } else {
                assert!(false, "Word not found");
            }
        }
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test007_metadata() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            assert_eq!(doc.metadata.metadatatype.unwrap(), "native");
            assert_eq!(doc.metadata.src, None);
            let language = doc.metadata.data.get("language");
            assert_eq!(language.expect("unwrapping meta"), "eng");
        }
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test008a_selector_type() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            let selector = doc.select_data(Selector::elements().element(Cmp::Is(ElementType::Word)), Recursion::Always, true);
            assert!(selector.selector().matchable());
            let mut count = 0;
            for item in selector {
                count += 1;
                assert_matches!(*item, DataType::Element(_));
            }
            assert_eq!(count, 11, "Checking whether we have the right amount of matches");
        }
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test008b_selector_set_class() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            let set = "https://raw.githubusercontent.com/LanguageMachines/uctodata/master/setdefinitions/tokconfig-eng.foliaset.ttl";
            let selector = doc.select_data(
                    Selector::from_query(&doc,
                        &Query::select()
                        .element(Cmp::Is(ElementType::Word))
                        .set(Cmp::Is(set.to_string()))
                        .class(Cmp::Is("PUNCTUATION".to_string()))).expect("Compiling query")
            , Recursion::Always, true);
            assert!(selector.selector().matchable());
            let mut count = 0;
            for item in selector {
                count += 1;
                assert_matches!(*item, DataType::Element(_));
            }
            assert_eq!(count, 2, "Checking whether we have the right amount of matches");
        }
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test008c_elementselector_set_class() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            let set = "https://raw.githubusercontent.com/LanguageMachines/uctodata/master/setdefinitions/tokconfig-eng.foliaset.ttl";
            let selector = doc.select(
                    Selector::from_query(&doc,
                        &Query::select()
                        .element(Cmp::Is(ElementType::Word))
                        .set(Cmp::Is(set.to_string()))
                        .class(Cmp::Is("PUNCTUATION".to_string()))).expect("Compiling query")
            , Recursion::Always);
            assert!(selector.selector().matchable());
            let mut count = 0;
            for item in selector {
                count += 1;
                assert_matches!(item.class(), Some("PUNCTUATION"));
            }
            assert_eq!(count, 2, "Checking whether we have the right amount of matches");
        }
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test008d_selector_elementgroup() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            let selector = doc.select_data(
                    Selector::from_query(&doc,
                        &Query::select()
                        .elementgroup(Cmp::Is(ElementGroup::Structure))
                        .set(Cmp::Any)
                        .class(Cmp::Any)).expect("Compiling query")
            , Recursion::Always, true);
            assert!(selector.selector.matchable());
            let mut count = 0;
            for item in selector {
                count += 1;
                assert_matches!(*item, DataType::Element(_));
            }
            assert_eq!(count, 14, "Checking whether we have the right amount of matches");
        }
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test009a_text() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            if let Some(word) = doc.get_element_by_id("example.p.1.s.2.w.4") {
                assert_eq!(&word.get_textdelimiter(true).expect("unwrapping text delimiter"), " ");
                match word.text(&TextParameters::default()) {
                    Ok(text) => assert_eq!(text, "example"),
                    Err(err) => assert!(false, format!("Obtaining text failed with error: {}",err))
                }
            } else {
                assert!(false, "word not found");
            }
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test009b_text_composed_retaintokenisation() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            if let Some(sentence) = doc.get_element_by_id("example.p.1.s.1") {
                match sentence.text(&TextParameters::default().retaintokenisation(true)) {
                    Ok(text) => assert_eq!(text, "Hello world !"),
                    Err(err) => assert!(false, format!("Obtaining text failed with error: {}",err))
                }
            } else {
                assert!(false, "word not found");
            }
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test009c_text_composed_detokenise() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            if let Some(sentence) = doc.get_element_by_id("example.p.1.s.1") {
                match sentence.text(&TextParameters::default()) {
                    Ok(text) => assert_eq!(text, "Hello world!"),
                    Err(err) => assert!(false, format!("Obtaining text failed with error: {}",err))
                }
            } else {
                assert!(false, "word not found");
            }
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test009c_text_on_span() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            if let Some(chunk) = doc.get_element_by_id("example.p.1.s.2.chunk.1") {
                assert_matches!(chunk.class(),Some("np"));
                match chunk.text(&TextParameters::default()) {
                    Ok(text) => assert_eq!(text, "an example"),
                    Err(err) => assert!(false, format!("Obtaining text on span failed with error: {}",err))
                }
            } else {
                assert!(false, "annotation not found");
            }
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test010a_get_inline_annotation() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            if let Some(word) = doc.get_element_by_id("example.p.1.s.2.w.4") {
                if let Some(pos) = word.get_annotation(AnnotationType::POS, Cmp::Any,Recursion::No) {
                    assert_matches!(pos.class(),Some("noun"));
                } else {
                    assert!(false, "annotation not found");
                }
            } else {
                assert!(false, "word not found");
            }
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test010b_get_ancestor() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            if let Some(word) = doc.get_element_by_id("example.p.1.s.2.w.4") {
                if let Some(sentence) = word.get_ancestor(ElementType::Sentence, Cmp::Any) {
                    assert_matches!(sentence.id(),Some("example.p.1.s.2"));
                } else {
                    assert!(false, "annotation not found");
                }
            } else {
                assert!(false, "word not found");
            }
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test010c_get_span_annotation() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            if let Some(word) = doc.get_element_by_id("example.p.1.s.2.w.4") {
                if let Some(chunk) = word.get_annotation(AnnotationType::CHUNKING, Cmp::Any,Recursion::No) {
                    assert_matches!(chunk.class(),Some("np"));
                } else {
                    assert!(false, "annotation not found");
                }
            } else {
                assert!(false, "word not found");
            }
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test010d_get_span_annotation_noduplicates() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            if let Some(sentence) = doc.get_element_by_id("example.p.1.s.2") {
                let mut count = 0;
                for span in sentence.get_annotations(AnnotationType::CHUNKING, Cmp::Any, Recursion::Always) {
                    count += 1;
                    assert_eq!(span.elementtype(), ElementType::Chunk);
                }
                assert_eq!(count,1, "testing whether we got the right amount of spans back");
            } else {
                assert!(false, "sentence not found");
            }
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}


#[test]
fn test011a_features() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            if let Some(word) = doc.get_element_by_id("example.p.1.s.2.w.4") {
                if let Some(pos) = word.get_annotation(AnnotationType::POS, Cmp::Any,Recursion::No) {
                    if let Some(feature) = pos.get_feature(Cmp::Is("number".to_string())) {
                        assert_matches!(feature.elementtype(), ElementType::Feature);
                        assert_matches!(feature.subset(), Some("number"));
                        assert_matches!(feature.class(), Some("singular"));
                    } else {
                        assert!(false, "feature not found");
                    }
                } else {
                    assert!(false, "annotation not found");
                }
            } else {
                assert!(false, "word not found");
            }
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test011b_feature_as_attribute() {
const TEST_FEAT_AS_ATTRIB: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<FoLiA xmlns="http://ilk.uvt.nl/folia" version="2.0" xml:id="example">
  <metadata type="native">
      <annotations>
          <token-annotation set="https://raw.githubusercontent.com/LanguageMachines/uctodata/master/setdefinitions/tokconfig-eng.foliaset.ttl">
             <annotator processor="p1" />
          </token-annotation>
          <text-annotation>
             <annotator processor="p1" />
          </text-annotation>
          <sentence-annotation>
             <annotator processor="p1" />
          </sentence-annotation>
          <pos-annotation set="adhoc">
             <annotator processor="p2" />
          </pos-annotation>
      </annotations>
      <provenance>
         <processor xml:id="p1" name="proycon" type="manual" />
      </provenance>
      <meta id="language">eng</meta>
  </metadata>
  <text xml:id="example.text">
      <s xml:id="example.p.1.s.1">
         <w xml:id="example.p.1.s.1.w.1" class="WORD">
            <t>example</t>
            <pos class="N(singular)" head="N">
                <feat subset="number" class="singular" />
            </pos>
         </w>
       </s>
  </text>
</FoLiA>"#;
    match Document::from_str(str::from_utf8(TEST_FEAT_AS_ATTRIB).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            if let Some(word) = doc.get_element_by_id("example.p.1.s.1.w.1") {
                if let Some(pos) = word.get_annotation(AnnotationType::POS, Cmp::Any,Recursion::No) {
                    if let Some(feature) = pos.get_feature(Cmp::Is("head".to_string())) {
                        assert_matches!(feature.elementtype(), ElementType::Feature);
                        assert_matches!(feature.subset(), Some("head"));
                        assert_matches!(feature.class(), Some("N"));
                    } else {
                        assert!(false, "feature not found");
                    }
                } else {
                    assert!(false, "annotation not found");
                }

            } else {
                assert!(false, "word not found");
            }
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }

}

#[test]
fn test012_spanroles() {
    match Document::from_str(str::from_utf8(EXAMPLE_DEP).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            if let Some(dependency) = doc.get_element_by_id("example.p.1.s.1.dependency.1") {
                let layer = dependency.get_parent().expect("getting parent of dependency");
                assert_eq!(layer.elementtype(),  ElementType::DependenciesLayer, "Sanity check for parent");
                assert_eq!(layer.get_parent().expect("getting parent of layer").elementtype(),  ElementType::Sentence, "Sanity check for parent of span layer");
                assert!(dependency.get_element(ElementType::DependencyDependent, Cmp::Any, Recursion::Always).is_some());
                assert!(dependency.get_element(ElementType::Headspan, Cmp::Any, Recursion::Always).is_some());
                assert_eq!(dependency.get_element(ElementType::DependencyDependent, Cmp::Any, Recursion::Always).unwrap().text(&TextParameters::default()).expect("unwrapping text of dep"), "man");
            } else {
                assert!(false, "dependency not found");
            }
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test013_textmarkup() {
    match Document::from_str(str::from_utf8(EXAMPLE_MARKUP).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            if let Some(sentence) = doc.get_element_by_id("example.p.1.s.1") {
                assert_eq!(sentence.text(&TextParameters::default()).expect("unwrapping text of dep"), "Hello world! How are you today?");
            } else {
                assert!(false, "dependency not found");
            }
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test014a_common_ancestor() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            let sentence = doc.get_element_key_by_id("example.p.1.s.1").unwrap();
            let par = doc.get_element_key_by_id("example.p.1").unwrap();
            let word1 = doc.get_element_key_by_id("example.p.1.s.1.w.1").unwrap();
            let word2 = doc.get_element_key_by_id("example.p.1.s.1.w.2").unwrap();
            let query = Query::select().elementgroup(Cmp::Is(ElementGroup::Structure));
            let common_ancestors = doc.common_ancestors(Selector::from_query(&doc,&query).expect("selector"), &[word1,word2]);
            assert_eq!(common_ancestors, &[sentence,par, 0]);
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test014b_common_ancestor() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            let par = doc.get_element_key_by_id("example.p.1").unwrap();
            let word1 = doc.get_element_key_by_id("example.p.1.s.1.w.1").unwrap();
            let word2 = doc.get_element_key_by_id("example.p.1.s.2.w.1").unwrap();
            let query = Query::select().elementgroup(Cmp::Is(ElementGroup::Structure));
            let common_ancestors = doc.common_ancestors(Selector::from_query(&doc,&query).expect("selector"), &[word1,word2]);
            assert_eq!(common_ancestors, &[par, 0]);
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test015_get_layer() {
    match Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example"), DocumentProperties::default()) {
        Ok(doc) => {
            let sentence_key = doc.get_element_key_by_id("example.p.1.s.2").unwrap();
            if let Ok(Some(_)) = doc.get_layer_key(sentence_key, AnnotationType::CHUNKING, Some("shallowsyntaxset")) {
                assert!(true);
            } else {
                assert!(false, format!("layer not found"));
            }
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test016_add_provenance() {
    let processor = Processor::new("test".to_string()).with_id("test.proc".to_owned()).autofill();
    match Document::new("example", DocumentProperties::default().with_processor(processor)) {
        Ok(mut doc) => {
            let root: ElementKey = 0;
            let sentence = doc.annotate(root,
                                        ElementData::new(ElementType::Sentence)
                                        .with_attrib(Attribute::Id("s.1".to_string()))
                                        ).expect("Adding sentence");

            doc.annotate(sentence,
                         ElementData::new(ElementType::Word)
                         .with_attrib(Attribute::Id("word.1".to_string()))
                         .with_text("hello".to_string())
                        ).expect("Adding word 1");

            let word2 = doc.annotate(sentence,
                         ElementData::new(ElementType::Word)
                         .with_attrib(Attribute::Id("word.2".to_string()))
                         .with_text("world".to_string())
                        ).expect("Adding word 2");

            //sanity tests after adding

            //check if processor is rightly associated with the document and is active
            if let Some(proc) = doc.get_processor_by_id("test.proc") {
                assert!(doc.active_processor.is_some());
                assert_eq!(doc.active_processor, proc.key);
            } else {
                assert!(false, format!("Processor was not found"));
            }


        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}
