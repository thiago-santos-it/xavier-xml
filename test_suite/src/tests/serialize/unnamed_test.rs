use xavier::{from_obj, XmlSerializable};
use xavier::encode;

#[derive(XmlSerializable)]
#[xml(ns="a", name="object")]
pub struct XMLObject(String);

#[test]
fn serialize() {
    let should = r#"<a:object>Some Text</a:object>"#;
    let xml = XMLObject(encode!("Some Text"));
    assert_eq!(from_obj(&xml), should);
}
