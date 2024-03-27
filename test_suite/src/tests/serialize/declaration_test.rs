use xavier::{from_obj, XmlSerializable};
use xavier::xtext;

#[derive(XmlSerializable)]
#[header(version="1.0" encoding="UTF-8" standalone="no")]
#[xml(name="xml")]
struct XMLObject {
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}

#[test]
fn serialize() {
    let should = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?><xml><some_string>Some Text</some_string><some_int>0</some_int><some_float>0</some_float></xml>"#;
    let xml = XMLObject { some_string: xtext!("Some Text"), some_int: 0, some_float: 0.0 };
    assert_eq!(from_obj(&xml), should);
}
