use xavier::{from_obj, XmlSerializable};

#[derive(XmlSerializable)]
#[xml(name="object")]
struct XMLObject;

#[test]
fn serialize() {
    let should = r#"<object></object>"#;
    let xml = XMLObject;
    assert_eq!(from_obj(&xml), should);
}
