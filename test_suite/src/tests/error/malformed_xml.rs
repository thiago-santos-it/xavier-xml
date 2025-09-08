use xavier::{from_xml, PError, XmlDeserializable};

#[test]
fn test_error_handling_malformed_missing_closing_tag() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingMalformed {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let malformed_xml = r#"
    <TestErrorHandlingMalformed>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <!-- Missing closing tag -->
    "#;
    
    let result = from_xml::<TestErrorHandlingMalformed>(malformed_xml);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_error_handling_malformed_unclosed_tag() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingMalformed {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let malformed_xml = r#"
    <TestErrorHandlingMalformed>
        <id>1</id>
        <name>John Doe
        <email>john@example.com</email>
    </TestErrorHandlingMalformed>"#;
    
    let result = from_xml::<TestErrorHandlingMalformed>(malformed_xml);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_error_handling_malformed_invalid_nesting() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingMalformed {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let malformed_xml = r#"
    <TestErrorHandlingMalformed>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tag>tag1</tag>
            <tag>tag2
        </tags>
        <active>true</active>
    </TestErrorHandlingMalformed>"#;
    
    let result = from_xml::<TestErrorHandlingMalformed>(malformed_xml);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_error_handling_malformed_empty_xml() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingMalformed {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let empty_xml = "";
    let result = from_xml::<TestErrorHandlingMalformed>(empty_xml);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_error_handling_malformed_invalid_xml_syntax() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingMalformed {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let invalid_xml = r#"
    <TestErrorHandlingMalformed>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tag>tag1</tag>
            <tag>tag2</tag>
        </tags>
        <active>true</active>
        <score>95.5</score>
    </TestErrorHandlingMalformed
    "#;
    
    let result = from_xml::<TestErrorHandlingMalformed>(invalid_xml);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_error_handling_malformed_mismatched_tags() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingMalformed {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let malformed_xml = r#"
    <TestErrorHandlingMalformed>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tag>tag1</tag>
            <tag>tag2</tag>
        </tags>
        <active>true</active>
        <score>95.5</score>
    </WrongTagName>
    "#;
    
    let result = from_xml::<TestErrorHandlingMalformed>(malformed_xml);
    assert!(result.is_err());
    
    Ok(())
} 