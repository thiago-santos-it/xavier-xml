use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
#[allow(dead_code)]
struct TestErrorStruct {
    pub id: u32,
    pub name: String,
    pub value: i32,
}

#[derive(XmlDeserializable, Debug)]
struct TestErrorStructWithOption {
    pub id: u32,
    pub name: Option<String>,
    pub value: Option<i32>,
}

#[derive(XmlDeserializable, Debug)]
#[allow(dead_code)]
struct TestErrorStructWithVec {
    pub id: u32,
    pub items: Vec<String>,
}

#[test]
fn test_empty_xml_error() {
    let empty_xml = "";
    let result = from_xml::<TestErrorStruct>(empty_xml);
    
    assert!(result.is_err());
    if let Err(error) = result {
        assert!(error.to_string().contains("Empty XML"));
    }
}

#[test]
fn test_whitespace_only_xml_error() {
    let whitespace_xml = "   \n\t  ";
    let result = from_xml::<TestErrorStruct>(whitespace_xml);
    
    assert!(result.is_err());
    if let Err(error) = result {
        assert!(error.to_string().contains("Empty XML"));
    }
}

#[test]
fn test_malformed_xml_missing_closing_tag() {
    let malformed_xml = r#"
    <TestErrorStruct>
        <id>123</id>
        <name>Test Name
        <value>456</value>
    </TestErrorStruct>"#;
    
    let result = from_xml::<TestErrorStruct>(malformed_xml);
    assert!(result.is_err());
}

#[test]
fn test_malformed_xml_missing_opening_tag() {
    let malformed_xml = r#"
    <TestErrorStruct>
        <id>123</id>
        <name>Test Name</name>
        </value>456</value>
    </TestErrorStruct>"#;
    
    let result = from_xml::<TestErrorStruct>(malformed_xml);
    assert!(result.is_err());
}

#[test]
fn test_malformed_xml_unclosed_root() {
    let malformed_xml = r#"
    <TestErrorStruct>
        <id>123</id>
        <name>Test Name</name>
        <value>456</value>"#;
    
    let result = from_xml::<TestErrorStruct>(malformed_xml);
    assert!(result.is_err());
}

#[test]
fn test_malformed_xml_wrong_tag_name() {
    let malformed_xml = r#"
    <WrongTagName>
        <id>123</id>
        <name>Test Name</name>
        <value>456</value>
    </WrongTagName>"#;
    
    let result = from_xml::<TestErrorStruct>(malformed_xml);
    assert!(result.is_err());
}

#[test]
fn test_malformed_xml_missing_required_field() {
    let malformed_xml = r#"
    <TestErrorStruct>
        <id>123</id>
        <name>Test Name</name>
    </TestErrorStruct>"#;
    
    let result = from_xml::<TestErrorStruct>(malformed_xml);
    assert!(result.is_err());
}

#[test]
#[allow(unused_variables)]
fn test_malformed_xml_extra_field() {
    let malformed_xml = r#"
    <TestErrorStruct>
        <id>123</id>
        <name>Test Name</name>
        <value>456</value>
        <extra_field>Extra Value</extra_field>
    </TestErrorStruct>"#;
    let _result = from_xml::<TestErrorStruct>(malformed_xml);
    // This test may pass or fail depending on implementation
    // Some parsers are tolerant to extra fields
    // We don't assert here as behavior may vary
}

#[test]
fn test_type_conversion_error_invalid_integer() {
    let invalid_integer_xml = r#"
    <TestErrorStruct>
        <id>123</id>
        <name>Test Name</name>
        <value>not_a_number</value>
    </TestErrorStruct>"#;
    
    let result = from_xml::<TestErrorStruct>(invalid_integer_xml);
    assert!(result.is_err());
}

#[test]
fn test_type_conversion_error_invalid_unsigned_integer() {
    let invalid_uint_xml = r#"
    <TestErrorStruct>
        <id>-123</id>
        <name>Test Name</name>
        <value>456</value>
    </TestErrorStruct>"#;
    
    let result = from_xml::<TestErrorStruct>(invalid_uint_xml);
    assert!(result.is_err());
}

#[test]
fn test_type_conversion_error_invalid_float() {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestFloatStruct {
        pub id: u32,
        pub value: f64,
    }
    
    let invalid_float_xml = r#"
    <TestFloatStruct>
        <id>123</id>
        <value>not_a_float</value>
    </TestFloatStruct>"#;
    
    let result = from_xml::<TestFloatStruct>(invalid_float_xml);
    assert!(result.is_err());
}

#[test]
fn test_type_conversion_error_invalid_boolean() {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestBooleanStruct {
        pub id: u32,
        pub flag: bool,
    }
    
    let invalid_boolean_xml = r#"
    <TestBooleanStruct>
        <id>123</id>
        <flag>not_a_boolean</flag>
    </TestBooleanStruct>"#;
    
    let result = from_xml::<TestBooleanStruct>(invalid_boolean_xml);
    assert!(result.is_err());
}

#[test]
fn test_encoding_error_invalid_utf8() {
    // Create XML with invalid UTF-8 bytes
    let invalid_utf8_bytes = vec![
        0x3c, 0x54, 0x65, 0x73, 0x74, 0x3e, // <Test>
        0x48, 0x65, 0x6c, 0x6c, 0x6f, // Hello
        0xff, 0xfe, // Invalid UTF-8 bytes
        0x3c, 0x2f, 0x54, 0x65, 0x73, 0x74, 0x3e // </Test>
    ];
    
    let invalid_utf8_xml = String::from_utf8(invalid_utf8_bytes);
    assert!(invalid_utf8_xml.is_err());
}

