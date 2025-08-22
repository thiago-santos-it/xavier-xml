use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};
use xavier::encode;

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct XMLObject {
    pub type_i8: i8,
    pub type_i16: i16,
    pub type_i32: i32,
    pub type_i64: i64,
    pub type_i128: i128,
    pub type_u8: u8,
    pub type_u16: u16,
    pub type_u32: u32,
    pub type_u64: u64,
    pub type_u128: u128,
    pub type_isize: isize,
    pub type_usize: usize,
    pub type_string: String,
    pub type_f32: f32,
    pub type_f64: f64,
    pub type_bool: bool,
    pub type_char: char,
}

// ===== DESERIALIZAÇÃO =====

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = r#"
    <XMLObject>
        <type_i8>42</type_i8>
        <type_i16>-100</type_i16>
        <type_i32>1000</type_i32>
        <type_i64>-10000</type_i64>
        <type_i128>123456789012345678901234567890</type_i128>
        <type_u8>255</type_u8>
        <type_u16>65535</type_u16>
        <type_u32>4294967295</type_u32>
        <type_u64>18446744073709551615</type_u64>
        <type_u128>340282366920938463463374607431768211455</type_u128>
        <type_isize>-42</type_isize>
        <type_usize>42</type_usize>
        <type_string>Hello, XML!</type_string>
        <type_f32>3.14</type_f32>
        <type_f64>3.14159265358979323846264338327950288</type_f64>
        <type_bool>true</type_bool>
        <type_char>A</type_char>
    </XMLObject>"#;
    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.type_i8, 42);
    assert_eq!(obj.type_i16, -100);
    assert_eq!(obj.type_i32, 1000);
    assert_eq!(obj.type_i64, -10000);
    assert_eq!(obj.type_i128, 123456789012345678901234567890);
    assert_eq!(obj.type_u8, 255);
    assert_eq!(obj.type_u16, 65535);
    assert_eq!(obj.type_u32, 4294967295);
    assert_eq!(obj.type_u64, 18446744073709551615);
    assert_eq!(obj.type_u128, 340282366920938463463374607431768211455);
    assert_eq!(obj.type_isize, -42);
    assert_eq!(obj.type_usize, 42);
    assert_eq!(obj.type_string, "Hello, XML!".to_string());
    assert_eq!(obj.type_f32, 3.14);
    assert_eq!(obj.type_f64, std::f64::consts::PI);
    assert_eq!(obj.type_bool, true);
    assert_eq!(obj.type_char, 'A');

    Ok(())
}

#[test]
fn deserialize_edge_cases() -> Result<(), PError> {
    let xml = r#"
    <XMLObject>
        <type_i8>-128</type_i8>
        <type_i16>-32768</type_i16>
        <type_i32>-2147483648</type_i32>
        <type_i64>-9223372036854775808</type_i64>
        <type_i128>-170141183460469231731687303715884105728</type_i128>
        <type_u8>0</type_u8>
        <type_u16>0</type_u16>
        <type_u32>0</type_u32>
        <type_u64>0</type_u64>
        <type_u128>0</type_u128>
        <type_isize>-9223372036854775808</type_isize>
        <type_usize>0</type_usize>
        <type_string></type_string>
        <type_f32>0.0</type_f32>
        <type_f64>0.0</type_f64>
        <type_bool>false</type_bool>
        <type_char> </type_char>
    </XMLObject>"#;
    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.type_i8, -128);
    assert_eq!(obj.type_i16, -32768);
    assert_eq!(obj.type_i32, -2147483648);
    assert_eq!(obj.type_i64, -9223372036854775808);
    assert_eq!(obj.type_i128, -170141183460469231731687303715884105728);
    assert_eq!(obj.type_u8, 0);
    assert_eq!(obj.type_u16, 0);
    assert_eq!(obj.type_u32, 0);
    assert_eq!(obj.type_u64, 0);
    assert_eq!(obj.type_u128, 0);
    assert_eq!(obj.type_isize, -9223372036854775808);
    assert_eq!(obj.type_usize, 0);
    assert_eq!(obj.type_string, "".to_string());
    assert_eq!(obj.type_f32, 0.0);
    assert_eq!(obj.type_f64, 0.0);
    assert_eq!(obj.type_bool, false);
    assert_eq!(obj.type_char, ' ');

    Ok(())
}

