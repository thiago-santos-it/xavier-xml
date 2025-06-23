use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
#[xml(name="object", case="Camel")]
struct XMLObject {
    #[xml(attribute, name="some_string")]
    pub other_string: String,
    #[xml(attribute)]
    pub some_int: i32
}

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = r#"<object some_string="Some text" some_int="11"/>"#;
    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.other_string, "Some text");
    assert_eq!(obj.some_int, 11);
    Ok(())
}

#[derive(XmlDeserializable, Debug)]
#[xml(tag, name="child")]
struct Child {
    #[xml(attribute, name="attr")]
    pub attribute: String,
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="object", case="Camel")]
struct XMLObjectSelfClosingChild {
    pub child: Child
}

#[test]
fn deserialize_child() -> Result<(), PError> {
    let xml = r#"<object><child attr="Some text"/></object>"#;
    let obj: XMLObjectSelfClosingChild = from_xml(&xml)?;
    assert_eq!(obj.child.attribute, "Some text");
    Ok(())
}

#[derive(XmlDeserializable, Debug)]
#[xml(tag, name="child")]
struct ChildEmpty;

#[derive(XmlDeserializable, Debug)]
#[xml(name="object", case="Camel")]
struct XMLObjectSelfClosingChildEmpty {
    pub child: Option<ChildEmpty>
}

#[test]
fn deserialize_child_empty() -> Result<(), PError> {
    let xml = r#"<object><child/></object>"#;
    let obj: XMLObjectSelfClosingChildEmpty = from_xml(&xml)?;
    assert_eq!(obj.child.is_some(), true);

    let xml = r#"<object></object>"#;
    let obj: XMLObjectSelfClosingChildEmpty = from_xml(&xml)?;
    assert_eq!(obj.child.is_none(), true);
    Ok(())
}