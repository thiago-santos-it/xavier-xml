use xavier::{from_xml, from_obj, PError, XmlDeserializable, XmlSerializable, encode};

#[test]
fn test_deserialization_basic_roundtrip() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    struct TestDeserializationBasic {
        pub id: u32,
        pub name: String,
        pub value: i32,
    }

    let test_data = TestDeserializationBasic {
        id: 1,
        name: "Test Basic Deserialization".to_string(),
        value: 42,
    };

    let xml = from_obj(&test_data);
    
    let parsed: TestDeserializationBasic = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_deserialization_with_xml_declaration() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    struct TestDeserializationWithDeclaration {
        pub id: u32,
        pub name: String,
        pub description: String,
    }

    let xml = r#"<?xml version="1.0" encoding="UTF-8" ?>
                       <TestDeserializationWithDeclaration>
                            <id>1</id>
                            <name>Test Name</name>
                            <description>Test Description</description>
                       </TestDeserializationWithDeclaration>"#;
    
    let obj: TestDeserializationWithDeclaration = from_xml(&xml)?;
    assert_eq!(obj.id, 1);
    assert_eq!(obj.name, "Test Name");
    assert_eq!(obj.description, "Test Description");
    
    Ok(())
}

#[test]
fn test_deserialization_with_doctype() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    struct TestDeserializationWithDoctype {
        pub id: u32,
        pub name: String,
        pub version: String,
    }

    let xml = r#"<!DOCTYPE xml SYSTEM "Note.dtd">
                       <TestDeserializationWithDoctype>
                            <id>2</id>
                            <name>Test Name</name>
                            <version>1.0</version>
                       </TestDeserializationWithDoctype>"#;
    
    let obj: TestDeserializationWithDoctype = from_xml(&xml)?;
    assert_eq!(obj.id, 2);
    assert_eq!(obj.name, "Test Name");
    assert_eq!(obj.version, "1.0");
    
    Ok(())
}

#[test]
fn test_deserialization_with_processing_instructions() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    struct TestDeserializationWithPI {
        pub id: u32,
        pub name: String,
        pub metadata: String,
    }

    let xml = r#"<?PI Some Thing In The Way...?>
                       <TestDeserializationWithPI>
                            <id>3</id>
                            <name>Test Name</name>
                            <metadata>Some metadata</metadata>
                       </TestDeserializationWithPI>"#;
    
    let obj: TestDeserializationWithPI = from_xml(&xml)?;
    assert_eq!(obj.id, 3);
    assert_eq!(obj.name, "Test Name");
    assert_eq!(obj.metadata, "Some metadata");
    
    Ok(())
}

#[test]
fn test_deserialization_with_comments() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    struct TestDeserializationWithComments {
        pub id: u32,
        pub name: String,
        pub description: String,
    }

    let xml = r#"<!-- This section must be ignored -->
                       <TestDeserializationWithComments>
                            <id>4</id>
                            <name>Test Name</name>
                            <description>Test Description</description>
                       </TestDeserializationWithComments>
                       <!-- This section must be ignored -->"#;
    
    let obj: TestDeserializationWithComments = from_xml(&xml)?;
    assert_eq!(obj.id, 4);
    assert_eq!(obj.name, "Test Name");
    assert_eq!(obj.description, "Test Description");
    
    Ok(())
}

#[test]
fn test_deserialization_mixed_content() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    struct TestDeserializationMixedContent {
        pub id: u32,
        pub name: String,
        pub active: bool,
        pub count: i32,
        pub price: f64,
    }

    let xml = r#"<?xml version="1.0" encoding="UTF-8" ?>
                 <!-- Mixed content test -->
                 <TestDeserializationMixedContent>
                      <id>5</id>
                      <name>Mixed Content Test</name>
                      <active>true</active>
                      <count>100</count>
                      <price>99.99</price>
                 </TestDeserializationMixedContent>
                 <!-- End of test -->"#;
    
    let obj: TestDeserializationMixedContent = from_xml(&xml)?;
    assert_eq!(obj.id, 5);
    assert_eq!(obj.name, "Mixed Content Test");
    assert_eq!(obj.active, true);
    assert_eq!(obj.count, 100);
    assert_eq!(obj.price, 99.99);
    
    Ok(())
}


