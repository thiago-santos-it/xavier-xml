use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};

#[test]
fn test_xml_attributes_basic_roundtrip() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestXmlAttributesBasic {
        pub id: u32,
        #[xml(attribute, name="name")]
        pub name: String,
        #[xml(attribute, name="value")]
        pub value: i32,
        #[xml(attribute, name="flag")]
        pub flag: bool,
        pub content: String,
    }

    let test_data = TestXmlAttributesBasic {
        id: 1,
        name: "Test Name".to_string(),
        value: 42,
        flag: true,
        content: "Content here".to_string(),
    };
    
    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestXmlAttributesBasic"));
    assert!(xml.contains("name=\"Test Name\""));
    assert!(xml.contains("value=\"42\""));
    assert!(xml.contains("flag=\"true\""));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<content>Content here</content>"));
    
    let parsed: TestXmlAttributesBasic = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_xml_attributes_optional_roundtrip() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestXmlAttributesOptional {
        pub id: u32,
        #[xml(attribute, name="name")]
        pub name: Option<String>,
        #[xml(attribute, name="value")]
        pub value: Option<i32>,
        pub content: String,
    }

    let test_data_some = TestXmlAttributesOptional {
        id: 1,
        name: Some("Test Name".to_string()),
        value: Some(42),
        content: "Content".to_string(),
    };
    
    let xml = from_obj(&test_data_some);
    
    assert!(xml.contains("<TestXmlAttributesOptional"));
    assert!(xml.contains("name=\"Test Name\""));
    assert!(xml.contains("value=\"42\""));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<content>Content</content>"));
    
    let parsed: TestXmlAttributesOptional = from_xml(&xml)?;
    assert_eq!(test_data_some, parsed);

    let test_data_none = TestXmlAttributesOptional {
        id: 2,
        name: None,
        value: None,
        content: "Content Only".to_string(),
    };
    
    let xml_none = from_obj(&test_data_none);
    
    assert!(xml_none.contains("<TestXmlAttributesOptional"));
    assert!(!xml_none.contains("name="));
    assert!(!xml_none.contains("value="));
    assert!(xml_none.contains("<id>2</id>"));
    assert!(xml_none.contains("<content>Content Only</content>"));
    
    let parsed_none: TestXmlAttributesOptional = from_xml(&xml_none)?;
    assert_eq!(test_data_none, parsed_none);
    
    Ok(())
}

#[test]
fn test_xml_attributes_mixed_content() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestXmlAttributesMixed {
        #[xml(attribute, name="id")]
        pub id: u32,
        pub name: String,
        #[xml(attribute, name="enabled")]
        pub enabled: bool,
        #[xml(inner="item")]
        pub data: Vec<String>,
    }

    let test_data = TestXmlAttributesMixed {
        id: 1,
        name: "Mixed Test".to_string(),
        enabled: true,
        data: vec!["item1".to_string(), "item2".to_string()],
    };
    
    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestXmlAttributesMixed"));
    assert!(xml.contains("id=\"1\""));
    assert!(xml.contains("enabled=\"true\""));
    assert!(xml.contains("<name>Mixed Test</name>"));
    assert!(xml.contains("<data>"));
    assert!(xml.contains("<item>item1</item>"));
    assert!(xml.contains("<item>item2</item>"));
    
    let parsed: TestXmlAttributesMixed = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_xml_attributes_complex_types() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestXmlAttributesComplex {
        #[xml(attribute, name="id")]
        pub id: u32,
        #[xml(attribute, name="price")]
        pub price: f64,
        #[xml(attribute, name="active")]
        pub active: bool,
        pub name: String,
        pub description: String,
    }

    let test_data = TestXmlAttributesComplex {
        id: 1,
        price: 99.99,
        active: true,
        name: "Complex Test".to_string(),
        description: "Testing complex attribute types".to_string(),
    };
    
    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestXmlAttributesComplex"));
    assert!(xml.contains("id=\"1\""));
    assert!(xml.contains("price=\"99.99\""));
    assert!(xml.contains("active=\"true\""));
    assert!(xml.contains("<name>Complex Test</name>"));
    assert!(xml.contains("<description>Testing complex attribute types</description>"));
    
    let parsed: TestXmlAttributesComplex = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

