use quick_xml::events::BytesText;
use crate::deserialize::parser::error::XmlError;

#[derive(Debug)]
pub struct Comment {
    pub content: String
}

impl Comment {
    pub fn parse(event: BytesText) -> Result<Comment, XmlError> {
        Ok(Comment { content: String::from_utf8(event.to_vec())? })
    }
}