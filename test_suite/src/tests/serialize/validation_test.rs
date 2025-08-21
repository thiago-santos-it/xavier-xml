use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestValidationStruct {
    pub id: u32,
    pub name: String,
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
    println!("XML gerado: {}", xml);
    
    // Verificar se o XML é válido fazendo round-trip
    let parsed: TestValidationStruct = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    // Verificar se contém todas as tags necessárias
    assert!(xml.contains("<TestValidationStruct>"));
    assert!(xml.contains("</TestValidationStruct>"));
    assert!(xml.contains("<id>123</id>"));
    assert!(xml.contains("<name>Test Name</name>"));
    assert!(xml.contains("<items>"));
    assert!(xml.contains("<items>item1</items>"));
    assert!(xml.contains("<items>item2</items>"));
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
    println!("XML complexo: {}", xml);
    
    // Verificar estrutura hierárquica
    assert!(xml.contains("<TestComplexStructure>"));
    assert!(xml.contains("</TestComplexStructure>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Root</name>"));
    assert!(xml.contains("<attributes>"));
    assert!(xml.contains("<children>"));
    
    // Verificar se pode ser parseado de volta
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
    
    // Primeira serialização
    let xml1 = from_obj(&test_data);
    let parsed1: TestFormatting = from_xml(&xml1)?;
    
    // Segunda serialização do resultado parseado
    let xml2 = from_obj(&parsed1);
    let parsed2: TestFormatting = from_xml(&xml2)?;
    
    // Terceira serialização
    let xml3 = from_obj(&parsed2);
    let parsed3: TestFormatting = from_xml(&xml3)?;
    
    // Verificar se todas as iterações mantêm a integridade dos dados
    assert_eq!(test_data, parsed1);
    assert_eq!(parsed1, parsed2);
    assert_eq!(parsed2, parsed3);
    
    // Verificar se os XMLs são idênticos após o primeiro round-trip
    assert_eq!(xml2, xml3);
    
    Ok(())
}

#[test]
fn test_xml_encoding_validation() -> Result<(), PError> {
    let test_data = TestValidationStruct {
        id: 789,
        name: "Test with special chars: & < > \" '".to_string(),
        items: vec!["item & < > \" '".to_string(), "normal item".to_string()],
        metadata: Some("Metadata with & < > \" ' characters".to_string()),
    };
    
    let xml = from_obj(&test_data);
    println!("XML com caracteres especiais: {}", xml);
    
    // Verificar se caracteres especiais foram escapados corretamente
    assert!(xml.contains("&amp;"));
    assert!(xml.contains("&lt;"));
    assert!(xml.contains("&gt;"));
    assert!(xml.contains("&quot;"));
    assert!(xml.contains("&apos;"));
    
    // Verificar se pode ser parseado de volta
    let parsed: TestValidationStruct = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
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
    println!("XML com valores vazios: {}", xml);
    
    // Verificar se campos vazios são tratados corretamente
    assert!(xml.contains("<id>0</id>"));
    assert!(xml.contains("<name></name>"));
    assert!(xml.contains("<items></items>"));
    // metadata deve estar ausente quando é None
    
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
        float_val: 3.14159265358979323846, // Pi com alta precisão
        uint_val: 18446744073709551615, // u64::MAX
    };
    
    let xml = from_obj(&test_data);
    println!("XML com precisão numérica: {}", xml);
    
    let parsed: TestNumeric = from_xml(&xml)?;
    
    // Verificar se valores numéricos foram preservados
    assert_eq!(test_data.int_val, parsed.int_val);
    assert_eq!(test_data.uint_val, parsed.uint_val);
    
    // Para floats, verificar se a diferença é pequena (devido à precisão)
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
    println!("XML com valores booleanos: {}", xml);

    // Verificar se valores booleanos foram serializados corretamente
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
    println!("XML com coleções: {}", xml);
    
    // Verificar se todas as tags estão presentes
    for tag in &test_data.tags {
        assert!(xml.contains(&format!("<tags>{}</tags>", tag)));
    }
    
    // Verificar se pode ser parseado de volta
    let parsed: TestFormatting = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_xml_malformed_handling() {
    // Teste com XML malformado para verificar se o parser falha graciosamente
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
    println!("XML com espaços em branco: {}", xml);
    
    // Verificar se espaços em branco foram preservados
    assert!(xml.contains("<name>  Test Name with Spaces  </name>"));
    assert!(xml.contains("<items>  item1  </items>"));
    assert!(xml.contains("<items>  item2  </items>"));
    assert!(xml.contains("<metadata>  Metadata with spaces  </metadata>"));
    
    let parsed: TestValidationStruct = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
} 