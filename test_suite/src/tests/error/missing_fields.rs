use xavier::{from_xml, PError, XmlDeserializable};

#[test]
fn test_error_handling_missing_required_field_name() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingMissing {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let xml_missing_field = r#"
    <TestErrorHandlingMissing>
        <id>1</id>
        <!-- Missing name field -->
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
        <active>true</active>
        <score>95.5</score>
    </TestErrorHandlingMissing>
    "#;
    
    let result = from_xml::<TestErrorHandlingMissing>(xml_missing_field);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_error_handling_missing_required_field_id() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingMissing {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let xml_missing_field = r#"
    <TestErrorHandlingMissing>
        <!-- Missing id field -->
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
        <active>true</active>
        <score>95.5</score>
    </TestErrorHandlingMissing>
    "#;
    
    let result = from_xml::<TestErrorHandlingMissing>(xml_missing_field);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_error_handling_missing_required_field_active() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingMissing {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let xml_missing_field = r#"
    <TestErrorHandlingMissing>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
        <!-- Missing active field -->
        <score>95.5</score>
    </TestErrorHandlingMissing>
    "#;
    
    let result = from_xml::<TestErrorHandlingMissing>(xml_missing_field);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_error_handling_missing_required_field_score() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingMissing {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let xml_missing_field = r#"
    <TestErrorHandlingMissing>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
        <active>true</active>
        <!-- Missing score field -->
    </TestErrorHandlingMissing>
    "#;
    
    let result = from_xml::<TestErrorHandlingMissing>(xml_missing_field);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_error_handling_missing_multiple_required_fields() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingMissing {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let xml_missing_fields = r#"
    <TestErrorHandlingMissing>
        <id>1</id>
        <!-- Missing name, active, and score fields -->
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
    </TestErrorHandlingMissing>
    "#;
    
    let result = from_xml::<TestErrorHandlingMissing>(xml_missing_fields);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_error_handling_missing_optional_fields_should_succeed() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingMissing {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let xml_missing_optional = r#"
    <TestErrorHandlingMissing>
        <id>1</id>
        <name>John Doe</name>
        <!-- Missing optional email field -->
        <tags>
            <tags>tag1</tags>
        </tags>
        <active>true</active>
        <score>95.5</score>
    </TestErrorHandlingMissing>
    "#;
    
    let result = from_xml::<TestErrorHandlingMissing>(xml_missing_optional);
    assert!(result.is_ok());
    
    let parsed = result.unwrap();
    assert_eq!(parsed.id, 1);
    assert_eq!(parsed.name, "John Doe");
    assert_eq!(parsed.email, None);
    assert_eq!(parsed.active, true);
    assert_eq!(parsed.score, 95.5);
    
    Ok(())
} 