#[test]
fn test_serialization_basic_roundtrip() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestSerializationBasic {
        pub id: u32,
        pub name: String,
        pub value: i32,
    }

    let test_data = TestSerializationBasic {
        id: 1,
        name: "Test Basic Serialization".to_string(),
        value: 42,
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestSerializationBasic>"));
    assert!(xml.contains("</TestSerializationBasic>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Basic Serialization</name>"));
    assert!(xml.contains("<value>42</value>"));

    let parsed: TestSerializationBasic = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

    Ok(())
}

#[test]
fn test_serialization_with_special_characters() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestSerializationSpecialChars {
        pub id: u32,
        pub description: String,
        pub count: i32,
        pub price: f32,
    }

    let test_data = TestSerializationSpecialChars {
        id: 2,
        description: encode!("Text with & < > \" ' characters"),
        count: -42,
        price: 3.15158,
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestSerializationSpecialChars>"));
    assert!(xml.contains("</TestSerializationSpecialChars>"));
    assert!(xml.contains("<id>2</id>"));
    assert!(xml.contains("Text with &amp; &lt; &gt; &quot; &apos; characters"));
    assert!(xml.contains("<count>-42</count>"));
    assert!(xml.contains("<price>3.15158</price>"));

    let parsed: TestSerializationSpecialChars = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.count, parsed.count);
    assert_eq!(test_data.price, parsed.price);

    Ok(())
}

#[test]
fn test_serialization_edge_cases() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestSerializationEdgeCases {
        pub id: u32,
        pub empty_string: String,
        pub min_int: i32,
        pub negative_infinity: f32,
    }

    let test_data = TestSerializationEdgeCases {
        id: 3,
        empty_string: encode!(""),
        min_int: i32::MIN,
        negative_infinity: f32::NEG_INFINITY,
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestSerializationEdgeCases>"));
    assert!(xml.contains("</TestSerializationEdgeCases>"));
    assert!(xml.contains("<id>3</id>"));
    assert!(xml.contains("<empty_string></empty_string>"));
    assert!(xml.contains(&format!("<min_int>{}</min_int>", i32::MIN)));
    assert!(xml.contains("<negative_infinity>-inf</negative_infinity>"));

    let parsed: TestSerializationEdgeCases = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

    Ok(())
}

#[test]
fn test_serialization_mixed_types() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestSerializationMixedTypes {
        pub id: u32,
        pub name: String,
        pub active: bool,
        pub count: i32,
        pub price: f64,
        pub metadata: Option<String>,
    }

    let test_data = TestSerializationMixedTypes {
        id: 4,
        name: "Mixed Types Test".to_string(),
        active: true,
        count: 100,
        price: 99.99,
        metadata: Some("Some metadata".to_string()),
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestSerializationMixedTypes>"));
    assert!(xml.contains("</TestSerializationMixedTypes>"));
    assert!(xml.contains("<id>4</id>"));
    assert!(xml.contains("<name>Mixed Types Test</name>"));
    assert!(xml.contains("<active>true</active>"));
    assert!(xml.contains("<count>100</count>"));
    assert!(xml.contains("<price>99.99</price>"));
    assert!(xml.contains("<metadata>Some metadata</metadata>"));

    let parsed: TestSerializationMixedTypes = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

    Ok(())
}

#[test]
fn test_advanced_serde_inner_roundtrip() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestAdvancedSerdeInner {
        pub id: u32,
        pub name: String,
        #[xml(inner="item")]
        pub items: Vec<String>,
        pub metadata: Option<String>,
    }

    let test_data = TestAdvancedSerdeInner {
        id: 1,
        name: "Test Advanced Serde Inner".to_string(),
        items: vec!["item1".to_string(), "item2".to_string()],
        metadata: Some("Test metadata".to_string()),
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestAdvancedSerdeInner>"));
    assert!(xml.contains("</TestAdvancedSerdeInner>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Advanced Serde Inner</name>"));
    assert!(xml.contains("<items>"));
    assert!(xml.contains("<item>item1</item>"));
    assert!(xml.contains("<item>item2</item>"));
    assert!(xml.contains("<metadata>Test metadata</metadata>"));

    let parsed: TestAdvancedSerdeInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

    Ok(())
}

