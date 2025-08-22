use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable)]
#[xml(ns="xml", name="object", case="Camel")]
struct XMLObject {
    #[xml(name="just_string")]
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}
#[test]
fn serialize() -> Result<(), PError> {

    let xml = r#"
    <xml:object
            xmlns:xml="http://www.w3.org/XML/1998/namespace"
            xmlns:xhtml="http://www.w3.org/1999/xhtml">
        <xml:justString>Some Text</xml:justString>
        <xml:someInt>10</xml:someInt>
        <xml:someFloat>11.0</xml:someFloat>
    </xml:object>"#;

    let obj: XMLObject =  from_xml(&xml)?;
    assert_eq!(obj. some_string, "Some Text");
    assert_eq!(obj.some_int, 10);
    assert_eq!(obj.some_float, 11.0);
    Ok(())
}