use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestOptionalFields {
    pub id: u32,
    pub name: Option<String>,
    pub description: Option<String>,
    #[xml(inner="tag")]
    pub tags: Option<Vec<String>>,
    pub metadata: Option<KeyValue>,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct KeyValue {
    pub key: String,
    pub value: String,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestPrivateFields {
    pub id: u32,
    pub name: String,
    #[xml(skip)]
    pub internal_data: String,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestSkipFields {
    pub id: u32,
    pub name: String,
    #[xml(skip)]
    pub skipped_field: String,
    pub visible_field: String,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestCustomNaming {
    #[xml(name="custom_id")]
    pub id: u32,
    #[xml(name="user_name")]
    pub name: String,
    #[xml(name="user_age")]
    pub age: u32,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestMixedVisibility {
    pub public_field: String,
    pub(crate) crate_field: String,
    pub private_field: String,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestNestedOptions {
    pub id: u32,
    pub outer: Option<String>,
    pub inner: Option<i32>,
}

#[test]
fn test_optional_fields_serialization() -> Result<(), PError> {
    let test_data = TestOptionalFields {
        id: 123,
        name: Some("Test Name".to_string()),
        description: Some("Test Description".to_string()),
        tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
        metadata: Some(KeyValue {
            key: "type".to_string(),
            value: "test".to_string(),
        }),
    };
    
    let xml = from_obj(&test_data);
    
    let parsed: TestOptionalFields = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_optional_fields_deserialization() -> Result<(), PError> {
    // Teste com campos opcionais ausentes
    let xml_without_optionals = r#"
    <TestOptionalFields>
        <id>456</id>
    </TestOptionalFields>"#;
    
    let parsed: TestOptionalFields = from_xml(xml_without_optionals)?;
    
    assert_eq!(parsed.id, 456);
    assert_eq!(parsed.name, None);
    assert_eq!(parsed.description, None);
    assert_eq!(parsed.tags, None);
    assert_eq!(parsed.metadata, None);
    
    Ok(())
}

#[test]
fn test_private_fields_handling() -> Result<(), PError> {
    let test_data = TestPrivateFields {
        id: 789,
        name: "Private Test".to_string(),
        internal_data: "Internal Value".to_string(),
    };
    
    let xml = from_obj(&test_data);
    
    // Internal data should be skipped during serialization
    assert!(!xml.contains("internal_data"));
    assert!(!xml.contains("Internal Value"));
    
    let parsed: TestPrivateFields = from_xml(&xml)?;
    
    assert_eq!(parsed.id, 789);
    assert_eq!(parsed.name, "Private Test");
    // internal_data should have default value or be handled appropriately
    
    Ok(())
}

#[test]
fn test_skip_fields_handling() -> Result<(), PError> {
    let test_data = TestSkipFields {
        id: 101,
        name: "Skip Test".to_string(),
        skipped_field: "Should be skipped".to_string(),
        visible_field: "Should be visible".to_string(),
    };
    
    let xml = from_obj(&test_data);
    
    // Skipped field should not appear in XML
    assert!(!xml.contains("skipped_field"));
    assert!(!xml.contains("Should be skipped"));
    
    // Visible field should appear
    assert!(xml.contains("visible_field"));
    assert!(xml.contains("Should be visible"));
    
    let parsed: TestSkipFields = from_xml(&xml)?;
    
    assert_eq!(parsed.id, 101);
    assert_eq!(parsed.name, "Skip Test");
    assert_eq!(parsed.visible_field, "Should be visible");
    
    Ok(())
}

#[test]
fn test_custom_naming_handling() -> Result<(), PError> {
    let test_data = TestCustomNaming {
        id: 202,
        name: "Custom Name".to_string(),
        age: 25,
    };
    
    let xml = from_obj(&test_data);
    
    // Should use custom names in XML
    assert!(xml.contains("custom_id"));
    assert!(xml.contains("user_name"));
    assert!(xml.contains("user_age"));
    
    // Should not contain original field names
    assert!(!xml.contains("id"));
    assert!(!xml.contains("name"));
    assert!(!xml.contains("age"));
    
    let parsed: TestCustomNaming = from_xml(&xml)?;
    
    assert_eq!(parsed.id, 202);
    assert_eq!(parsed.name, "Custom Name");
    assert_eq!(parsed.age, 25);
    
    Ok(())
} 