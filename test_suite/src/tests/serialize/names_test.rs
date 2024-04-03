use xavier::{from_obj, XmlSerializable};
use xavier::encode;

#[derive(XmlSerializable)]
#[xml(name="object", case="Camel", prefix="xml_", suffix="Item", no_suffix, no_prefix)]
struct XMLObject {
    #[xml(name="just_string")]
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}

#[test]
fn serialize() {
    let should = r#"<object><xmlJustStringItem>Some Text</xmlJustStringItem><xmlSomeIntItem>0</xmlSomeIntItem><xmlSomeFloatItem>0</xmlSomeFloatItem></object>"#;
    let xml = XMLObject { some_string: encode!("Some Text"), some_int: 0, some_float: 0.0 };
    assert_eq!(from_obj(&xml), should);
}

#[derive(XmlSerializable)]
#[xml(name="object", case="Camel", prefix="xml_", suffix="Item", no_suffix, no_prefix)]
struct XMLObjectIgnoreCase {
    #[xml(name="just_string", ignore_case)]
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}

#[test]
fn ignore_case() {
    let should = r#"<object><xml_just_stringItem>Some Text</xml_just_stringItem><xmlSomeIntItem>0</xmlSomeIntItem><xmlSomeFloatItem>0</xmlSomeFloatItem></object>"#;
    let xml = XMLObjectIgnoreCase { some_string: encode!("Some Text"), some_int: 0, some_float: 0.0 };
    assert_eq!(from_obj(&xml), should);
}
