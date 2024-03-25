use std::fmt::{Display, Formatter};
use xavier_derive::XMLSerializable;
use xavier_xml::serialize::parser::XMLSerializable;
use xavier_xml::xtext;

#[derive(XMLSerializable)]
#[xml(tag="TagName", namespace="ns")]
pub struct CustomElement {
    #[xml(field="fieldA")]
    pub field_a: String,
    #[xml(field="fieldB")]
    pub field_b: String
}

#[test]
fn test_element() {
    let xml = CustomElement { field_a: xtext!("Some Text"), field_b: xtext!("Some Text") };
    println!("Element XML: {}", xml.to_xml(true));
}

#[derive(XMLSerializable)]
enum CustomEnum {
    ValueA
}

// As many libs we don't implement of infer any string value
impl Display for CustomEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CustomEnum::ValueA => { "Value A".to_string() },
        };
        write!(f, "{}", str)
    }
}

#[test]
fn test_enum() {
    let xml = CustomEnum::ValueA;
    println!("Enum XML: {}", xml.to_xml(true));
}

