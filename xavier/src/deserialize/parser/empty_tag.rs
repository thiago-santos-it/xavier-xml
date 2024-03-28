use quick_xml::events::BytesStart;
use crate::deserialize::parser::error::XmlError;

#[derive(Debug)]
pub struct XmlEmptyTag {
    pub name: String
}

impl XmlEmptyTag {
    pub(crate) fn parse(event: BytesStart) -> Result<XmlEmptyTag, XmlError> {
        let name=  String::from_utf8(event.name().0.to_vec())?;
        Ok(XmlEmptyTag { name })
    }
}