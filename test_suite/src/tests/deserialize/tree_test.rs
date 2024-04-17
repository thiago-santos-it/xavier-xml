use xavier::{PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
#[xml(name="my_child")]
struct Child {
    pub child_field_a: String,
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="object", case="Camel")]
struct XMLObject {
    pub field_a: String,
    pub child: Child
}

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = r#"<object><fieldA>Some Text</fieldA><child><my_child><child_field_a>Other value</child_field_a></my_child></child></object>"#;
    let mut reader = quick_xml::Reader::from_str(&xml);
    let obj =  XMLObject::from_xml(&mut reader, None)?;
    assert_eq!(obj.field_a, "Some Text");
    assert_eq!(obj.child.child_field_a, "Other value");
    Ok(())
}


#[derive(XmlDeserializable, Debug)]
#[xml(name="object", case="Camel")]
struct XMLObjectTree {
    pub field_a: String,
    #[xml(tree)]
    pub child_a: Child,
    #[xml(tree)]
    pub child_b: Child
}

#[test]
fn deserialize_tree() -> Result<(), PError> {
    let xml = r#"
    <object>
        <fieldA>Some Text</fieldA>
        <my_child>
            <child_field_a>Other value A</child_field_a>
        </my_child>
        <my_child>
            <child_field_a>Other value B</child_field_a>
        </my_child>
    </object>"#;

    let mut reader = quick_xml::Reader::from_str(&xml);
    let obj =  XMLObjectTree::from_xml(&mut reader, None)?;
    assert_eq!(obj.child_a.child_field_a, "Other value A");
    assert_eq!(obj.child_b.child_field_a, "Other value B");
    assert_eq!(obj.field_a, "Some Text");
    Ok(())
}
