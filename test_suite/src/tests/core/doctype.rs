use xavier::{doctype, encode, from_obj, from_xml, XmlSerializable, XmlDeserializable, PError};

#[test]
fn test_doctype_handling_basic_roundtrip() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable)]
    #[declaration]
    #[dtd = "Note.dtd"]
    #[xml(name="test_doctype")]
    struct TestDoctypeBasic {
        pub id: u32,
        pub name: String,
        pub description: String,
    }

    let test_data = TestDoctypeBasic { 
        id: 1,
        name: encode!("Test Doctype"),
        description: encode!("Test Description"),
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<test_doctype>"));
    assert!(xml.contains("</test_doctype>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Doctype</name>"));
    assert!(xml.contains("<description>Test Description</description>"));
    
    let (target, file) = doctype!(&xml)?;
    assert_eq!("test_doctype", target);
    assert_eq!("Note.dtd", file);
    
    let parsed: TestDoctypeBasic = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    assert_eq!(test_data.description, parsed.description);
    
    Ok(())
}

#[test]
fn test_doctype_handling_with_attributes() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable)]
    #[declaration]
    #[dtd = "Complex.dtd"]
    #[xml(name="test_doctype_complex")]
    struct TestDoctypeComplex {
        pub id: u32,
        pub name: String,
        pub version: String,
        pub active: bool,
    }

    let test_data = TestDoctypeComplex { 
        id: 1,
        name: encode!("Complex Doctype"),
        version: encode!("1.0"),
        active: true,
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<test_doctype_complex>"));
    assert!(xml.contains("</test_doctype_complex>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Complex Doctype</name>"));
    assert!(xml.contains("<version>1.0</version>"));
    assert!(xml.contains("<active>true</active>"));
    
    let (target, file) = doctype!(&xml)?;
    assert_eq!("test_doctype_complex", target);
    assert_eq!("Complex.dtd", file);
    
    let parsed: TestDoctypeComplex = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    assert_eq!(test_data.version, parsed.version);
    assert_eq!(test_data.active, parsed.active);
    
    Ok(())
}

#[test]
fn test_doctype_handling_nested_structures() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable)]
    struct TestDoctypeChild {
        pub id: u32,
        pub name: String,
    }

    #[derive(XmlSerializable, XmlDeserializable)]
    #[declaration]
    #[dtd = "Nested.dtd"]
    #[xml(name="test_doctype_nested")]
    struct TestDoctypeNested {
        pub id: u32,
        pub name: String,
        pub children: Vec<TestDoctypeChild>,
    }

    let test_data = TestDoctypeNested { 
        id: 1,
        name: encode!("Nested Doctype"),
        children: vec![
            TestDoctypeChild { id: 1, name: encode!("Child 1") },
            TestDoctypeChild { id: 2, name: encode!("Child 2") },
        ],
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<test_doctype_nested>"));
    assert!(xml.contains("</test_doctype_nested>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Nested Doctype</name>"));
    assert!(xml.contains("<children>"));
    assert!(xml.contains("<TestDoctypeChild>"));
    
    let (target, file) = doctype!(&xml)?;
    assert_eq!("test_doctype_nested", target);
    assert_eq!("Nested.dtd", file);
    
    let parsed: TestDoctypeNested = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    assert_eq!(test_data.children.len(), parsed.children.len());
    
    Ok(())
}