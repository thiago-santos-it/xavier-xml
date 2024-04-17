use xavier::{PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
struct XMLObject {
    #[xml(attribute)]
    pub some_string: String,
    #[xml(attribute)]
    pub some_int: i32,
    pub some_float: f32
}

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = r#"
    <XMLObject some_string="Some text" some_int="11">
        <some_float>10</some_float>
    </XMLObject>"#;
    let mut reader = quick_xml::Reader::from_str(&xml);
    let obj = XMLObject::from_xml(&mut reader, None)?;
    assert_eq!(obj.some_string, "Some text");
    assert_eq!(obj.some_int, 11);
    assert_eq!(obj.some_float, 10.0);
    Ok(())
}
