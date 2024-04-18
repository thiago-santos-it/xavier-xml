use xavier::{PError, XmlDeserializable};

#[derive(XmlDeserializable)]
#[xml(name="object")]
struct XMLObject;

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = r#"<object/>"#;
    let mut reader = quick_xml::Reader::from_str(&xml);
    let _ =  XMLObject::from_xml(&mut reader, None)?;
    Ok(())
}

#[derive(XmlDeserializable)]
#[xml(name="outer")]
struct XMLObjectWithEmptyTag {
    #[xml(flatten)]
    pub empty: XMLObject
}

#[test]
fn deserialize_inner() -> Result<(), PError> {
    let xml = r#"<outer><object/></outer>"#;
    let mut reader = quick_xml::Reader::from_str(&xml);
    let _ =  XMLObjectWithEmptyTag::from_xml(&mut reader, None)?;
    Ok(())
}
