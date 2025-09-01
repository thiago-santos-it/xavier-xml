use xavier::{from_xml, PError, XmlDeserializable};

#[test]
fn test_error_handling_type_mismatch_string_to_number() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingTypeMismatch {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let xml_with_wrong_type = r#"
    <TestErrorHandlingTypeMismatch>
        <id>not_a_number</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
            <tags>tag2</tags>
        </tags>
        <active>true</active>
        <score>95.5</score>
    </TestErrorHandlingTypeMismatch>
    "#;
    
    let result = from_xml::<TestErrorHandlingTypeMismatch>(xml_with_wrong_type);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_error_handling_type_mismatch_string_to_bool() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingTypeMismatch {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let xml_with_wrong_type = r#"
    <TestErrorHandlingTypeMismatch>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
        <active>not_a_boolean</active>
        <score>95.5</score>
    </TestErrorHandlingTypeMismatch>
    "#;
    
    let result = from_xml::<TestErrorHandlingTypeMismatch>(xml_with_wrong_type);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_error_handling_type_mismatch_string_to_float() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingTypeMismatch {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let xml_with_wrong_type = r#"
    <TestErrorHandlingTypeMismatch>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
        <active>true</active>
        <score>not_a_float</score>
    </TestErrorHandlingTypeMismatch>
    "#;
    
    let result = from_xml::<TestErrorHandlingTypeMismatch>(xml_with_wrong_type);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_error_handling_type_mismatch_invalid_u64() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingTypeMismatch {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let xml_with_wrong_type = r#"
    <TestErrorHandlingTypeMismatch>
        <id>-1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
        <active>true</active>
        <score>95.5</score>
    </TestErrorHandlingTypeMismatch>
    "#;
    
    let result = from_xml::<TestErrorHandlingTypeMismatch>(xml_with_wrong_type);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_error_handling_type_mismatch_invalid_collection() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingTypeMismatch {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let xml_with_wrong_type = r#"
    <TestErrorHandlingTypeMismatch>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>not_a_collection</tags>
        <active>true</active>
        <score>95.5</score>
    </TestErrorHandlingTypeMismatch>
    "#;
    
    let result = from_xml::<TestErrorHandlingTypeMismatch>(xml_with_wrong_type);
    assert!(result.is_err());
    
    Ok(())
}

