use xavier::{from_obj, from_xml, XmlSerializable, XmlDeserializable, PError};

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestVecInner {
    pub id: u32,
    pub name: String,
    #[xml(inner="item")]
    pub items: Vec<String>,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestOptionVecInner {
    pub id: u32,
    pub name: String,
    #[xml(inner="inner_item")]
    pub items: Option<Vec<String>>,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestComplexVecInner {
    pub id: u32,
    pub name: String,
    #[xml(inner="inner_item")]
    pub children: Vec<ChildItem>,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct ChildItem {
    pub value: String,
    pub count: i32,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestOptionComplexVecInner {
    pub id: u32,
    pub name: String,
    pub children: Option<Vec<ChildItem>>,
}

#[test]
fn serialize_and_deserialize_vec_inner() -> Result<(), PError> {
    let test_data = TestVecInner {
        id: 1,
        name: "Test Collection".to_string(),
        items: vec!["item1".to_string(), "item2".to_string(), "item3".to_string()],
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestVecInner>"));
    assert!(xml.contains("</TestVecInner>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Collection</name>"));
    
    assert!(xml.contains("<items>"));
    assert!(xml.contains("</items>"));
    assert!(xml.contains("<item>item1</item>"));
    assert!(xml.contains("<item>item2</item>"));
    assert!(xml.contains("<item>item3</item>"));

    let parsed: TestVecInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn serialize_and_deserialize_option_vec_inner_some() -> Result<(), PError> {
    let test_data = TestOptionVecInner {
        id: 2,
        name: "Test Option Collection".to_string(),
        items: Some(vec!["option1".to_string(), "option2".to_string()]),
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestOptionVecInner>"));
    assert!(xml.contains("</TestOptionVecInner>"));
    assert!(xml.contains("<id>2</id>"));
    assert!(xml.contains("<name>Test Option Collection</name>"));
    
    assert!(xml.contains("<items>"));
    assert!(xml.contains("</items>"));
    assert!(xml.contains("<item>option1</item>"));
    assert!(xml.contains("<item>option2</item>"));
    
    let parsed: TestOptionVecInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn serialize_and_deserialize_option_vec_inner_none() -> Result<(), PError> {
    let test_data = TestOptionVecInner {
        id: 3,
        name: "Test Option Collection None".to_string(),
        items: None,
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestOptionVecInner>"));
    assert!(xml.contains("</TestOptionVecInner>"));
    assert!(xml.contains("<id>3</id>"));
    assert!(xml.contains("<name>Test Option Collection None</name>"));
    
    assert!(!xml.contains("<items>"));
    assert!(!xml.contains("</items>"));
    assert!(!xml.contains("<item>"));
    
    let parsed: TestOptionVecInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn serialize_and_deserialize_complex_vec_inner() -> Result<(), PError> {
    let test_data = TestComplexVecInner {
        id: 4,
        name: "Test Complex Collection".to_string(),
        children: vec![
            ChildItem { value: "child1".to_string(), count: 10 },
            ChildItem { value: "child2".to_string(), count: 20 },
        ],
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestComplexVecInner>"));
    assert!(xml.contains("</TestComplexVecInner>"));
    assert!(xml.contains("<id>4</id>"));
    assert!(xml.contains("<name>Test Complex Collection</name>"));
    
    assert!(xml.contains("<children>"));
    assert!(xml.contains("</children>"));
    assert!(xml.contains("<inner_item>"));
    assert!(xml.contains("</inner_item>"));
    assert!(xml.contains("<value>child1</value>"));
    assert!(xml.contains("<count>10</count>"));
    assert!(xml.contains("<value>child2</value>"));
    assert!(xml.contains("<count>20</count>"));

    let parsed: TestComplexVecInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn serialize_and_deserialize_option_complex_vec_inner_some() -> Result<(), PError> {
    let test_data = TestOptionComplexVecInner {
        id: 5,
        name: "Test Option Complex Collection".to_string(),
        children: Some(vec![
            ChildItem { value: "complex1".to_string(), count: 100 },
            ChildItem { value: "complex2".to_string(), count: 200 },
        ]),
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestOptionComplexVecInner>"));
    assert!(xml.contains("</TestOptionComplexVecInner>"));
    assert!(xml.contains("<id>5</id>"));
    assert!(xml.contains("<name>Test Option Complex Collection</name>"));
    
    assert!(xml.contains("<children>"));
    assert!(xml.contains("</children>"));
    assert!(xml.contains("<inner_item>"));
    assert!(xml.contains("</inner_item>"));
    assert!(xml.contains("<value>complex1</value>"));
    assert!(xml.contains("<count>100</count>"));
    assert!(xml.contains("<value>complex2</value>"));
    assert!(xml.contains("<count>200</count>"));
    
    let parsed: TestOptionComplexVecInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn serialize_and_deserialize_option_complex_vec_inner_none() -> Result<(), PError> {
    let test_data = TestOptionComplexVecInner {
        id: 6,
        name: "Test Option Complex Collection None".to_string(),
        children: None,
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestOptionComplexVecInner>"));
    assert!(xml.contains("</TestOptionComplexVecInner>"));
    assert!(xml.contains("<id>6</id>"));
    assert!(xml.contains("<name>Test Option Complex Collection None</name>"));
    
    assert!(!xml.contains("<children>"));
    assert!(!xml.contains("</children>"));
    assert!(!xml.contains("<child_item>"));
    
    let parsed: TestOptionComplexVecInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn serialize_and_deserialize_empty_vec_inner() -> Result<(), PError> {
    let test_data = TestVecInner {
        id: 7,
        name: "Test Empty Collection".to_string(),
        items: vec![],
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestVecInner>"));
    assert!(xml.contains("</TestVecInner>"));
    assert!(xml.contains("<id>7</id>"));
    assert!(xml.contains("<name>Test Empty Collection</name>"));
    
    assert!(xml.contains("<items>"));
    assert!(xml.contains("</items>"));
    assert!(!xml.contains("<item>"));
    
    let parsed: TestVecInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn serialize_and_deserialize_empty_option_vec_inner() -> Result<(), PError> {
    let test_data = TestOptionVecInner {
        id: 8,
        name: "Test Empty Option Collection".to_string(),
        items: Some(vec![]),
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestOptionVecInner>"));
    assert!(xml.contains("</TestOptionVecInner>"));
    assert!(xml.contains("<id>8</id>"));
    assert!(xml.contains("<name>Test Empty Option Collection</name>"));
    
    assert!(xml.contains("<items>"));
    assert!(xml.contains("</items>"));
    assert!(!xml.contains("<item>"));
    
    let parsed: TestOptionVecInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

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