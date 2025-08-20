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
    println!("XML serializado: {}", xml);
    
    // Verificar se o XML contém as tags internas corretas
    assert!(xml.contains("<data><item>value1</item><item>value2</item><item>value3</item></data>"));
    assert!(xml.contains("<numbers><number>10</number><number>20</number><number>30</number></numbers>"));
    
    // Teste de deserialização
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
    println!("XML com tipos diferentes: {}", xml);
    
    let parsed: TestInnerFloat = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestInnerNested {
    pub id: u32,
    #[xml(inner="child")]
    pub children: Vec<TestInnerChild>,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestInnerChild {
    pub name: String,
    pub value: i32,
}

#[test]
fn test_inner_nested_structs() -> Result<(), PError> {
    let test_data = TestInnerNested {
        id: 123,
        children: vec![
            TestInnerChild { name: "Child1".to_string(), value: 10 },
            TestInnerChild { name: "Child2".to_string(), value: 20 },
        ],
    };
    
    let xml = from_obj(&test_data);
    println!("XML com structs aninhados: {}", xml);
    
    let parsed: TestInnerNested = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestInnerOption {
    pub id: u32,
    #[xml(inner="item")]
    pub data: Option<Vec<String>>,
}

#[test]
fn test_inner_option_collections() -> Result<(), PError> {
    // Teste com Some
    let test_data = TestInnerOption {
        id: 123,
        data: Some(vec!["value1".to_string(), "value2".to_string()]),
    };
    
    let xml = from_obj(&test_data);
    let parsed: TestInnerOption = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    // Teste com None
    let test_data_none = TestInnerOption {
        id: 456,
        data: None,
    };
    
    let xml_none = from_obj(&test_data_none);
    let parsed_none: TestInnerOption = from_xml(&xml_none)?;
    assert_eq!(test_data_none, parsed_none);
    
    Ok(())
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestInnerMixed {
    pub id: u32,
    #[xml(inner="tag")]
    pub tags: Vec<String>,
    pub normal_field: String,
    #[xml(inner="number")]
    pub numbers: Vec<i32>,
}

#[test]
fn test_inner_mixed_with_normal_fields() -> Result<(), PError> {
    let test_data = TestInnerMixed {
        id: 789,
        tags: vec!["tag1".to_string(), "tag2".to_string()],
        normal_field: "Normal Value".to_string(),
        numbers: vec![1, 2, 3],
    };
    
    let xml = from_obj(&test_data);
    println!("XML com campos mistos: {}", xml);
    
    let parsed: TestInnerMixed = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_inner_malformed_xml() {
    // XML com tag interna incorreta
    let malformed_xml = r#"
    <TestInnerStruct>
        <id>123</id>
        <name>Test</name>
        <data>
            <wrong_tag>value1</wrong_tag>
            <item>value2</item>
        </data>
        <numbers>
            <number>10</number>
        </numbers>
    </TestInnerStruct>"#;
    
    let result = from_xml::<TestInnerStruct>(malformed_xml);
    // Deve falhar porque <wrong_tag> não é reconhecido
    assert!(result.is_err());
}

#[test]
fn test_inner_missing_outer_tag() {
    // XML sem a tag externa do campo
    let missing_outer_xml = r#"
    <TestInnerStruct>
        <id>123</id>
        <name>Test</name>
        <item>value1</item>
        <item>value2</item>
        <numbers>
            <number>10</number>
        </numbers>
    </TestInnerStruct>"#;
    
    let result = from_xml::<TestInnerStruct>(missing_outer_xml);
    // Deve falhar porque <item> está fora de <data>
    assert!(result.is_err());
}