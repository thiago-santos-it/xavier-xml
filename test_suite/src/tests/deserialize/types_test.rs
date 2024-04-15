use xavier::{PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
struct XMLObject {
    pub type_i8: u8,
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

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = r#"<XMLObject>
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
                        </XMLObject>
                        "#;
    let mut reader = quick_xml::Reader::from_str(&xml);
    let obj =  XMLObject::from_xml(&mut reader, None)?;
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
