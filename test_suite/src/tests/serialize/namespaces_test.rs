use xavier::{from_obj, namespaces, XmlSerializable};
use xavier::serialize::namespaces::Namespaces;
use xavier::xtext;

#[derive(XmlSerializable)]
#[xml(ns="xml", name="object", case="Camel")]
struct XMLObject {
    #[xml(xmlns)]
    pub namespaces: Namespaces,
    #[xml(name="just_string")]
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}
#[test]
fn serialize() {
    let should = r#"<xml:object xmlns:xml="http://www.w3.org/XML/1998/namespace" xmlns:xhtml="http://www.w3.org/1999/xhtml"><xml:justString>Some Text</xml:justString><xml:someInt>0</xml:someInt><xml:someFloat>0</xml:someFloat></xml:object>"#;
    let xmlns = namespaces!(xml = "http://www.w3.org/XML/1998/namespace", xhtml = "http://www.w3.org/1999/xhtml");
    let xml = XMLObject { namespaces: xmlns, some_string: xtext!("Some Text"), some_int: 0, some_float: 0.0 };
    assert_eq!(from_obj(&xml), should);
}
