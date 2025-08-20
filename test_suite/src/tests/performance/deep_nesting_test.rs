use std::panic;
use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct DeepNestedStruct {
    pub level: u32,
    pub content: String,
    #[xml(tree)]
    pub child_deep: Option<Box<DeepNestedStruct>>,
}

fn generate_deep_nested_xml(depth: u32) -> String {
    let mut xml = String::new();
    xml.push_str("<DeepNestedStruct>");
    xml.push_str(&format!("<level>{}</level>", depth));
    xml.push_str(&format!("<content>Level {}</content>", depth));
    
    if depth > 1 {
        xml.push_str("<child_deep>");
        xml.push_str(&generate_deep_nested_xml(depth - 1));
        xml.push_str("</child_deep>");
    }
    
    xml.push_str("</DeepNestedStruct>");
    xml
}

#[test]
fn stress_deep_nesting() {
    let xml = generate_deep_nested_xml(50);
    let result = from_xml::<DeepNestedStruct>(&xml);
    assert!(result.is_ok());
    
    let parsed: DeepNestedStruct = result.unwrap();
    assert_eq!(parsed.level, 50);
}

#[test]
fn stress_deep_nesting_100_levels() {
    let xml = generate_deep_nested_xml(100);
    let result = from_xml::<DeepNestedStruct>(&xml);
    assert!(result.is_ok());
    
    let parsed: DeepNestedStruct = result.unwrap();
    assert_eq!(parsed.level, 100);
}

#[test]
fn stress_deep_nesting_serialization() {

    let mut nested = DeepNestedStruct {
        level: 1,
        content: "Root".to_string(),
        child_deep: None,
    };
    
    for i in 2..=50 {
        nested = DeepNestedStruct {
            level: i,
            content: format!("Level {}", i),
            child_deep: Some(Box::new(nested)),
        };
    }
    
    let xml = from_obj(&nested);
    
    let result: Result<DeepNestedStruct, PError> = from_xml::<DeepNestedStruct>(&xml);
    match result {
        Ok(parsed) => {
            assert_eq!(parsed.level, 50);
        }
        Err(e) => {
            panic!("Failed to parse XML: {:?}", e);
        }
    }
}