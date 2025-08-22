use std::fmt::Display;
use std::str::FromStr;
use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
enum SimpleEnum {
    VariantA,
    VariantB,
    VariantC,
}

impl Display for SimpleEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimpleEnum::VariantA => write!(f, "VariantA"),
            SimpleEnum::VariantB => write!(f, "VariantB"),
            SimpleEnum::VariantC => write!(f, "VariantC"),
        }
    }
}

impl FromStr for SimpleEnum {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "VariantA" => Ok(SimpleEnum::VariantA),
            "VariantB" => Ok(SimpleEnum::VariantB),
            "VariantC" => Ok(SimpleEnum::VariantC),
            _ => Err(()),
        }
    }
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct XMLObject {
    pub enum_field: SimpleEnum,
}

#[test]
fn deserialize_enum_variant_a() -> Result<(), PError> {
    let xml = r#"
    <XMLObject>
        <enum_field>VariantA</enum_field>
    </XMLObject>"#;
    
    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.enum_field, SimpleEnum::VariantA);
    Ok(())
}

#[test]
fn deserialize_enum_variant_b() -> Result<(), PError> {
    let xml = r#"
    <XMLObject>
        <enum_field>VariantB</enum_field>
    </XMLObject>"#;
    
    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.enum_field, SimpleEnum::VariantB);
    Ok(())
}

#[test]
fn deserialize_enum_variant_c() -> Result<(), PError> {
    let xml = r#"
    <XMLObject>
        <enum_field>VariantC</enum_field>
    </XMLObject>"#;
    
    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.enum_field, SimpleEnum::VariantC);
    Ok(())
}

#[test]
fn serialize_enum_variant_a() {
    let test_data = XMLObject {
        enum_field: SimpleEnum::VariantA,
    };
    
    let result = from_obj(&test_data);
    assert!(!result.is_empty());
    assert!(result.contains("<enum_field>VariantA</enum_field>"));
}

#[test]
fn serialize_enum_variant_b() {
    let test_data = XMLObject {
        enum_field: SimpleEnum::VariantB,
    };
    
    let result = from_obj(&test_data);
    assert!(!result.is_empty());
    assert!(result.contains("<enum_field>VariantB</enum_field>"));
}

#[test]
fn serialize_enum_variant_c() {
    let test_data = XMLObject {
        enum_field: SimpleEnum::VariantC,
    };
    
    let result = from_obj(&test_data);
    assert!(!result.is_empty());
    assert!(result.contains("<enum_field>VariantC</enum_field>"));
}

#[test]
fn round_trip_enum_variant_a() -> Result<(), PError> {
    let original = SimpleEnum::VariantA;
    let xml = from_obj(&original);
    let parsed: SimpleEnum = from_xml(&format!("<xml>{}</xml>", &xml))?;
    assert_eq!(original, parsed);
    Ok(())
}

#[test]
fn round_trip_enum_variant_b() -> Result<(), PError> {
    let original = SimpleEnum::VariantB;
    let xml = from_obj(&original);
    let parsed: SimpleEnum = from_xml(&format!("<xml>{}</xml>", &xml))?;
    assert_eq!(original, parsed);
    Ok(())
}

#[test]
fn round_trip_enum_variant_c() -> Result<(), PError> {
    let original = SimpleEnum::VariantC;
    let xml = from_obj(&original);
    let parsed: SimpleEnum = from_xml(&format!("<xml>{}</xml>", &xml))?;
    assert_eq!(original, parsed);
    Ok(())
}
