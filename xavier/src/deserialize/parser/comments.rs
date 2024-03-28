use quick_xml::events::BytesText;
use crate::deserialize::parser::error::XmlError;

#[derive(Debug)]
pub struct XmlComment {
    pub content: String
}

impl XmlComment {
    pub fn parse(event: BytesText) -> Result<XmlComment, XmlError> {
        Ok(XmlComment { content: String::from_utf8(event.to_vec())? })
    }
}