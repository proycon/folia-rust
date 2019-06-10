use libfolia_rs::folia;

#[test]
fn instantiate() {
    if let Ok(doc) = folia::Document::new("example", folia::BodyType::Text) {
        assert_eq!(doc.id(), "example");
    } else {
        assert!(false);
    }
}
