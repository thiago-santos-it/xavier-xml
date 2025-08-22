use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
#[xml(name="object", case="Camel", prefix="xml_", suffix="Item", no_suffix, no_prefix)]
struct XMLObject {
    #[xml(name="just_string")]
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = r#"
    <object>
        <xmlJustStringItem>Some Text</xmlJustStringItem>
        <xmlSomeIntItem>10</xmlSomeIntItem>
        <xmlSomeFloatItem>11</xmlSomeFloatItem>
    </object>"#;
    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.some_string, "Some Text");
    assert_eq!(obj.some_int, 10);
    assert_eq!(obj.some_float, 11.0);
    Ok(())
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="object", case="Camel", prefix="xml_", suffix="Item", no_suffix, no_prefix)]
struct XMLObjectIgnoreCase {
    #[xml(name="just_string", ignore_case)]
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}

#[test]
fn ignore_case() -> Result<(), PError> {
    let xml = r#"
    <object>
        <xml_just_stringItem>Some Text</xml_just_stringItem>
        <xmlSomeIntItem>10</xmlSomeIntItem><xmlSomeFloatItem>11</xmlSomeFloatItem>
    </object>"#;

    let obj: XMLObjectIgnoreCase = from_xml(&xml)?;
    assert_eq!(obj.some_string, "Some Text");
    assert_eq!(obj.some_int, 10);
    assert_eq!(obj.some_float, 11.0);
    Ok(())
}
