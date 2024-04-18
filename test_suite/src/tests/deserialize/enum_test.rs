use std::str::FromStr;
use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable, PartialEq, Debug)]
enum CustomEnum {
    ValueA, ValueB
}

impl FromStr for CustomEnum {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "ValueA" => Ok(CustomEnum::ValueA),
            "ValueB" => Ok(CustomEnum::ValueB),
            _ => Err(()),
        }
    }
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="object")]
struct XMLObject {
    pub enum_field: CustomEnum,
}

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = r#"<object><enum_field>ValueA</enum_field></object>"#;
    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.enum_field, CustomEnum::ValueA);
    Ok(())
}
