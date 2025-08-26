use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct Child {
    pub child_field_a: String,
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct XMLObject {
    pub field_a: String,
    pub children: Vec<Child>
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct ChildRecursive {
    #[xml(attribute, name="attr")]
    pub attribute: String,
    pub child_field_a: String,
    #[xml(tree)]
    pub inner: Option<Box<ChildRecursive>>
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct XMLObjectRecursion {
    pub field_a: String,
    #[xml(tree)]
    pub child: ChildRecursive
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct ChildFlat {
    #[xml(attribute, name="attr")]
    pub attribute: String,
    pub child_field_a: String,
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct XMLObjectFlat {
    pub field_a: String,
    #[xml(tree)]
    pub child_a: ChildFlat,
    #[xml(tree)]
    pub child_b: ChildFlat
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
#[xml(name="inner")]
struct ChildA {
    #[xml(attribute, name="attr")]
    pub attribute: String,
    pub child_field_a: String,
    #[xml(tree)]
    pub inner: ChildB
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
#[xml(name="inner")]
struct ChildB {
    #[xml(attribute, name="attr")]
    pub attribute: String,
    pub child_field_a: String,
    #[xml(tree)]
    pub inner: ChildC
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
#[xml(name="inner")]
struct ChildC {
    #[xml(attribute, name="attr")]
    pub attribute: String,
    pub child_field_a: String,
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
#[xml(name="child")]
struct ChildRoot {
    #[xml(tree)]
    pub inner: ChildA
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct XMLObjectTree {
    pub field_a: String,
    #[xml(tree)]
    pub child: ChildRoot
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct ChildSibling {
    pub child_field_a: String
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct XMLObjectSibling {
    pub field_a: String,
    #[xml(tree)]
    pub siblings: Vec<ChildSibling>
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct ChildSiblingA {
    pub child_field_a: String
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct ChildSiblingB {
    pub child_field_a: String
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct XMLObjectSiblings {
    pub field_a: String,
    #[xml(tree)]
    pub siblings_a: Vec<ChildSiblingA>,
    #[xml(tree)]
    pub siblings_b: Vec<ChildSiblingB>
}

#[test]
fn collection_basic() -> Result<(), PError> {
    let xml = r#"
    <XMLObject>
        <field_a>Some Text</field_a>
        <children>
            <Child>
                <child_field_a>Other value A</child_field_a>
            </Child>
            <Child>
                <child_field_a>Other value B</child_field_a>
            </Child>
        </children>
    </XMLObject>"#;

    let obj: XMLObject = from_xml(&xml)?;

    assert_eq!(obj.field_a, "Some Text");
    assert_eq!(obj.children[0].child_field_a, "Other value A");
    assert_eq!(obj.children[1].child_field_a, "Other value B");

    let test_data = XMLObject {
        field_a: "Test Collection".to_string(),
        children: vec![
            Child { child_field_a: "Child 1".to_string() },
            Child { child_field_a: "Child 2".to_string() },
            Child { child_field_a: "Child 3".to_string() },
        ],
    };

    let result = from_obj(&test_data);
    assert!(!result.is_empty());
    assert!(result.contains("<field_a>Test Collection</field_a>"));
    assert!(result.contains("<child_field_a>Child 1</child_field_a>"));
    assert!(result.contains("<child_field_a>Child 2</child_field_a>"));
    assert!(result.contains("<child_field_a>Child 3</child_field_a>"));
    Ok(())
}

#[test]
fn collection_empty() -> Result<(), PError> {
    let xml = r#"
    <XMLObject>
        <field_a>Some Text</field_a>
        <children>
        </children>
    </XMLObject>"#;

    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.field_a, "Some Text");
    assert_eq!(obj.children.len(), 0);
    let test_data = XMLObject {
        field_a: "Empty Collection".to_string(),
        children: vec![],
    };

    let result = from_obj(&test_data);
    assert!(!result.is_empty());
    assert!(result.contains("<field_a>Empty Collection</field_a>"));
    assert!(result.contains("<children></children>"));
    Ok(())
}

#[test]
fn collection_tree() -> Result<(), PError> {
    let xml = r#"
    <XMLObjectTree>
        <field_a>Tree Test</field_a>
        <child>
            <inner>
                <inner>
                    <inner>
                        <child_field_a>Deep Value</child_field_a>
                    </inner>
                </inner>
            </inner>
        </child>
    </XMLObjectTree>"#;
    let obj: XMLObjectTree = from_xml(&xml)?;
    assert_eq!(obj.field_a, "Tree Test");

    let child_c = ChildC {
        attribute: "attr_c".to_string(),
        child_field_a: "Value C".to_string(),
    };
    
    let child_b = ChildB {
        attribute: "attr_b".to_string(),
        child_field_a: "Value B".to_string(),
        inner: child_c,
    };
    
    let child_a = ChildA {
        attribute: "attr_a".to_string(),
        child_field_a: "Value A".to_string(),
        inner: child_b,
    };
    
    let child_root = ChildRoot {
        inner: child_a,
    };
    
    let test_data = XMLObjectTree {
        field_a: "Tree Structure".to_string(),
        child: child_root,
    };
    
    let result = from_obj(&test_data);
    assert!(!result.is_empty());
    assert!(result.contains("<field_a>Tree Structure</field_a>"));
    assert!(result.contains("attr_a"));
    assert!(result.contains("attr_b"));
    assert!(result.contains("attr_c"));
    Ok(())
}

#[test]
fn sibling_collections() -> Result<(), PError> {
    let test_data = XMLObjectSiblings {
        field_a: "Siblings Test".to_string(),
        siblings_a: vec![
            ChildSiblingA { child_field_a: "Sibling A1".to_string() },
            ChildSiblingA { child_field_a: "Sibling A2".to_string() },
        ],
        siblings_b: vec![
            ChildSiblingB { child_field_a: "Sibling B1".to_string() },
        ],
    };
    
    let result = from_obj(&test_data);
    assert!(!result.is_empty());
    assert!(result.contains("<field_a>Siblings Test</field_a>"));
    assert!(result.contains("<siblings_a>"));
    assert!(result.contains("<siblings_b>"));
    assert!(result.contains("Sibling A1"));
    assert!(result.contains("Sibling A2"));
    assert!(result.contains("Sibling B1"));

    Ok(())
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestOptionVecInner {
    pub id: u32,
    pub name: String,
    #[xml(inner="item")]
    pub items: Option<Vec<String>>,
}

#[test]
fn test_option_vec_inner_none() -> Result<(), PError> {
    let test_data = TestOptionVecInner {
        id: 123,
        name: "Test Object".to_string(),
        items: None,
    };
    
    let xml = from_obj(&test_data);
    
    // Verifica se a tag <items> NÃO aparece quando é None
    assert!(!xml.contains("<items>"), "Tag <items> should not appear when Option<Vec> is None");
    assert!(!xml.contains("</items>"), "Closing tag </items> should not appear when Option<Vec> is None");
    
    // Verifica se outras tags aparecem normalmente
    assert!(xml.contains("<id>123</id>"));
    assert!(xml.contains("<name>Test Object</name>"));
    
    // Testa round-trip (serializar → deserializar)
    let parsed: TestOptionVecInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    assert_eq!(parsed.items, None);
    
    Ok(())
}

#[test]
fn test_option_vec_inner_some() -> Result<(), PError> {
    let test_data = TestOptionVecInner {
        id: 456,
        name: "Test Object With Items".to_string(),
        items: Some(vec!["item1".to_string(), "item2".to_string(), "item3".to_string()]),
    };
    
    let xml = from_obj(&test_data);
    
    // Verifica se a tag <items> aparece quando tem valores
    assert!(xml.contains("<items>"), "Tag <items> should appear when Option<Vec> has values");
    assert!(xml.contains("</items>"), "Closing tag </items> should appear when Option<Vec> has values");
    
    // Verifica se os itens individuais aparecem
    assert!(xml.contains("<item>item1</item>"));
    assert!(xml.contains("<item>item2</item>"));
    assert!(xml.contains("<item>item3</item>"));
    
    // Verifica se outras tags aparecem normalmente
    assert!(xml.contains("<id>456</id>"));
    assert!(xml.contains("<name>Test Object With Items</name>"));
    
    // Testa round-trip (serializar → deserializar)
    let parsed: TestOptionVecInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    assert_eq!(parsed.items, Some(vec!["item1".to_string(), "item2".to_string(), "item3".to_string()]));
    
    Ok(())
}

#[test]
fn test_option_vec_inner_empty() -> Result<(), PError> {
    let test_data = TestOptionVecInner {
        id: 789,
        name: "Test Object Empty".to_string(),
        items: Some(vec![]),
    };
    
    let xml = from_obj(&test_data);
    
    // Verifica se a tag <items> aparece mesmo quando o Vec está vazio
    assert!(xml.contains("<items>"), "Tag <items> should appear even when Vec is empty");
    assert!(xml.contains("</items>"), "Closing tag </items> should appear even when Vec is empty");
    
    // Verifica se outras tags aparecem normalmente
    assert!(xml.contains("<id>789</id>"));
    assert!(xml.contains("<name>Test Object Empty</name>"));
    
    // Testa round-trip (serializar → deserializar)
    let parsed: TestOptionVecInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    assert_eq!(parsed.items, Some(vec![]));
    
    Ok(())
} 