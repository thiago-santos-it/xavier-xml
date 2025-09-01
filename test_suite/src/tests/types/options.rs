use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};

#[test]
fn test_options_basic_roundtrip() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestOptionsBasic {
        pub id: u32,
        pub name: String,
        pub description: Option<String>,
        pub count: Option<i32>,
        pub price: Option<f64>,
        pub active: Option<bool>,
    }

    let test_data = TestOptionsBasic {
        id: 1,
        name: "Test Options".to_string(),
        description: Some("Test Description".to_string()),
        count: Some(42),
        price: Some(3.14),
        active: Some(true),
    };

    
    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestOptionsBasic>"));
    assert!(xml.contains("</TestOptionsBasic>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Options</name>"));
    assert!(xml.contains("<description>Test Description</description>"));
    assert!(xml.contains("<count>42</count>"));
    assert!(xml.contains("<price>3.14</price>"));
    assert!(xml.contains("<active>true</active>"));
    
    
    let parsed: TestOptionsBasic = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_options_with_none_values() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestOptionsNone {
        pub id: u32,
        pub name: String,
        pub description: Option<String>,
        pub count: Option<i32>,
        pub price: Option<f64>,
        pub active: Option<bool>,
    }

    let test_data = TestOptionsNone {
        id: 1,
        name: "Test Options None".to_string(),
        description: None,
        count: None,
        price: None,
        active: None,
    };

    
    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestOptionsNone>"));
    assert!(xml.contains("</TestOptionsNone>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Options None</name>"));
    
    assert!(!xml.contains("<description>"));
    assert!(!xml.contains("<count>"));
    assert!(!xml.contains("<price>"));
    assert!(!xml.contains("<active>"));
    
    
    let parsed: TestOptionsNone = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_options_with_attributes() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestOptionsWithAttributes {
        #[xml(attribute)]
        pub attr_string: String,
        #[xml(attribute)]
        pub opt_attr_string: Option<String>,
        #[xml(attribute)]
        pub none_string: Option<String>,
        pub id: u32,
        pub name: String,
        pub description: Option<String>,
    }

    let test_data = TestOptionsWithAttributes {
        attr_string: "Required Attr".to_string(),
        opt_attr_string: Some("Optional Attr".to_string()),
        none_string: None,
        id: 1,
        name: "Test With Attributes".to_string(),
        description: Some("Test Description".to_string()),
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestOptionsWithAttributes"));
    assert!(xml.contains("</TestOptionsWithAttributes>"));
    assert!(xml.contains("attr_string=\"Required Attr\""));
    assert!(xml.contains("opt_attr_string=\"Optional Attr\""));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test With Attributes</name>"));
    assert!(xml.contains("<description>Test Description</description>"));

    let parsed: TestOptionsWithAttributes = from_xml(&xml)?;
    assert_eq!(test_data.attr_string, parsed.attr_string);
    assert_eq!(test_data.opt_attr_string, parsed.opt_attr_string);
    assert_eq!(test_data.none_string, parsed.none_string);
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    assert_eq!(test_data.description, parsed.description);
    
    Ok(())
}

#[test]
fn test_options_mixed_types() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestOptionsMixedTypes {
        pub id: u32,
        pub name: String,
        pub some_string: String,
        pub some_int: i32,
        pub some_float: f32,
        pub opt_some_string: Option<String>,
        pub opt_some_int: Option<i32>,
        pub opt_some_float: Option<f32>,
        pub none_some_string: Option<String>,
        pub none_some_int: Option<i32>,
        pub none_some_float: Option<f32>,
    }

    let test_data = TestOptionsMixedTypes {
        id: 1,
        name: "Mixed Types".to_string(),
        some_string: "Required String".to_string(),
        some_int: 42,
        some_float: 3.14,
        opt_some_string: Some("Optional String".to_string()),
        opt_some_int: Some(123),
        opt_some_float: Some(2.718),
        none_some_string: None,
        none_some_int: None,
        none_some_float: None,
    };

    
    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestOptionsMixedTypes>"));
    assert!(xml.contains("</TestOptionsMixedTypes>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Mixed Types</name>"));
    assert!(xml.contains("<some_string>Required String</some_string>"));
    assert!(xml.contains("<some_int>42</some_int>"));
    assert!(xml.contains("<some_float>3.14</some_float>"));
    assert!(xml.contains("<opt_some_string>Optional String</opt_some_string>"));
    assert!(xml.contains("<opt_some_int>123</opt_some_int>"));
    assert!(xml.contains("<opt_some_float>2.718</opt_some_float>"));
    
    assert!(!xml.contains("<none_some_string>"));
    assert!(!xml.contains("<none_some_int>"));
    assert!(!xml.contains("<none_some_float>"));
    
    
    let parsed: TestOptionsMixedTypes = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    assert_eq!(test_data.some_string, parsed.some_string);
    assert_eq!(test_data.some_int, parsed.some_int);
    assert_eq!(test_data.some_float, parsed.some_float);
    assert_eq!(test_data.opt_some_string, parsed.opt_some_string);
    assert_eq!(test_data.opt_some_int, parsed.opt_some_int);
    assert_eq!(test_data.opt_some_float, parsed.opt_some_float);
    assert_eq!(test_data.none_some_string, parsed.none_some_string);
    assert_eq!(test_data.none_some_int, parsed.none_some_int);
    assert_eq!(test_data.none_some_float, parsed.none_some_float);
    
    Ok(())
}

#[test]
fn test_options_empty_fields() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestOptionsEmptyFields {
        pub id: u32,
        pub name: String,
        pub description: Option<String>,
        pub count: Option<i32>,
        pub price: Option<f64>,
    }

    let xml = r#"
    <TestOptionsEmptyFields>
        <id>1</id>
        <name>Empty Fields</name>
        <description></description>
        <count></count>
        <price></price>
    </TestOptionsEmptyFields>"#;
    
    let obj: TestOptionsEmptyFields = from_xml(&xml)?;
    assert_eq!(obj.id, 1);
    assert_eq!(obj.name, "Empty Fields");
    
    assert_eq!(obj.description, Some("".to_string()));
    assert_eq!(obj.count, Some(0));
    assert_eq!(obj.price, Some(0.0));
    
    Ok(())
}
