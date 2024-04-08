use std::str::FromStr;
use crate::deserialize::error::PError;
use crate::deserialize::macro_trait::{Context, XmlDeserializable};

impl <T: FromStr> XmlDeserializable for T where PError: From<<T as FromStr>::Err> {
    fn from_xml(_: bool, context: Context) -> Result<Self, PError> {
        match context {
            Context::BytesText(bytes) => { Ok(String::from_utf8(bytes.to_vec())?.parse()?) },
            Context::BytesCData(bytes) => { Ok(String::from_utf8(bytes.to_vec())?.parse()?) },
            _ => { return Err(PError::new("Error trying to parse primitive, invalid context")) }
        }
    }
}
