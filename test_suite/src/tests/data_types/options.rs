use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct XMLObject {
    #[xml(attribute)]
    pub attr_string: String,
    #[xml(attribute)]
    pub opt_attr_string: Option<String>,
    #[xml(attribute)]
    pub none_string: Option<String>,
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32,
    pub opt_some_string: Option<String>,
    pub opt_some_int: Option<i32>,
    pub opt_some_float: Option<f32>,
    pub none_some_string: Option<String>,
    pub none_some_int: Option<i32>,
    pub none_some_float: Option<f32>
}

#[test]
fn deserialize_with_all_fields() -> Result<(), PError> {
    let xml = r#"
    <XMLObject attr_string="Required Attr" opt_attr_string="Optional Attr">
        <some_string>Required String</some_string>
        <some_int>42</some_int>
        <some_float>3.14</some_float>
        <opt_some_string>Optional String</opt_some_string>
        <opt_some_int>123</opt_some_int>
        <opt_some_float>2.718</opt_some_float>
    </XMLObject>"#;
    
    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.attr_string, "Required Attr");
    assert_eq!(obj.opt_attr_string, Some("Optional Attr".to_string()));
    assert_eq!(obj.none_string, None);
    assert_eq!(obj.some_string, "Required String");
    assert_eq!(obj.some_int, 42);
    assert_eq!(obj.some_float, 3.14);
    assert_eq!(obj.opt_some_string, Some("Optional String".to_string()));
    assert_eq!(obj.opt_some_int, Some(123));
    assert_eq!(obj.opt_some_float, Some(2.718));
    assert_eq!(obj.none_some_string, None);
    assert_eq!(obj.none_some_int, None);
    assert_eq!(obj.none_some_float, None);
    Ok(())
}

#[test]
fn deserialize_with_missing_optional_fields() -> Result<(), PError> {
    let xml = r#"
    <XMLObject attr_string="Required Attr">
        <some_string>Required String</some_string>
        <some_int>42</some_int>
        <some_float>3.14</some_float>
    </XMLObject>"#;
    
    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.attr_string, "Required Attr");
    assert_eq!(obj.opt_attr_string, None);
    assert_eq!(obj.none_string, None);
    assert_eq!(obj.some_string, "Required String");
    assert_eq!(obj.some_int, 42);
    assert_eq!(obj.some_float, 3.14);
    assert_eq!(obj.opt_some_string, None);
    assert_eq!(obj.opt_some_int, None);
    assert_eq!(obj.opt_some_float, None);
    assert_eq!(obj.none_some_string, None);
    assert_eq!(obj.none_some_int, None);
    assert_eq!(obj.none_some_float, None);
    Ok(())
}

#[test]
fn deserialize_with_empty_optional_fields() -> Result<(), PError> {
    let xml = r#"
    <XMLObject attr_string="Required Attr">
        <some_string>Required String</some_string>
        <some_int>42</some_int>
        <some_float>3.14</some_float>
        <opt_some_string></opt_some_string>
        <opt_some_int></opt_some_int>
        <opt_some_float></opt_some_float>
    </XMLObject>"#;
    
    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.attr_string, "Required Attr");
    assert_eq!(obj.some_string, "Required String");
    assert_eq!(obj.some_int, 42);
    assert_eq!(obj.some_float, 3.14);
    // Empty fields should be handled appropriately
    Ok(())
}

#[test]
fn serialize_with_all_fields() {
    let test_data = XMLObject {
        attr_string: "Serialized Required Attr".to_string(),
        opt_attr_string: Some("Serialized Optional Attr".to_string()),
        none_string: None,
        some_string: "Serialized Required String".to_string(),
        some_int: 456,
        some_float: 1.618,
        opt_some_string: Some("Serialized Optional String".to_string()),
        opt_some_int: Some(789),
        opt_some_float: Some(2.718),
        none_some_string: None,
        none_some_int: None,
        none_some_float: None,
    };
    
    let result = from_obj(&test_data);
    assert!(!result.is_empty());
    assert!(result.contains("attr_string=\"Serialized Required Attr\""));
    assert!(result.contains("opt_attr_string=\"Serialized Optional Attr\""));
    assert!(!result.contains("none_string="));
    assert!(result.contains("<some_string>Serialized Required String</some_string>"));
    assert!(result.contains("<some_int>456</some_int>"));
    assert!(result.contains("<some_float>1.618</some_float>"));
    assert!(result.contains("<opt_some_string>Serialized Optional String</opt_some_string>"));
    assert!(result.contains("<opt_some_int>789</opt_some_int>"));
    assert!(result.contains("<opt_some_float>2.718</opt_some_float>"));
    assert!(!result.contains("<none_some_string>"));
    assert!(!result.contains("<none_some_int>"));
    assert!(!result.contains("<none_some_float>"));
}

#[test]
fn serialize_with_missing_optional_fields() {
    let test_data = XMLObject {
        attr_string: "Serialized Required Attr".to_string(),
        opt_attr_string: None,
        none_string: None,
        some_string: "Serialized Required String".to_string(),
        some_int: 456,
        some_float: 1.618,
        opt_some_string: None,
        opt_some_int: None,
        opt_some_float: None,
        none_some_string: None,
        none_some_int: None,
        none_some_float: None,
    };
    
    let result = from_obj(&test_data);
    assert!(!result.is_empty());
    assert!(result.contains("attr_string=\"Serialized Required Attr\""));
    assert!(!result.contains("opt_attr_string="));
    assert!(result.contains("<some_string>Serialized Required String</some_string>"));
    assert!(result.contains("<some_int>456</some_int>"));
    assert!(result.contains("<some_float>1.618</some_float>"));
    assert!(!result.contains("<opt_some_string>"));
    assert!(!result.contains("<opt_some_int>"));
    assert!(!result.contains("<opt_some_float>"));
}

#[test]
fn round_trip_with_all_fields() -> Result<(), PError> {
    let original = XMLObject {
        attr_string: "Round Trip Required Attr".to_string(),
        opt_attr_string: Some("Round Trip Optional Attr".to_string()),
        none_string: None,
        some_string: "Round Trip Required String".to_string(),
        some_int: 999,
        some_float: 2.236,
        opt_some_string: Some("Round Trip Optional String".to_string()),
        opt_some_int: Some(555),
        opt_some_float: Some(1.414),
        none_some_string: None,
        none_some_int: None,
        none_some_float: None,
    };
    
    let xml = from_obj(&original);
    let parsed: XMLObject = from_xml(&xml)?;
    assert_eq!(original, parsed);
    Ok(())
}

#[test]
fn round_trip_with_missing_optional_fields() -> Result<(), PError> {
    let original = XMLObject {
        attr_string: "Round Trip Required Attr".to_string(),
        opt_attr_string: None,
        none_string: None,
        some_string: "Round Trip Required String".to_string(),
        some_int: 777,
        some_float: 3.333,
        opt_some_string: None,
        opt_some_int: None,
        opt_some_float: None,
        none_some_string: None,
        none_some_int: None,
        none_some_float: None,
    };
    
    let xml = from_obj(&original);
    let parsed: XMLObject = from_xml(&xml)?;
    assert_eq!(original, parsed);
    Ok(())
}
