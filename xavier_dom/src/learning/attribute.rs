use quick_xml::events::attributes::Attribute;
use crate::deserialize::parser::error::XmlError;

#[derive(Debug)]
pub struct Attribute {
    pub key: String,
    pub value: String
}

impl Attribute {
    pub fn parse(attr: &Attribute) -> Result<Attribute, XmlError> {
        let key = String::from_utf8(attr.key.0.to_vec())?;
        let value = String::from_utf8(attr.value.to_vec())?;
        Ok(Attribute { key, value })
    }
}