#[test]
fn serialize_all_types() {
    let xml = XMLObject {
        type_string: encode!("Test String"),
        type_i8: -128,
        type_i16: -32768,
        type_i32: -2147483648,
        type_i64: -9223372036854775808,
        type_i128: -170141183460469231731687303715884105728,
        type_u8: 255,
        type_u16: 65535,
        type_u32: 4294967295,
        type_u64: 18446744073709551615,
        type_u128: 340282366920938463463374607431768211455,
        type_isize: -42,
        type_usize: 42,
        type_f32: 3.14159,
        type_f64: 3.141592653589793,
        type_bool: true,
        type_char: 'A',
    };
    
    let result = from_obj(&xml);
    assert!(!result.is_empty());
    assert!(result.contains("<type_string>Test String</type_string>"));
    assert!(result.contains("<type_i8>-128</type_i8>"));
    assert!(result.contains("<type_i16>-32768</type_i16>"));
    assert!(result.contains("<type_i32>-2147483648</type_i32>"));
    assert!(result.contains("<type_i64>-9223372036854775808</type_i64>"));
    assert!(result.contains("<type_u8>255</type_u8>"));
    assert!(result.contains("<type_u16>65535</type_u16>"));
    assert!(result.contains("<type_u32>4294967295</type_u32>"));
    assert!(result.contains("<type_u64>18446744073709551615</type_u64>"));
    assert!(result.contains("<type_f32>3.14159</type_f32>"));
    assert!(result.contains("<type_f64>3.141592653589793</type_f64>"));
    assert!(result.contains("<type_bool>true</type_bool>"));
    assert!(result.contains("<type_char>A</type_char>"));
}

#[test]
fn serialize_edge_cases() {
    let xml = XMLObject {
        type_string: encode!(""),
        type_i8: i8::MIN,
        type_i16: i16::MIN,
        type_i32: i32::MIN,
        type_i64: i64::MIN,
        type_i128: i128::MIN,
        type_u8: u8::MAX,
        type_u16: u16::MAX,
        type_u32: u32::MAX,
        type_u64: u64::MAX,
        type_u128: u128::MAX,
        type_isize: isize::MIN,
        type_usize: usize::MAX,
        type_f32: f32::NEG_INFINITY,
        type_f64: f64::NEG_INFINITY,
        type_bool: false,
        type_char: '\0',
    };
    
    let result = from_obj(&xml);
    assert!(!result.is_empty());
    assert!(result.contains("<type_string></type_string>"));
    assert!(result.contains(&format!("<type_i8>{}</type_i8>", i8::MIN)));
    assert!(result.contains(&format!("<type_i16>{}</type_i16>", i16::MIN)));
    assert!(result.contains(&format!("<type_i32>{}</type_i32>", i32::MIN)));
    assert!(result.contains(&format!("<type_i64>{}</type_i64>", i64::MIN)));
    assert!(result.contains(&format!("<type_u8>{}</type_u8>", u8::MAX)));
    assert!(result.contains(&format!("<type_u16>{}</type_u16>", u16::MAX)));
    assert!(result.contains(&format!("<type_u32>{}</type_u32>", u32::MAX)));
    assert!(result.contains(&format!("<type_u64>{}</type_u64>", u64::MAX)));
    assert!(result.contains("<type_f32>-inf</type_f32>"));
    assert!(result.contains("<type_f64>-inf</type_f64>"));
    assert!(result.contains("<type_bool>false</type_bool>"));
    assert!(result.contains(&format!("<type_char>{}</type_char>", '\0')));
}
