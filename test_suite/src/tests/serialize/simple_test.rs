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

#[derive(XmlSerializable)]
struct XMLObjectWithAllTypes {
    pub string_field: String,
    pub int_8: i8,
    pub int_16: i16,
    pub int_32: i32,
    pub int_64: i64,
    pub int_128: i128,
    pub uint_8: u8,
    pub uint_16: u16,
    pub uint_32: u32,
    pub uint_64: u64,
    pub uint_128: u128,
    pub float_32: f32,
    pub float_64: f64,
    pub bool_field: bool,
    pub char_field: char,
}

#[test]
fn serialize_all_types() {
    let xml = XMLObjectWithAllTypes {
        string_field: encode!("Test String"),
        int_8: -128,
        int_16: -32768,
        int_32: -2147483648,
        int_64: -9223372036854775808,
        int_128: -170141183460469231731687303715884105728,
        uint_8: 255,
        uint_16: 65535,
        uint_32: 4294967295,
        uint_64: 18446744073709551615,
        uint_128: 340282366920938463463374607431768211455,
        float_32: 3.14159,
        float_64: 3.141592653589793,
        bool_field: true,
        char_field: 'A',
    };
    
    let result = from_obj(&xml);
    assert!(!result.is_empty());
    assert!(result.contains("<string_field>Test String</string_field>"));
    assert!(result.contains("<int_8>-128</int_8>"));
    assert!(result.contains("<int_16>-32768</int_16>"));
    assert!(result.contains("<int_32>-2147483648</int_32>"));
    assert!(result.contains("<int_64>-9223372036854775808</int_64>"));
    assert!(result.contains("<uint_8>255</uint_8>"));
    assert!(result.contains("<uint_16>65535</uint_16>"));
    assert!(result.contains("<uint_32>4294967295</uint_32>"));
    assert!(result.contains("<uint_64>18446744073709551615</uint_64>"));
    assert!(result.contains("<float_32>3.14159</float_32>"));
    assert!(result.contains("<float_64>3.141592653589793</float_64>"));
    assert!(result.contains("<bool_field>true</bool_field>"));
    assert!(result.contains("<char_field>A</char_field>"));
}
