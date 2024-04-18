use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
#[xml(name="my_child")]
struct Child {
    pub child_field_a: String,
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="object", case="Camel")]
struct XMLObject {
    pub field_a: String,
    pub children: Vec<Child>
}

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = r#"
    <object>
        <fieldA>Some Text</fieldA>
        <children>
            <my_child>
                <child_field_a>Other value A</child_field_a>
            </my_child>
            <my_child>
                <child_field_a>Other value B</child_field_a>
            </my_child>
        </children>
    </object>"#;

    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.field_a, "Some Text");
    assert_eq!(obj.children[0].child_field_a, "Other value A");
    assert_eq!(obj.children[1].child_field_a, "Other value B");
    Ok(())
}


#[derive(XmlDeserializable, Debug)]
#[xml(name="object", case="Camel")]
struct XMLObjectOption {
    pub field_a: String,
    pub children: Option<Vec<Child>>
}

#[test]
fn deserialize_option() -> Result<(), PError> {
    let xml = r#"
    <object>
        <fieldA>Some Text</fieldA>
        <children>
            <my_child>
                <child_field_a>Other value A</child_field_a>
            </my_child>
            <my_child>
                <child_field_a>Other value B</child_field_a>
            </my_child>
        </children>
    </object>"#;

    let obj: XMLObjectOption = from_xml(&xml)?;
    assert_eq!(obj.field_a, "Some Text");
    let children = obj.children.unwrap();
    assert_eq!(children[0].child_field_a, "Other value A");
    assert_eq!(children[1].child_field_a, "Other value B");
    Ok(())
}