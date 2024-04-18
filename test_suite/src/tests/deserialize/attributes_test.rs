use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
struct XMLObject {
    #[xml(attribute, name="some_string")]
    pub other_string: String,
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
    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.other_string, "Some text");
    assert_eq!(obj.some_int, 11);
    assert_eq!(obj.some_float, 10.0);
    Ok(())
}
