use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};

#[test]
fn test_error_handling_specific_optional_fields_serialization() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestErrorHandlingSpecificOptional {
        pub id: u32,
        pub name: Option<String>,
        pub description: Option<String>,
        #[xml(inner="tagz")]
        pub tags: Option<Vec<String>>,
        pub metadata: Option<TestErrorHandlingSpecificKeyValue>,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestErrorHandlingSpecificKeyValue {
        pub key: String,
        pub value: String,
    }

    let test_data = TestErrorHandlingSpecificOptional {
        id: 123,
        name: Some("Test Name".to_string()),
        description: Some("Test Description".to_string()),
        tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
        metadata: Some(TestErrorHandlingSpecificKeyValue {
            key: "type".to_string(),
            value: "test".to_string(),
        }),
    };
    
    let xml = from_obj(&test_data);
    
    let parsed: TestErrorHandlingSpecificOptional = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_error_handling_specific_optional_fields_deserialization() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestErrorHandlingSpecificOptional {
        pub id: u32,
        pub name: Option<String>,
        pub description: Option<String>,
        #[xml(inner="tagz")]
        pub tags: Option<Vec<String>>,
        pub metadata: Option<TestErrorHandlingSpecificKeyValue>,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestErrorHandlingSpecificKeyValue {
        pub key: String,
        pub value: String,
    }

    let xml_without_optionals = r#"
    <TestErrorHandlingSpecificOptional>
        <id>456</id>
    </TestErrorHandlingSpecificOptional>"#;
    
    let parsed: TestErrorHandlingSpecificOptional = from_xml(xml_without_optionals)?;
    
    assert_eq!(parsed.id, 456);
    assert_eq!(parsed.name, None);
    assert_eq!(parsed.description, None);
    assert_eq!(parsed.tags, None);
    assert_eq!(parsed.metadata, None);
    
    Ok(())
}

#[test]
fn test_error_handling_specific_skip_fields() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestErrorHandlingSpecificSkip {
        pub id: u32,
        pub name: String,
        #[xml(skip)]
        pub skipped_field: Option<String>,
        pub visible_field: String,
    }

    let test_data = TestErrorHandlingSpecificSkip {
        id: 123,
        name: "Test Name".to_string(),
        skipped_field: Some("Internal Data".to_string()),
        visible_field: "Visible Data".to_string(),
    };
    
    let xml = from_obj(&test_data);
    
    assert!(!xml.contains("skipped_field"));
    assert!(xml.contains("visible_field"));
    
    let parsed: TestErrorHandlingSpecificSkip = from_xml(&xml)?;
    assert_eq!(parsed.id, test_data.id);
    assert_eq!(parsed.name, test_data.name);
    assert_eq!(parsed.visible_field, test_data.visible_field);
    
    Ok(())
}

#[test]
fn test_error_handling_specific_custom_naming() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestErrorHandlingSpecificCustom {
        #[xml(name="custom_id")]
        pub id: u32,
        #[xml(name="user_name")]
        pub name: String,
        #[xml(name="user_age")]
        pub age: u32,
    }

    let test_data = TestErrorHandlingSpecificCustom {
        id: 123,
        name: "John Doe".to_string(),
        age: 30,
    };
    
    let xml = from_obj(&test_data);
    
    assert!(xml.contains("custom_id"));
    assert!(xml.contains("user_name"));
    assert!(xml.contains("user_age"));
    
    let parsed: TestErrorHandlingSpecificCustom = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_error_handling_specific_nested_options() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestErrorHandlingSpecificNested {
        pub id: u32,
        pub outer: Option<String>,
        pub inner: Option<i32>,
    }

    let test_data = TestErrorHandlingSpecificNested {
        id: 123,
        outer: Some("Outer Value".to_string()),
        inner: Some(42),
    };
    
    let xml = from_obj(&test_data);
    
    let parsed: TestErrorHandlingSpecificNested = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_error_handling_specific_malformed_inner_tags() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestErrorHandlingSpecificInner {
        pub id: u32,
        pub name: String,
        #[xml(inner="tagz")]
        pub tags: Option<Vec<String>>,
    }

    let xml_malformed_inner = r#"
    <TestErrorHandlingSpecificInner>
        <id>123</id>
        <name>Test</name>
        <tags>
            <wrong_tag>tag1</wrong_tag>
            <tagz>tag2</tagz>
        </tags>
    </TestErrorHandlingSpecificInner>"#;
    
    let result = from_xml::<TestErrorHandlingSpecificInner>(xml_malformed_inner);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_error_handling_specific_invalid_attribute() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestErrorHandlingSpecificAttribute {
        #[xml(attribute)]
        pub id: u32,
        pub name: String,
        pub description: String,
    }

    let xml_invalid_attribute = r#"
    <TestErrorHandlingSpecificAttribute id="not_a_number">
        <name>Test</name>
        <description>Test Description</description>
    </TestErrorHandlingSpecificAttribute>"#;
    
    let result = from_xml::<TestErrorHandlingSpecificAttribute>(xml_invalid_attribute);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_error_handling_specific_missing_root_tag() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestErrorHandlingSpecificRoot {
        pub id: u32,
        pub name: String,
        pub description: String,
    }

    let xml_missing_root = r#"
    <WrongRootTag>
        <id>123</id>
        <name>Test</name>
        <description>Test Description</description>
    </WrongRootTag>"#;
    
    let result = from_xml::<TestErrorHandlingSpecificRoot>(xml_missing_root);
    assert!(result.is_err());
    
    Ok(())
} 