use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug, PartialEq)]
#[xml(name="object")]
struct XMLObject;

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = r#"<object/>"#;
    let _: XMLObject = from_xml(&xml)?;
    Ok(())
}

#[derive(XmlDeserializable)]
#[xml(name="outer")]
struct XMLObjectWithEmptyTag {
    #[xml(flatten)]
    pub empty: Option<XMLObject>
}

#[test]
fn deserialize_inner() -> Result<(), PError> {
    let xml = r#"<outer><object/></outer>"#;
    let obj: XMLObjectWithEmptyTag = from_xml(&xml)?;
    assert_ne!(obj.empty, None);
    Ok(())
}
