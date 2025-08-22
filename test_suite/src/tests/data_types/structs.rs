use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
#[xml(name="child")]
struct Child {
    #[xml(attribute, name="attr")]
    pub attribute: String,
    #[xml(value)]
    pub value: String
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct XMLObject {
    #[xml(attribute, name="attr")]
    pub attribute: String,
    pub field_a: String,
    #[xml(tree)]
    pub child: Child
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct XMLObjectUnit;

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct XMLObjectWithEmptyTag {
    pub field_a: String,
    #[xml(flatten)]
    pub empty: Option<XMLObjectUnit>
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
pub struct XMLObjectSimple {
    pub field_a: String,
    pub field_b: i32,
    pub field_c: f64,
}

#[test]
fn deserialize_struct_with_attributes() -> Result<(), PError> {
    let xml = r#"
    <XMLObject attr="Attribute Value">
        <field_a>Field Value</field_a>
        <child attr="Child Attr">Child Value</child>
    </XMLObject>"#;
    
    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.attribute, "Attribute Value");
    assert_eq!(obj.field_a, "Field Value");
    assert_eq!(obj.child.attribute, "Child Attr");
    assert_eq!(obj.child.value, "Child Value");
    Ok(())
}

#[test]
fn deserialize_unit_struct() -> Result<(), PError> {
    let xml = r#"<XMLObjectUnit></XMLObjectUnit>"#;
    let obj: XMLObjectUnit = from_xml(&xml)?;
    assert_eq!(obj, XMLObjectUnit);
    Ok(())
}

#[test]
fn deserialize_simple_struct() -> Result<(), PError> {
    let xml = r#"
    <XMLObjectSimple>
        <field_a>Test String</field_a>
        <field_b>42</field_b>
        <field_c>3.14</field_c>
    </XMLObjectSimple>"#;
    
    let obj: XMLObjectSimple = from_xml(&xml)?;
    assert_eq!(obj.field_a, "Test String");
    assert_eq!(obj.field_b, 42);
    assert_eq!(obj.field_c, 3.14);
    Ok(())
}

#[test]
fn serialize_struct_with_attributes() {
    let test_data = XMLObject {
        attribute: "Serialized Attr".to_string(),
        field_a: "Serialized Field".to_string(),
        child: Child {
            attribute: "Serialized Child Attr".to_string(),
            value: "Serialized Child Value".to_string(),
        },
    };
    
    let result = from_obj(&test_data);
    assert!(!result.is_empty());
    assert!(result.contains("attr=\"Serialized Attr\""));
    assert!(result.contains("<field_a>Serialized Field</field_a>"));
    assert!(result.contains("attr=\"Serialized Child Attr\""));
    assert!(result.contains(">Serialized Child Value<"));
}

#[test]
fn serialize_unit_struct() {
    let test_data = XMLObjectUnit;
    let result = from_obj(&test_data);
    assert!(!result.is_empty());
    assert!(result.contains("<XMLObjectUnit/>"));
}

#[test]
fn serialize_simple_struct() {
    let test_data = XMLObjectSimple {
        field_a: "Serialized String".to_string(),
        field_b: 123,
        field_c: 2.718,
    };
    
    let result = from_obj(&test_data);
    assert!(!result.is_empty());
    assert!(result.contains("<XMLObjectSimple>"));
    assert!(result.contains("<field_a>Serialized String</field_a>"));
    assert!(result.contains("<field_b>123</field_b>"));
    assert!(result.contains("<field_c>2.718</field_c>"));
}

#[test]
fn round_trip_struct_with_attributes() -> Result<(), PError> {
    let original = XMLObject {
        attribute: "Round Trip Attr".to_string(),
        field_a: "Round Trip Field".to_string(),
        child: Child {
            attribute: "Round Trip Child Attr".to_string(),
            value: "Round Trip Child Value".to_string(),
        },
    };
    
    let xml = from_obj(&original);
    let parsed: XMLObject = from_xml(&xml)?;
    assert_eq!(original, parsed);
    Ok(())
}

#[test]
fn round_trip_unit_struct() -> Result<(), PError> {
    let original = XMLObjectUnit;
    let xml = from_obj(&original);
    let parsed: XMLObjectUnit = from_xml(&xml)?;
    assert_eq!(original, parsed);
    Ok(())
}

#[test]
fn round_trip_simple_struct() -> Result<(), PError> {
    let original = XMLObjectSimple {
        field_a: "Round Trip String".to_string(),
        field_b: 456,
        field_c: 1.618,
    };
    
    let xml = from_obj(&original);
    let parsed: XMLObjectSimple = from_xml(&xml)?;
    assert_eq!(original, parsed);
    Ok(())
} 