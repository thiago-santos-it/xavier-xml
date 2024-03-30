use quick_xml::events::BytesDecl;
use crate::deserialize::parser::error::XmlError;

#[derive(Debug)]
pub struct Declaration {
    pub version: String,
    pub encoding: Option<String>,
    pub standalone: Option<String>
}

impl Declaration {
    pub fn parse(event: BytesDecl) -> Result<Declaration, XmlError> {

        let version = String::from_utf8(event.version()?.to_vec())?;

        let encoding = if let Some(encoding) = event.encoding() {
            Some(String::from_utf8(encoding?.to_vec())?)
        } else {
            None
        };

        let standalone = if let Some(standalone) = event.standalone() {
            Some(String::from_utf8(standalone?.to_vec())?)
        } else {
            None
        };

        Ok(Declaration { version, encoding, standalone })
    }
}