#[test]
fn test_advanced_serde_complex_roundtrip() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestAdvancedSerdeKeyValue {
        pub key: String,
        pub value: String,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestAdvancedSerdeComplex {
        pub id: u32,
        pub name: String,
        pub attributes: Vec<TestAdvancedSerdeKeyValue>,
        pub children: Vec<TestAdvancedSerdeComplex>,
    }

    let test_data = TestAdvancedSerdeComplex {
        id: 1,
        name: "Root".to_string(),
        attributes: vec![
            TestAdvancedSerdeKeyValue { key: "type".to_string(), value: "root".to_string() },
            TestAdvancedSerdeKeyValue { key: "version".to_string(), value: "1.0".to_string() },
        ],
        children: vec![
            TestAdvancedSerdeComplex {
                id: 2,
                name: "Child1".to_string(),
                attributes: vec![
                    TestAdvancedSerdeKeyValue { key: "type".to_string(), value: "child".to_string() },
                ],
                children: vec![],
            },
            TestAdvancedSerdeComplex {
                id: 3,
                name: "Child2".to_string(),
                attributes: vec![],
                children: vec![],
            },
        ],
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestAdvancedSerdeComplex>"));
    assert!(xml.contains("</TestAdvancedSerdeComplex>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Root</name>"));
    assert!(xml.contains("<attributes>"));
    assert!(xml.contains("<children>"));

    let parsed: TestAdvancedSerdeComplex = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

    Ok(())
}

#[test]
fn test_advanced_serde_formatted_roundtrip() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestAdvancedSerdeFormatted {
        pub id: u32,
        pub name: String,
        pub description: String,
        #[xml(inner="tag")]
        pub tags: Vec<String>,
    }

    let test_data = TestAdvancedSerdeFormatted {
        id: 1,
        name: "Formatted Test".to_string(),
        description: "A formatted test object".to_string(),
        tags: vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()],
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestAdvancedSerdeFormatted>"));
    assert!(xml.contains("</TestAdvancedSerdeFormatted>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Formatted Test</name>"));
    assert!(xml.contains("<description>A formatted test object</description>"));
    assert!(xml.contains("<tags>"));
    assert!(xml.contains("<tag>tag1</tag>"));
    assert!(xml.contains("<tag>tag2</tag>"));
    assert!(xml.contains("<tag>tag3</tag>"));

    let parsed: TestAdvancedSerdeFormatted = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

    Ok(())
}

#[test]
fn test_advanced_serde_nested_structures() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestAdvancedSerdeNestedChild {
        pub id: u32,
        pub name: String,
        pub value: String,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestAdvancedSerdeNested {
        pub id: u32,
        pub name: String,
        pub children: Vec<TestAdvancedSerdeNestedChild>,
        pub metadata: Option<String>,
    }

    let test_data = TestAdvancedSerdeNested {
        id: 1,
        name: "Nested Test".to_string(),
        children: vec![
            TestAdvancedSerdeNestedChild {
                id: 1,
                name: "Child A".to_string(),
                value: "Value A".to_string(),
            },
            TestAdvancedSerdeNestedChild {
                id: 2,
                name: "Child B".to_string(),
                value: "Value B".to_string(),
            },
        ],
        metadata: Some("Nested metadata".to_string()),
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestAdvancedSerdeNested>"));
    assert!(xml.contains("</TestAdvancedSerdeNested>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Nested Test</name>"));
    assert!(xml.contains("<children>"));
    assert!(xml.contains("<TestAdvancedSerdeNestedChild>"));
    assert!(xml.contains("<metadata>Nested metadata</metadata>"));

    let parsed: TestAdvancedSerdeNested = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

    Ok(())
}

#[test]
fn test_advanced_serde_mixed_types() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestAdvancedSerdeMixed {
        pub id: u32,
        pub name: String,
        pub active: bool,
        pub count: i32,
        pub price: f64,
        #[xml(inner="item")]
        pub items: Vec<String>,
        pub metadata: Option<String>,
    }

    let test_data = TestAdvancedSerdeMixed {
        id: 1,
        name: "Mixed Types Test".to_string(),
        active: true,
        count: 42,
        price: 99.99,
        items: vec!["item1".to_string(), "item2".to_string()],
        metadata: Some("Mixed metadata".to_string()),
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestAdvancedSerdeMixed>"));
    assert!(xml.contains("</TestAdvancedSerdeMixed>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Mixed Types Test</name>"));
    assert!(xml.contains("<active>true</active>"));
    assert!(xml.contains("<count>42</count>"));
    assert!(xml.contains("<price>99.99</price>"));
    assert!(xml.contains("<items>"));
    assert!(xml.contains("<item>item1</item>"));
    assert!(xml.contains("<item>item2</item>"));
    assert!(xml.contains("<metadata>Mixed metadata</metadata>"));

    let parsed: TestAdvancedSerdeMixed = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

    Ok(())
}