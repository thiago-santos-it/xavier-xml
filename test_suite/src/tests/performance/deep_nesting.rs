use std::panic;
use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};

#[test]
fn test_performance_deep_nesting_50_levels() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceDeepNested {
        pub level: u32,
        pub content: String,
        #[xml(tree)]
        pub child_deep: Option<Box<TestPerformanceDeepNested>>,
    }

    fn generate_deep_nested_xml(depth: u32) -> String {
        let mut xml = String::new();
        xml.push_str("<TestPerformanceDeepNested>");
        xml.push_str(&format!("<level>{}</level>", depth));
        xml.push_str(&format!("<content>Level {}</content>", depth));
        
        if depth > 1 {
            xml.push_str("<child_deep>");
            xml.push_str(&generate_deep_nested_xml(depth - 1));
            xml.push_str("</child_deep>");
        }
        
        xml.push_str("</TestPerformanceDeepNested>");
        xml
    }

    let xml = generate_deep_nested_xml(50);
    let result = from_xml::<TestPerformanceDeepNested>(&xml);
    assert!(result.is_ok());
    
    let parsed: TestPerformanceDeepNested = result.unwrap();
    assert_eq!(parsed.level, 50);
    
    Ok(())
}

#[test]
fn test_performance_deep_nesting_100_levels() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceDeepNested {
        pub level: u32,
        pub content: String,
        #[xml(tree)]
        pub child_deep: Option<Box<TestPerformanceDeepNested>>,
    }

    fn generate_deep_nested_xml(depth: u32) -> String {
        let mut xml = String::new();
        xml.push_str("<TestPerformanceDeepNested>");
        xml.push_str(&format!("<level>{}</level>", depth));
        xml.push_str(&format!("<content>Level {}</content>", depth));
        
        if depth > 1 {
            xml.push_str("<child_deep>");
            xml.push_str(&generate_deep_nested_xml(depth - 1));
            xml.push_str("</child_deep>");
        }
        
        xml.push_str("</TestPerformanceDeepNested>");
        xml
    }

    let xml = generate_deep_nested_xml(100);
    let result = from_xml::<TestPerformanceDeepNested>(&xml);
    assert!(result.is_ok());
    
    let parsed: TestPerformanceDeepNested = result.unwrap();
    assert_eq!(parsed.level, 100);
    
    Ok(())
}

#[test]
fn test_performance_deep_nesting_serialization() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceDeepNested {
        pub level: u32,
        pub content: String,
        #[xml(tree)]
        pub child_deep: Option<Box<TestPerformanceDeepNested>>,
    }

    let mut nested = TestPerformanceDeepNested {
        level: 1,
        content: "Root".to_string(),
        child_deep: None,
    };
    
    for i in 2..=50 {
        nested = TestPerformanceDeepNested {
            level: i,
            content: format!("Level {}", i),
            child_deep: Some(Box::new(nested)),
        };
    }
    
    let xml = from_obj(&nested);
    
    let result: Result<TestPerformanceDeepNested, PError> = from_xml::<TestPerformanceDeepNested>(&xml);
    match result {
        Ok(parsed) => {
            assert_eq!(parsed.level, 50);
        }
        Err(e) => {
            panic!("Failed to parse XML: {:?}", e);
        }
    }
    
    Ok(())
}

#[test]
fn test_performance_deep_nesting_with_attributes() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceDeepNestedWithAttributes {
        #[xml(attribute)]
        pub level: u32,
        pub content: String,
        pub metadata: String,
        #[xml(tree)]
        pub child_deep: Option<Box<TestPerformanceDeepNestedWithAttributes>>,
    }

    fn generate_deep_nested_with_attributes_xml(depth: u32) -> String {
        let mut xml = String::new();
        xml.push_str(&format!("<TestPerformanceDeepNestedWithAttributes level=\"{}\">", depth));
        xml.push_str(&format!("<content>Level {}</content>", depth));
        xml.push_str(&format!("<metadata>Metadata for level {}</metadata>", depth));
        
        if depth > 1 {
            xml.push_str("<child_deep>");
            xml.push_str(&generate_deep_nested_with_attributes_xml(depth - 1));
            xml.push_str("</child_deep>");
        }
        
        xml.push_str("</TestPerformanceDeepNestedWithAttributes>");
        xml
    }

    let xml = generate_deep_nested_with_attributes_xml(25);
    let result = from_xml::<TestPerformanceDeepNestedWithAttributes>(&xml);
    assert!(result.is_ok());
    
    let parsed: TestPerformanceDeepNestedWithAttributes = result.unwrap();
    assert_eq!(parsed.level, 25);
    assert_eq!(parsed.content, "Level 25");
    
    Ok(())
}

#[test]
fn test_performance_deep_nesting_mixed_types() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceDeepNestedMixed {
        pub level: u32,
        pub content: String,
        pub count: i32,
        pub active: bool,
        pub tags: Vec<String>,
        #[xml(tree)]
        pub child_deep: Option<Box<TestPerformanceDeepNestedMixed>>,
    }

    fn generate_deep_nested_mixed_xml(depth: u32) -> String {
        let mut xml = String::new();
        xml.push_str("<TestPerformanceDeepNestedMixed>");
        xml.push_str(&format!("<level>{}</level>", depth));
        xml.push_str(&format!("<content>Level {}</content>", depth));
        xml.push_str(&format!("<count>{}</count>", depth as i32));
        xml.push_str(&format!("<active>{}</active>", depth % 2 == 0));
        xml.push_str("<tags>");
        xml.push_str(&format!("<tags>tag{}</tags>", depth));
        xml.push_str(&format!("<tags>level{}</tags>", depth));
        xml.push_str("</tags>");
        
        if depth > 1 {
            xml.push_str("<child_deep>");
            xml.push_str(&generate_deep_nested_mixed_xml(depth - 1));
            xml.push_str("</child_deep>");
        }
        
        xml.push_str("</TestPerformanceDeepNestedMixed>");
        xml
    }

    let xml = generate_deep_nested_mixed_xml(30);
    let result = from_xml::<TestPerformanceDeepNestedMixed>(&xml);
    assert!(result.is_ok());
    
    let parsed: TestPerformanceDeepNestedMixed = result.unwrap();
    assert_eq!(parsed.level, 30);
    assert_eq!(parsed.content, "Level 30");
    assert_eq!(parsed.count, 30);
    assert_eq!(parsed.active, true);
    assert_eq!(parsed.tags.len(), 2);
    
    Ok(())
}