use std::str;
use folia;
use folia::store::Store;

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
      </annotations>
      <provenance>
         <processor xml:id="p1" name="proycon" type="manual" />
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
            <t>World</t>
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
      </s>
    </p>
  </text>
</FoLiA>"#;

#[test]
fn test001_instantiate() {
    match folia::Document::new("example", folia::BodyType::Text) {
        Ok(doc) => {
            assert_eq!(doc.id(), "example");
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}



#[test]
fn test002_append() {
    match folia::Document::new("example", folia::BodyType::Text) {
        Ok(mut doc) => {
            let root: folia::ElementKey = 0;
            let sentence = doc.add_to(root,
                                            folia::FoliaElement::new(folia::ElementType::Sentence)
                                                                .with_attrib(folia::Attribute::Id("s.1".to_string())) ).expect("Obtaining sentence");
            doc.add_to(sentence,
                             folia::FoliaElement::new(folia::ElementType::Word)
                                                 .with(folia::DataType::text("hello"))).expect("Adding word 1");
            doc.add_to(sentence,
                             folia::FoliaElement::new(folia::ElementType::Word)
                                                 .with(folia::DataType::text("world"))).expect("Adding word 2");
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}

#[test]
fn test003_parse() {
    match folia::Document::from_str(str::from_utf8(EXAMPLE).expect("decoding utf-8")) {
        Ok(doc) => {
            assert_eq!(doc.id(), "example", "ID check");
            assert_eq!(doc.provenancestore.chain.len(), 1, "Sanity check of provenance chain (count only)");
        },
        Err(err) => {
            assert!(false, format!("Instantiation failed with error: {}",err));
        }
    }
}


#[test]
fn test004_get_word_from_index() {
    match folia::Document::from_str(str::from_utf8(EXAMPLE).expect("decoding utf-8")) {
        Ok(doc) => {
            if let Some(word) = doc.elementstore.get_by_id("example.p.1.s.1.w.1") {
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
    match folia::Document::from_str(str::from_utf8(EXAMPLE).expect("decoding utf-8")) {
        Ok(doc) => {
            if let Some(word) = doc.elementstore.get_by_id("example.p.1.s.1.w.1") {
                let set = word.decoded_set(&doc.declarationstore);
                assert_eq!(set.expect("Unwrapping set"), "https://raw.githubusercontent.com/LanguageMachines/uctodata/master/setdefinitions/tokconfig-eng.foliaset.ttl");
                let class = word.decoded_class(&doc.declarationstore);
                assert_eq!(class.expect("Unwrapping class"), "WORD");
                let processor = word.decoded_processor(&doc.provenancestore);
                assert_eq!(processor.expect("Unwrapping class"), "p1");
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
fn test006_serialise() {
    match folia::Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example")) {
        Ok(doc) => {
            match doc.xml(0) {
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
fn test007_metadata() {
    match folia::Document::from_str(str::from_utf8(EXAMPLE).expect("conversion from utf-8 of example")) {
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
