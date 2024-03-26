use xavier::{from_obj, XmlSerializable};
use xavier::xtext;

#[derive(XmlSerializable)]
struct XMLObject {
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}
#[test]
fn serialize() {
    let should = r#"<XMLObject><some_string>Some Text</some_string><some_int>0</some_int><some_float>0</some_float></XMLObject>"#;
    let xml = XMLObject { some_string: xtext!("Some Text"), some_int: 0, some_float: 0.0 };
    assert_eq!(from_obj(&xml), should);
}
