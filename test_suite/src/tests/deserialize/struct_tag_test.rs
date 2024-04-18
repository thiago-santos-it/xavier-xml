use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
#[xml(tag, name="child")]
struct Child {
    #[xml(attribute, name="attr")]
    pub attribute: String,
    #[xml(value)]
    pub value: String,
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="object", case="Camel")]
struct XMLObject {
    pub field_a: String,
    #[xml(tree)]
    pub child: Child
}

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = r#"<object><fieldA>Some Text</fieldA><child attr="Attr Value">Other value</child></object>"#;
    let obj: XMLObject = from_xml(&xml)?;
    println!("{:?}", obj);
    Ok(())
}
