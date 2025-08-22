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
fn test_private_fields() -> Result<(), PError> {
    let test_data = TestPrivateFields {
        id: 123,
        name: "Test Name".to_string(),
        internal_data: "Internal data".to_string(),
    };
    
    let xml = from_obj(&test_data);
    
    assert!(!xml.contains("internal_data"));
    assert!(!xml.contains("Internal data"));
    
    let parsed: TestPrivateFields = from_xml(&xml)?;
    
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);

    Ok(())
}

#[test]
fn test_skip_fields() -> Result<(), PError> {
    let test_data = TestSkipFields {
        id: 123,
        name: "Test Name".to_string(),
        skipped_field: "This should be skipped".to_string(),
        visible_field: "This should be visible".to_string(),
    };
    
    let xml = from_obj(&test_data);
    
    assert!(!xml.contains("skipped_field"));
    assert!(!xml.contains("This should be skipped"));
    
    assert!(xml.contains("visible_field"));
    assert!(xml.contains("This should be visible"));
    
    let parsed: TestSkipFields = from_xml(&xml)?;

    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    assert_eq!(test_data.visible_field, parsed.visible_field);
    
    Ok(())
}

#[test]
fn test_custom_naming() -> Result<(), PError> {
    let test_data = TestCustomNaming {
        id: 123,
        name: "Custom Name".to_string(),
        age: 25,
    };
    
    let xml = from_obj(&test_data);

    assert!(xml.contains("<custom_id>123</custom_id>"));
    assert!(xml.contains("<user_name>Custom Name</user_name>"));
    assert!(xml.contains("<user_age>25</user_age>"));

    assert!(!xml.contains("<id>"));
    assert!(!xml.contains("<name>"));
    assert!(!xml.contains("<age>"));
    
    let parsed: TestCustomNaming = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_custom_naming_manual_xml() -> Result<(), PError> {
    let manual_xml = r#"
    <TestCustomNaming>
        <custom_id>456</custom_id>
        <user_name>Manual Custom Name</user_name>
        <user_age>30</user_age>
    </TestCustomNaming>"#;
    
    let parsed: TestCustomNaming = from_xml(manual_xml)?;
    
    assert_eq!(parsed.id, 456);
    assert_eq!(parsed.name, "Manual Custom Name");
    assert_eq!(parsed.age, 30);
    
    Ok(())
}

#[test]
fn test_nested_options() -> Result<(), PError> {
    let test_data = TestNestedOptions {
        id: 123,
        outer: Some("Nested String".to_string()),
        inner: Some(123),
    };
    
    let xml = from_obj(&test_data);
    
    let parsed: TestNestedOptions = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

    let test_data_none = TestNestedOptions {
        id: 456,
        outer: None,
        inner: Some(789),
    };
    
    let xml_none = from_obj(&test_data_none);
    let parsed_none: TestNestedOptions = from_xml(&xml_none)?;
    assert_eq!(test_data_none, parsed_none);
    
    Ok(())
}

#[test]
fn test_mixed_visibility() -> Result<(), PError> {
    let test_data = TestMixedVisibility {
        public_field: "Public".to_string(),
        crate_field: "Crate".to_string(),
        private_field: "Private".to_string(),
    };
    
    let xml = from_obj(&test_data);

    assert!(xml.contains("public_field"));
    assert!(xml.contains("crate_field"));
    assert!(xml.contains("private_field"));
    
    let parsed: TestMixedVisibility = from_xml(&xml)?;
    
    assert_eq!(test_data.public_field, parsed.public_field);
    assert_eq!(test_data.crate_field, parsed.crate_field);
    assert_eq!(test_data.private_field, parsed.private_field);
    
    Ok(())
}

#[test]
fn test_edge_cases() -> Result<(), PError> {

    let test_empty = TestOptionalFields {
        id: 123,
        name: Some("".to_string()),
        description: Some("".to_string()),
        tags: Some(vec![]),
        metadata: None,
    };
    
    let xml_empty = from_obj(&test_empty);
    let parsed_empty: TestOptionalFields = from_xml(&xml_empty)?;
    assert_eq!(test_empty, parsed_empty);

    let test_extreme = TestCustomNaming {
        id: u32::MAX,
        name: "Extreme".to_string(),
        age: u32::MAX,
    };
    
    let xml_extreme = from_obj(&test_extreme);
    let parsed_extreme: TestCustomNaming = from_xml(&xml_extreme)?;
    assert_eq!(test_extreme, parsed_extreme);
    
    Ok(())
} 