use xavier::{from_xml, PError, XmlDeserializable};

#[test]
fn test_error_handling_panic_missing_required_field() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingPanic {
        pub id: u32,
        pub name: String,
        pub description: String,
    }

    let xml = r#"
    <TestErrorHandlingPanic>
        <id>1</id>
        <!-- Missing name field -->
        <description>Test Description</description>
    </TestErrorHandlingPanic>"#;
    
    let result: Result<TestErrorHandlingPanic, PError> = from_xml(&xml);
    if let Err(_error) = result {
        assert!(true)
    } else {
        assert!(false)
    }
    
    Ok(())
}

#[test]
fn test_error_handling_panic_infinite_recursion() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingPanic {
        pub id: u32,
        pub name: String,
        pub description: String,
    }

    let xml = r#"
    <TestErrorHandlingPanic>
        <id>1</id>
        <name>Test</name>
        <description>Test Description</description>
        <name>Test2</name>
    </TestErrorHandlingPanic>"#;
    
    let result: Result<TestErrorHandlingPanic, PError> = from_xml(&xml);
    assert!(result.is_err() || result.is_ok());
    
    Ok(())
}

#[test]
fn test_error_handling_panic_malformed_nested_structure() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingPanic {
        pub id: u32,
        pub name: String,
        pub description: String,
    }

    let xml = r#"
    <TestErrorHandlingPanic>
        <id>1</id>
        <name>Test</name>
        <description>Test Description</description>
        <nested>
            <id>2</id>
            <name>Nested</name>
            <description>Nested Description</description>
        </nested>
    </TestErrorHandlingPanic>"#;
    
    let result: Result<TestErrorHandlingPanic, PError> = from_xml(&xml);
    assert!(result.is_err() || result.is_ok());
    
    Ok(())
}

#[test]
fn test_error_handling_panic_very_deep_nesting() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingPanic {
        pub id: u32,
        pub name: String,
        pub description: String,
    }

    let mut xml = String::new();
    xml.push_str("<TestErrorHandlingPanic>");
    
    for i in 0..1000 {
        xml.push_str(&format!("<level{}>", i));
    }
    
    xml.push_str("<id>1</id>");
    xml.push_str("<name>Test</name>");
    xml.push_str("<description>Test Description</description>");
    
    for i in (0..1000).rev() {
        xml.push_str(&format!("</level{}>", i));
    }
    
    xml.push_str("</TestErrorHandlingPanic>");
    
    let result: Result<TestErrorHandlingPanic, PError> = from_xml(&xml);
    assert!(result.is_err() || result.is_ok());
    
    Ok(())
}

#[test]
fn test_error_handling_panic_large_data() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingPanic {
        pub id: u32,
        pub name: String,
        pub data: Vec<String>,
    }

    let mut xml = String::new();
    xml.push_str("<TestErrorHandlingPanic>");
    xml.push_str("<id>1</id>");
    xml.push_str("<name>Large Data Test</name>");
    xml.push_str("<data>");
    
    for i in 0..10000 {
        xml.push_str(&format!("<item>Data item {}</item>", i));
    }
    
    xml.push_str("</data>");
    xml.push_str("</TestErrorHandlingPanic>");
    
    let result: Result<TestErrorHandlingPanic, PError> = from_xml(&xml);
    assert!(result.is_err() || result.is_ok());
    
    Ok(())
}

#[test]
fn test_error_handling_panic_invalid_utf8() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingPanic {
        pub id: u32,
        pub name: String,
        pub description: String,
    }

    let invalid_utf8 = vec![0xFF, 0xFE, 0x00]; 
    let xml = String::from_utf8_lossy(&invalid_utf8);
    
    let result: Result<TestErrorHandlingPanic, PError> = from_xml(&xml);
    assert!(result.is_err() || result.is_ok());
    
    Ok(())
} 