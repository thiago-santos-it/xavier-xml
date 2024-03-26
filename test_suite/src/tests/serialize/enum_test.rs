use std::fmt::{Display, Formatter};
use xavier::{from_obj, XmlSerializable};

#[derive(XmlSerializable)]
enum CustomEnum {
    ValueA
}

// Many libs don't implement of infer any string value in this case, we are no exception.
impl Display for CustomEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CustomEnum::ValueA => { "Value A".to_string() },
        };
        write!(f, "{}", str)
    }
}

#[derive(XmlSerializable)]
#[xml(name="object")]
struct XMLObject {
    pub enum_field: CustomEnum,
}

#[test]
fn serialize() {
    let should = r#"<object><enum_field>Value A</enum_field></object>"#;
    let xml = XMLObject { enum_field: CustomEnum::ValueA  };
    assert_eq!(from_obj(&xml), should);
}
