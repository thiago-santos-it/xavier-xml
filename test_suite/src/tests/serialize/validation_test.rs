use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError, encode};

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestValidationStruct {
    pub id: u32,
    pub name: String,
    #[xml(inner="item")]
    pub items: Vec<String>,
    pub metadata: Option<String>,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestComplexStructure {
    pub id: u32,
    pub name: String,
    pub attributes: Vec<KeyValue>,
    pub children: Vec<TestComplexStructure>,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct KeyValue {
    pub key: String,
    pub value: String,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestFormatting {
    pub id: u32,
    pub name: String,
    pub description: String,
    #[xml(inner="tag")]
    pub tags: Vec<String>,
}

#[test]
fn test_xml_validity() -> Result<(), PError> {
    let test_data = TestValidationStruct {
        id: 123,
        name: "Test Name".to_string(),
        items: vec!["item1".to_string(), "item2".to_string()],
        metadata: Some("Test metadata".to_string()),
    };
    
    let xml = from_obj(&test_data);

    let parsed: TestValidationStruct = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    // Verify that it contains all necessary tags
    assert!(xml.contains("<TestValidationStruct>"));
    assert!(xml.contains("</TestValidationStruct>"));
    assert!(xml.contains("<id>123</id>"));
    assert!(xml.contains("<name>Test Name</name>"));
    assert!(xml.contains("<items>"));
    assert!(xml.contains("<item>item1</item>"));
    assert!(xml.contains("<item>item2</item>"));
    assert!(xml.contains("<metadata>Test metadata</metadata>"));
    
    Ok(())
}

#[test]
fn test_xml_structure_validation() -> Result<(), PError> {
    let test_data = TestComplexStructure {
        id: 1,
        name: "Root".to_string(),
        attributes: vec![
            KeyValue { key: "type".to_string(), value: "root".to_string() },
            KeyValue { key: "version".to_string(), value: "1.0".to_string() },
        ],
        children: vec![
            TestComplexStructure {
                id: 2,
                name: "Child1".to_string(),
                attributes: vec![
                    KeyValue { key: "type".to_string(), value: "child".to_string() },
                ],
                children: vec![],
            },
            TestComplexStructure {
                id: 3,
                name: "Child2".to_string(),
                attributes: vec![],
                children: vec![],
            },
        ],
    };
    
    let xml = from_obj(&test_data);
    
    // Verify hierarchical structure
    assert!(xml.contains("<TestComplexStructure>"));
    assert!(xml.contains("</TestComplexStructure>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Root</name>"));
    assert!(xml.contains("<attributes>"));
    assert!(xml.contains("<children>"));
    
    // Verify that it can be parsed back
    let parsed: TestComplexStructure = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    assert_eq!(test_data.attributes.len(), parsed.attributes.len());
    assert_eq!(test_data.children.len(), parsed.children.len());
    
    Ok(())
}

#[test]
fn test_xml_round_trip_validation() -> Result<(), PError> {
    let test_data = TestFormatting {
        id: 456,
        name: "Round Trip Test".to_string(),
        description: "Testing round trip serialization and deserialization".to_string(),
        tags: vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()],
    };
    
    // First serialization
    let xml1 = from_obj(&test_data);
    let parsed1: TestFormatting = from_xml(&xml1)?;
    
    // Second serialization of parsed result
    let xml2 = from_obj(&parsed1);
    let parsed2: TestFormatting = from_xml(&xml2)?;
    
    // Third serialization
    let xml3 = from_obj(&parsed2);
    let parsed3: TestFormatting = from_xml(&xml3)?;
    
    // Verify that all iterations maintain data integrity
    assert_eq!(test_data, parsed1);
    assert_eq!(parsed1, parsed2);
    assert_eq!(parsed2, parsed3);
    
    // Verify that XMLs are identical after first round-trip
    assert_eq!(xml2, xml3);
    
    Ok(())
}

#[test]
fn test_xml_encoding_validation() -> Result<(), PError> {
    let test_data = TestValidationStruct {
        id: 789,
        name: encode!("Test with special chars: & < > \" '"),
        items: vec![
            encode!("item & < > \" '"),
            encode!("normal item")],
        metadata: Some(encode!("Metadata with & < > \" ' characters")),
    };
    
    let xml = from_obj(&test_data);
    
    // Verify that special characters were escaped correctly
    assert!(xml.contains("&amp;"));
    assert!(xml.contains("&lt;"));
    assert!(xml.contains("&gt;"));
    assert!(xml.contains("&quot;"));
    assert!(xml.contains("&apos;"));
    
    // Verify that it can be parsed back
    let _parsed: TestValidationStruct = from_xml(&xml)?;
    
    Ok(())
}

#[test]
fn test_xml_empty_values() -> Result<(), PError> {
    let test_data = TestValidationStruct {
        id: 0,
        name: "".to_string(),
        items: vec![],
        metadata: None,
    };
    
    let xml = from_obj(&test_data);
    
    // Verify that empty fields are handled correctly
    assert!(xml.contains("<id>0</id>"));
    assert!(xml.contains("<name></name>"));
    assert!(xml.contains("<items></items>"));
    // metadata should be absent when it's None
    
    let parsed: TestValidationStruct = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_xml_numeric_precision() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestNumeric {
        pub int_val: i64,
        pub float_val: f64,
        pub uint_val: u64,
    }
    
    let test_data = TestNumeric {
        int_val: -9223372036854775808, // i64::MIN
        float_val: 3.14159265358979323846, // Pi with high precision
        uint_val: 18446744073709551615, // u64::MAX
    };
    
    let xml = from_obj(&test_data);
    
    let parsed: TestNumeric = from_xml(&xml)?;
    
    // Verify that numeric values were preserved
    assert_eq!(test_data.int_val, parsed.int_val);
    assert_eq!(test_data.uint_val, parsed.uint_val);
    
    // For floats, verify that difference is small (due to precision)
    let diff = (test_data.float_val - parsed.float_val).abs();
    assert!(diff < 1e-15);
    
    Ok(())
}

#[test]
fn test_xml_boolean_values() -> Result<(), PError> {

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestBoolean {
        pub true_val: bool,
        pub false_val: bool,
        pub optional_true: Option<bool>,
        pub optional_false: Option<bool>,
        pub optional_none: Option<bool>,
    }

    let test_data = TestBoolean {
        true_val: true,
        false_val: false,
        optional_true: Some(true),
        optional_false: Some(false),
        optional_none: None,
    };
    
    let xml = from_obj(&test_data);

    // Verify that boolean values were serialized correctly
    assert!(xml.contains("<true_val>true</true_val>"));
    assert!(xml.contains("<false_val>false</false_val>"));
    assert!(xml.contains("<optional_true>true</optional_true>"));
    assert!(xml.contains("<optional_false>false</optional_false>"));
    
    let parsed: TestBoolean = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_xml_collection_validation() -> Result<(), PError> {
    let test_data = TestFormatting {
        id: 999,
        name: "Collection Test".to_string(),
        description: "Testing collections".to_string(),
        tags: vec![
            "tag1".to_string(),
            "tag2".to_string(),
            "tag3".to_string(),
            "tag4".to_string(),
            "tag5".to_string(),
        ],
    };
    
    let xml = from_obj(&test_data);
    
    // Verify that all tags are present
    for tag in &test_data.tags {
        assert!(xml.contains(&format!("<tag>{}</tag>", tag)));
    }
    
    // Verify that it can be parsed back
    let parsed: TestFormatting = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_xml_malformed_handling() {
    // Test with malformed XML to verify graceful failure of the parser
    let malformed_xml = r#"
    <TestValidationStruct>
        <id>123</id>
        <name>Test Name
        <items>
            <items>item1</items>
            <items>item2</items>
        </items>
        <metadata>Test metadata</metadata>
    </TestValidationStruct>"#;
    
    let result = from_xml::<TestValidationStruct>(malformed_xml);
    assert!(result.is_err());
}

#[test]
fn test_xml_whitespace_handling() -> Result<(), PError> {
    let test_data = TestValidationStruct {
        id: 123,
        name: "  Test Name with Spaces  ".to_string(),
        items: vec!["  item1  ".to_string(), "  item2  ".to_string()],
        metadata: Some("  Metadata with spaces  ".to_string()),
    };
    
    let xml = from_obj(&test_data);
    
    // Verify that whitespace was preserved
    assert!(xml.contains("<name>  Test Name with Spaces  </name>"));
    assert!(xml.contains("<item>  item1  </item>"));
    assert!(xml.contains("<item>  item2  </item>"));
    assert!(xml.contains("<metadata>  Metadata with spaces  </metadata>"));
    
    let parsed: TestValidationStruct = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
} 