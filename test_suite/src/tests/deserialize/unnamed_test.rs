use xavier::{PError, XmlDeserializable};

#[derive(XmlDeserializable)]
#[xml(ns="a", name="object")]
pub struct XMLObject(String);

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = r#"<a:object>Some Text<!--Ignore--></a:object>"#;
    let mut reader = quick_xml::Reader::from_str(&xml);
    let obj =  XMLObject::from_xml(&mut reader, None)?;
    assert_eq!(obj.0, "Some Text");
    Ok(())
}
