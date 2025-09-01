use xavier::{declaration, encode, from_obj, from_xml, XmlSerializable, XmlDeserializable, PError};

#[test]
fn test_xml_declarations_basic_roundtrip() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable)]
    #[declaration]
    #[xml(name="test_declaration", case="Camel")]
    struct TestXmlDeclarationBasic {
        pub id: u32,
        pub name: String,
        pub description: String,
    }

    let test_data = TestXmlDeclarationBasic {
        id: 1,
        name: encode!("Test Declaration"),
        description: encode!("Test Description"),
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<testDeclaration>"));
    assert!(xml.contains("</testDeclaration>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Declaration</name>"));
    assert!(xml.contains("<description>Test Description</description>"));
    
    let (version, encoding, standalone) = declaration!(&xml)?;
    assert_eq!(version, "1.0");
    assert_eq!(encoding.unwrap(), "UTF-8");
    assert_eq!(standalone.unwrap(), "no");
    
    let parsed: TestXmlDeclarationBasic = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    assert_eq!(test_data.description, parsed.description);
    
    Ok(())
}

#[test]
fn test_xml_declarations_with_attributes() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable)]
    #[declaration]
    #[xml(name="test_declaration_attributes", case="Camel")]
    struct TestXmlDeclarationAttributes {
        pub id: u32,
        pub name: String,
        pub version: String,
        pub active: bool,
    }

    let test_data = TestXmlDeclarationAttributes {
        id: 1,
        name: encode!("Test Declaration Attributes"),
        version: encode!("1.0"),
        active: true,
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<testDeclarationAttributes>"));
    assert!(xml.contains("</testDeclarationAttributes>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Declaration Attributes</name>"));
    assert!(xml.contains("<version>1.0</version>"));
    assert!(xml.contains("<active>true</active>"));
    
    let (version, encoding, standalone) = declaration!(&xml)?;
    assert_eq!(version, "1.0");
    assert_eq!(encoding.unwrap(), "UTF-8");
    assert_eq!(standalone.unwrap(), "no");
    
    let parsed: TestXmlDeclarationAttributes = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    assert_eq!(test_data.version, parsed.version);
    assert_eq!(test_data.active, parsed.active);
    
    Ok(())
}

#[test]
fn test_xml_declarations_nested_structures() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable)]
    struct TestXmlDeclarationChild {
        pub id: u32,
        pub name: String,
    }

    #[derive(XmlSerializable, XmlDeserializable)]
    #[declaration]
    #[xml(name="test_declaration_nested", case="Camel")]
    struct TestXmlDeclarationNested {
        pub id: u32,
        pub name: String,
        pub children: Vec<TestXmlDeclarationChild>,
    }

    let test_data = TestXmlDeclarationNested {
        id: 1,
        name: encode!("Test Declaration Nested"),
        children: vec![
            TestXmlDeclarationChild { id: 1, name: "Child 1".to_string() },
            TestXmlDeclarationChild { id: 2, name: "Child 2".to_string() },
        ],
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<testDeclarationNested>"));
    assert!(xml.contains("</testDeclarationNested>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Declaration Nested</name>"));
    assert!(xml.contains("<children>"));
    assert!(xml.contains("<TestXmlDeclarationChild>"));
    
    let (version, encoding, standalone) = declaration!(&xml)?;
    assert_eq!(version, "1.0");
    assert_eq!(encoding.unwrap(), "UTF-8");
    assert_eq!(standalone.unwrap(), "no");
    
    let parsed: TestXmlDeclarationNested = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    assert_eq!(test_data.children.len(), parsed.children.len());
    
    Ok(())
}

#[test]
fn test_xml_declarations_manual_xml() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable)]
    #[declaration]
    #[xml(name="test_declaration_manual", case="Camel")]
    struct TestXmlDeclarationManual {
        pub id: u32,
        pub name: String,
        pub description: String,
    }

    let manual_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
    <testDeclarationManual>
        <id>1</id>
        <name>Manual Test</name>
        <description>Manual Description</description>
    </testDeclarationManual>"#;

    let obj: TestXmlDeclarationManual = from_xml(manual_xml)?;
    assert_eq!(obj.id, 1);
    assert_eq!(obj.name, "Manual Test");
    assert_eq!(obj.description, "Manual Description");
    
    Ok(())
}