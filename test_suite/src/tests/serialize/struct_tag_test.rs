use xavier::{from_obj, XmlSerializable};
use xavier::encode;

#[derive(XmlSerializable)]
#[xml(tag, name="child")]
struct Child {
    #[xml(attribute, name="attr")]
    pub attribute: String,
    #[xml(value)]
    pub value: String,
}
#[derive(XmlSerializable)]
#[xml(name="object", case="Camel")]
struct XMLObject {
    pub field_a: String,
    #[xml(tree)]
    pub child: Child
}

#[test]
fn serialize() {
    let should = r#"<object><fieldA>Some Text</fieldA><child attr="Attr Value">Other value</child></object>"#;
    let xml = XMLObject { field_a: encode!("Some Text"), child: Child { attribute: "Attr Value".to_string(), value: encode!("Other value")} };
    assert_eq!(from_obj(&xml), should);
}
