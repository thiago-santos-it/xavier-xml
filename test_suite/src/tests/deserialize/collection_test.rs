use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug, PartialEq)]
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

#[test]
fn deserialize_empty_collection() -> Result<(), PError> {
    let xml = r#"
    <object>
        <fieldA>Some Text</fieldA>
        <children>
        </children>
    </object>"#;

    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.field_a, "Some Text");
    assert_eq!(obj.children.len(), 0);
    Ok(())
}

#[test]
fn deserialize_option_empty_collection() -> Result<(), PError> {
    let xml = r#"
    <object>
        <fieldA>Some Text</fieldA>
        <children>
        </children>
    </object>"#;

    let obj: XMLObjectOption = from_xml(&xml)?;
    assert_eq!(obj.field_a, "Some Text");
    let children = obj.children.unwrap();
    assert_eq!(children.len(), 0);
    Ok(())
}

#[test]
fn deserialize_option_missing_collection() -> Result<(), PError> {
    let xml = r#"
    <object>
        <fieldA>Some Text</fieldA>
    </object>"#;

    let obj: XMLObjectOption = from_xml(&xml)?;
    assert_eq!(obj.field_a, "Some Text");
    assert_eq!(obj.children, None);
    Ok(())
}

#[test]
fn deserialize_large_collection() -> Result<(), PError> {
    let mut xml = r#"
    <object>
        <fieldA>Large Collection Test</fieldA>
        <children>
    "#.to_string();
    
    for i in 0..100 {
        xml.push_str(&format!(r#"
            <my_child>
                <child_field_a>Child {} Value</child_field_a>
            </my_child>"#, i));
    }
    
    xml.push_str(r#"
        </children>
    </object>"#);

    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.field_a, "Large Collection Test");
    assert_eq!(obj.children.len(), 100);
    assert_eq!(obj.children[0].child_field_a, "Child 0 Value");
    assert_eq!(obj.children[99].child_field_a, "Child 99 Value");
    Ok(())
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="nested_child")]
struct NestedChild {
    pub nested_field: String,
    pub sub_children: Vec<Child>,
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="nested_object")]
struct NestedObject {
    pub field_a: String,
    pub nested_children: Vec<NestedChild>,
}

#[test]
fn deserialize_nested_collections() -> Result<(), PError> {
    let xml = r#"
    <nested_object>
        <field_a>Nested Test</field_a>
        <nested_children>
            <nested_child>
                <nested_field>Nested 1</nested_field>
                <sub_children>
                    <my_child>
                        <child_field_a>Sub Child 1A</child_field_a>
                    </my_child>
                    <my_child>
                        <child_field_a>Sub Child 1B</child_field_a>
                    </my_child>
                </sub_children>
            </nested_child>
            <nested_child>
                <nested_field>Nested 2</nested_field>
                <sub_children>
                    <my_child>
                        <child_field_a>Sub Child 2A</child_field_a>
                    </my_child>
                </sub_children>
            </nested_child>
        </nested_children>
    </nested_object>"#;

    let obj: NestedObject = from_xml(&xml)?;
    assert_eq!(obj.field_a, "Nested Test");
    assert_eq!(obj.nested_children.len(), 2);
    assert_eq!(obj.nested_children[0].nested_field, "Nested 1");
    assert_eq!(obj.nested_children[0].sub_children.len(), 2);
    assert_eq!(obj.nested_children[0].sub_children[0].child_field_a, "Sub Child 1A");
    assert_eq!(obj.nested_children[0].sub_children[1].child_field_a, "Sub Child 1B");
    assert_eq!(obj.nested_children[1].nested_field, "Nested 2");
    assert_eq!(obj.nested_children[1].sub_children.len(), 1);
    assert_eq!(obj.nested_children[1].sub_children[0].child_field_a, "Sub Child 2A");
    Ok(())
}