use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestAttributes {
    pub id: u32,
    #[xml(attribute, name="name")]
    pub name: String,
    #[xml(attribute, name="value")]
    pub value: i32,
    #[xml(attribute, name="flag")]
    pub flag: bool,
    pub content: String,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestOptionalAttributes {
    pub id: u32,
    #[xml(attribute, name="name")]
    pub name: Option<String>,
    #[xml(attribute, name="value")]
    pub value: Option<i32>,
    pub content: String,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestMixedAttributes {
    #[xml(attribute, name="id")]
    pub id: u32,
    pub name: String,
    #[xml(attribute, name="enabled")]
    pub enabled: bool,
    pub data: Vec<String>,
}

#[test]
fn test_basic_attributes() -> Result<(), PError> {
    let test_data = TestAttributes {
        id: 123,
        name: "Test Name".to_string(),
        value: 456,
        flag: true,
        content: "Content here".to_string(),
    };
    
    let xml = from_obj(&test_data);
    
    assert!(xml.contains("name=\"Test Name\""));
    assert!(xml.contains("value=\"456\""));
    assert!(xml.contains("flag=\"true\""));
    
    let parsed: TestAttributes = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_optional_attributes() -> Result<(), PError> {
    // Teste com todos os atributos
    let test_data = TestOptionalAttributes {
        id: 123,
        name: Some("Test Name".to_string()),
        value: Some(456),
        content: "Content".to_string(),
    };
    
    let xml = from_obj(&test_data);
    let parsed: TestOptionalAttributes = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    // Teste sem atributos opcionais
    let test_data_none = TestOptionalAttributes {
        id: 789,
        name: None,
        value: None,
        content: "Content Only".to_string(),
    };
    
    let xml_none = from_obj(&test_data_none);
    let parsed_none: TestOptionalAttributes = from_xml(&xml_none)?;
    assert_eq!(test_data_none, parsed_none);
    
    Ok(())
}

#[test]
fn test_mixed_attributes_and_content() -> Result<(), PError> {
    let test_data = TestMixedAttributes {
        id: 123,
        name: "Mixed Test".to_string(),
        enabled: true,
        data: vec!["item1".to_string(), "item2".to_string()],
    };
    
    let xml = from_obj(&test_data);
    
    assert!(xml.contains("id=\"123\""));
    assert!(xml.contains("enabled=\"true\""));
    
    let parsed: TestMixedAttributes = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_attributes_with_special_characters() -> Result<(), PError> {
    let test_data = TestAttributes {
        id: 123,
        name: "Test & Name <with> \"quotes\"".to_string(),
        value: -123,
        flag: false,
        content: "Content with & < > \" characters".to_string(),
    };
    
    let xml = from_obj(&test_data);
    
    let parsed: TestAttributes = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_attributes_validation() {
    // XML with missing required attribute
    let invalid_xml = r#"
    <TestAttributes>
        <id>123</id>
        <content>Content</content>
    </TestAttributes>"#;
    
    let result = from_xml::<TestAttributes>(invalid_xml);
    // Should fail because name, value and flag are required
    assert!(result.is_err());
}

#[test]
fn test_attributes_manual_xml() -> Result<(), PError> {
    let manual_xml = r#"
    <TestAttributes name="Manual Test" value="999" flag="false">
        <id>123</id>
        <content>Manual content</content>
    </TestAttributes>"#;
    
    let parsed: TestAttributes = from_xml(manual_xml)?;
    
    assert_eq!(parsed.id, 123);
    assert_eq!(parsed.name, "Manual Test");
    assert_eq!(parsed.value, 999);
    assert_eq!(parsed.flag, false);
    assert_eq!(parsed.content, "Manual content");
    
    Ok(())
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestNestedAttributes {
    #[xml(attr="id")]
    pub id: u32,
    pub inner: TestInnerWithAttributes,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestInnerWithAttributes {
    #[xml(attr="name")]
    pub name: String,
    pub value: String,
}

#[test]
fn test_nested_attributes() -> Result<(), PError> {
    let test_data = TestNestedAttributes {
        id: 123,
        inner: TestInnerWithAttributes {
            name: "Inner Name".to_string(),
            value: "Inner Value".to_string(),
        },
    };
    
    let xml = from_obj(&test_data);
    
    let parsed: TestNestedAttributes = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

