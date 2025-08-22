use xavier::{from_obj, XmlSerializable};
use xavier::encode;

#[derive(XmlSerializable)]
struct XMLObject {
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}

#[test]
fn serialize() {
    let should = r#"<XMLObject><some_string>Some Text</some_string><some_int>0</some_int><some_float>0</some_float></XMLObject>"#;
    let xml = XMLObject { some_string: encode!("Some Text"), some_int: 0, some_float: 0.0 };
    assert_eq!(from_obj(&xml), should);
}


#[test]
fn serialize_with_special_characters() {
    let xml = XMLObject {
        some_string: encode!("Text with & < > \" ' characters"),
        some_int: -42,
        some_float: 3.14159
    };
    let result = from_obj(&xml);
    assert!(!result.is_empty());
    assert!(result.contains("Text with &amp; &lt; &gt; &quot; &apos; characters"));
    assert!(result.contains("<some_int>-42</some_int>"));
    assert!(result.contains("<some_float>3.14159</some_float>"));
}

#[test]
fn serialize_edge_cases() {
    let xml = XMLObject {
        some_string: encode!(""),
        some_int: i32::MIN,
        some_float: f32::NEG_INFINITY
    };
    let result = from_obj(&xml);
    assert!(!result.is_empty());
    assert!(result.contains("<some_string></some_string>"));
    assert!(result.contains(&format!("<some_int>{}</some_int>", i32::MIN)));
    assert!(result.contains("<some_float>-inf</some_float>"));
}