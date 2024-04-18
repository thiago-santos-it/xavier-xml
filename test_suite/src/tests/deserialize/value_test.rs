use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
#[xml(tag, name="child")]
struct Child {
    #[xml(value)]
    pub value: String,
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="object", case="Camel")]
struct XMLObject {
    pub field_a: String,
    #[xml(tree)]
    pub child: Child,
    #[xml(value)]
    pub value_a: String,
    #[xml(value)]
    pub value_b: String,
}

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = r#"<object><fieldA>Some Text</fieldA><child attr="Attr Value">Other value</child>Something</object>"#;
    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.field_a, "Some Text");
    assert_eq!(obj.value_a, "Something");
    assert_eq!(obj.value_a, obj.value_b);
    assert_eq!(obj.child.value, "Other value");
    Ok(())
}
