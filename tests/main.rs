use libfolia_rs::folia;

#[test]
fn instantiate() {
    if let Ok(doc) = folia::Document::new("example", folia::BodyType::Text) {
        assert_eq!(doc.id(), "example");
    } else {
        assert!(false);
    }
}

#[test]
fn append() {
    if let Ok(doc) = folia::Document::new("example", folia::BodyType::Text) {
        let mut body = doc.get_mut_body().unwrap();
        let attribs = vec![ folia::Attribute::Id("s.1".to_string()) ];
        body.append(folia::ElementType::Sentence, Some(attribs), None  );
    } else {
        assert!(false);
    }
}
