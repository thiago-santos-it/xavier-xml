use std::{error, fmt};
use std::string::FromUtf8Error;
use quick_xml::events::attributes::AttrError;

#[derive(Debug)]
pub struct XmlError {
    message: String,
}

impl XmlError {
    pub(crate) fn new(message: &str) -> Self {
        XmlError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for XmlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for XmlError {}

impl From<FromUtf8Error> for XmlError {
    fn from(value: FromUtf8Error) -> Self {
        XmlError { message: value.to_string() }
    }
}

impl From<quick_xml::Error> for XmlError {
    fn from(value: quick_xml::Error) -> Self {
        XmlError { message: value.to_string() }
    }
}

impl From<AttrError> for XmlError {
    fn from(value: AttrError) -> Self {
        XmlError { message: value.to_string() }
    }
}