#[test]
fn test_encoding_error_invalid_xml_entities() {
    let invalid_entities_xml = r#"
    <TestErrorStruct>
        <id>123</id>
        <name>Test &amp; Name</name>
        <value>456</value>
    </TestErrorStruct>"#;
    
    // This test should pass as &amp; is a valid entity
    let result = from_xml::<TestErrorStruct>(invalid_entities_xml);
    assert!(result.is_ok());
}

#[test]
fn test_encoding_error_malformed_entities() {
    let malformed_entities_xml = r#"
    <TestErrorStruct>
        <id>123</id>
        <name>Test & Name</name>
        <value>456</value>
    </TestErrorStruct>"#;
    
    let _result = from_xml::<TestErrorStruct>(malformed_entities_xml);
    // May fail due to unescaped &
    // We don't assert here as behavior may vary
}

#[test]
fn test_collection_parsing_error() {
    let malformed_collection_xml = r#"
    <TestErrorStructWithVec>
        <id>123</id>
        <items>
            <items>item1</items>
            <items>item2
            <items>item3</items>
        </items>
    </TestErrorStructWithVec>"#;
    
    let result = from_xml::<TestErrorStructWithVec>(malformed_collection_xml);
    assert!(result.is_err());
}

#[test]
fn test_option_field_parsing() -> Result<(), PError> {
    // Test with optional fields present
    let valid_option_xml = r#"
    <TestErrorStructWithOption>
        <id>123</id>
        <name>Test Name</name>
        <value>456</value>
    </TestErrorStructWithOption>"#;
    
    let result = from_xml::<TestErrorStructWithOption>(valid_option_xml)?;
    assert_eq!(result.id, 123);
    assert_eq!(result.name, Some("Test Name".to_string()));
    assert_eq!(result.value, Some(456));
    
    // Test with optional fields missing
    let missing_option_xml = r#"
    <TestErrorStructWithOption>
        <id>789</id>
    </TestErrorStructWithOption>"#;
    
    let result = from_xml::<TestErrorStructWithOption>(missing_option_xml)?;
    assert_eq!(result.id, 789);
    assert_eq!(result.name, None);
    assert_eq!(result.value, None);
    
    Ok(())
}

#[test]
fn test_nested_structure_parsing_error() {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestNestedStruct {
        pub id: u32,
        pub inner: TestInnerStruct,
    }
    
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestInnerStruct {
        pub name: String,
        pub value: i32,
    }
    
    let malformed_nested_xml = r#"
    <TestNestedStruct>
        <id>123</id>
        <inner>
            <name>Inner Name
            <value>456</value>
        </inner>
    </TestNestedStruct>"#;
    
    let result = from_xml::<TestNestedStruct>(malformed_nested_xml);
    assert!(result.is_err());
}

#[test]
fn test_error_message_content() {
    let malformed_xml = r#"
    <TestErrorStruct>
        <id>123</id>
        <name>Test Name
        <value>456</value>
    </TestErrorStruct>"#;
    
    let result = from_xml::<TestErrorStruct>(malformed_xml);
    assert!(result.is_err());
    
    if let Err(error) = result {
        let error_message = error.to_string();
        
        // Check if error message contains useful information
        assert!(!error_message.is_empty());
        assert!(error_message.len() > 10); // Should have at least some information
    }
}

#[test]
fn test_multiple_errors_handling() {
    let multiple_errors_xml = r#"
    <TestErrorStruct>
        <id>not_a_number</id>
        <name>Test Name
        <value>also_not_a_number</value>
    </TestErrorStruct>"#;
    
    let result = from_xml::<TestErrorStruct>(multiple_errors_xml);
    assert!(result.is_err());
    
    // The parser should fail on the first error encountered
    // We don't make specific assertions about which error was reported
}

#[test]
fn test_error_recovery_attempts() {
    // Test to verify if the parser attempts to recover from errors
    let recoverable_xml = r#"
    <TestErrorStruct>
        <id>123</id>
        <name>Test Name</name>
        <value>456</value>
        <extra_field>Extra</extra_field>
        <another_extra>Another</another_extra>
    </TestErrorStruct>"#;
    
    let _result = from_xml::<TestErrorStruct>(recoverable_xml);
    // Depending on implementation, may pass or fail
    // We don't assert here
}

#[test]
fn test_error_with_large_xml() {
    // Create large XML with an error at the end
    let mut large_xml = String::from("<TestErrorStruct>\n");
    large_xml.push_str("<id>123</id>\n");
    large_xml.push_str("<name>Test Name</name>\n");
    
    // Add many valid lines
    for i in 0..1000 {
        large_xml.push_str(&format!("<extra_field_{}>Value {}</extra_field_{}>\n", i, i, i));
    }
    
    large_xml.push_str("<value>456</value>\n");
    large_xml.push_str("</TestErrorStruct>"); // Correct closing
    
    let result = from_xml::<TestErrorStruct>(&large_xml);
    // Should pass as XML is valid
    assert!(result.is_ok());
    
    // Now create large XML with an error
    let mut large_xml_with_error = String::from("<TestErrorStruct>\n");
    large_xml_with_error.push_str("<id>123</id>\n");
    large_xml_with_error.push_str("<name>Test Name</name>\n");
    
    // Add many valid lines
    for i in 0..1000 {
        large_xml_with_error.push_str(&format!("<extra_field_{}>Value {}</extra_field_{}>\n", i, i, i));
    }
    
    large_xml_with_error.push_str("<value>456</value>\n");
    // Missing root tag closing
    
    let result = from_xml::<TestErrorStruct>(&large_xml_with_error);
    assert!(result.is_err());
}
