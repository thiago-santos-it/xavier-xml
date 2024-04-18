use xavier::{from_obj, XmlSerializable};

#[derive(XmlSerializable)]
#[xml(name="object")]
struct XMLObject;

#[test]
fn serialize() {
    let should = r#"<object/>"#;
    let xml = XMLObject;
    assert_eq!(from_obj(&xml), should);
}

#[derive(XmlSerializable)]
#[xml(name="outer")]
struct XMLObjectWithEmptyTag {
    #[xml(flatten)]
    pub empty: XMLObject
}

#[test]
fn serialize_inner() {
    let should = r#"<outer><object/></outer>"#;
    let xml = XMLObjectWithEmptyTag { empty : XMLObject };
    assert_eq!(from_obj(&xml), should);
}
