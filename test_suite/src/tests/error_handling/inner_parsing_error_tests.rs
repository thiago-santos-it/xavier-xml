use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestInnerStruct {
    pub id: u32,
    pub name: String,
    #[xml(inner="item")]
    pub data: Vec<String>,
    #[xml(inner="number")]
    pub numbers: Vec<i32>,
}

#[test]
fn test_inner_serialization_deserialization() -> Result<(), PError> {
    let test_data = TestInnerStruct {
        id: 123,
        name: "Test".to_string(),
        data: vec!["value1".to_string(), "value2".to_string(), "value3".to_string()],
        numbers: vec![10, 20, 30],
    };
    
    let xml = from_obj(&test_data);
    
    // Verify that XML contains correct internal tags
    assert!(xml.contains("<data><item>value1</item><item>value2</item><item>value3</item></data>"));
    assert!(xml.contains("<numbers><number>10</number><number>20</number><number>30</number></numbers>"));
    
    // Test deserialization
    let parsed: TestInnerStruct = from_xml(&xml)?;
    
    assert_eq!(test_data, parsed);
    Ok(())
}

#[test]
fn test_inner_manual_xml() -> Result<(), PError> {
    let manual_xml = r#"
    <TestInnerStruct>
        <id>456</id>
        <name>Manual Test</name>
        <data>
            <item>manual1</item>
            <item>manual2</item>
        </data>
        <numbers>
            <number>100</number>
            <number>200</number>
            <number>300</number>
        </numbers>
    </TestInnerStruct>"#;
    
    let manual_parsed: TestInnerStruct = from_xml(manual_xml)?;
    
    assert_eq!(manual_parsed.id, 456);
    assert_eq!(manual_parsed.name, "Manual Test");
    assert_eq!(manual_parsed.data, vec!["manual1", "manual2"]);
    assert_eq!(manual_parsed.numbers, vec![100, 200, 300]);
    
    Ok(())
}

#[test]
fn test_inner_empty_collections() -> Result<(), PError> {
    let empty_xml = r#"
    <TestInnerStruct>
        <id>789</id>
        <name>Empty Test</name>
        <data></data>
        <numbers></numbers>
    </TestInnerStruct>"#;
    
    let parsed: TestInnerStruct = from_xml(empty_xml)?;
    
    assert_eq!(parsed.id, 789);
    assert_eq!(parsed.name, "Empty Test");
    assert_eq!(parsed.data, Vec::<String>::new());
    assert_eq!(parsed.numbers, Vec::<i32>::new());
    
    Ok(())
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestInnerFloat {
    #[xml(inner="price")]
    pub prices: Vec<f64>,
    #[xml(inner="flag")]
    pub flags: Vec<bool>,
}

#[test]
fn test_inner_different_types() -> Result<(), PError> {
    let test_data = TestInnerFloat {
        prices: vec![1.5, 2.7, 3.14],
        flags: vec![true, false, true],
    };
    
    let xml = from_obj(&test_data);
    
    let parsed: TestInnerFloat = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
} 