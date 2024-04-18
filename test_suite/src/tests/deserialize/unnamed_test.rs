use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable)]
#[xml(ns="a", name="object")]
pub struct XMLObject(String);

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = r#"<a:object>Some Text<!--Ignore--></a:object>"#;
    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.0, "Some Text");
    Ok(())
}
