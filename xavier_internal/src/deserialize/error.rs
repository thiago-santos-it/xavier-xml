use std::{error, fmt};
use std::char::ParseCharError;
use std::convert::Infallible;
use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;
use std::string::FromUtf8Error;
use quick_xml::events::attributes::AttrError;

#[derive(Debug)]
pub struct PError {
    message: String,
}

impl PError {
    pub fn new(message: &str) -> Self {
        PError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for PError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for PError {}

impl From<FromUtf8Error> for PError {
    fn from(value: FromUtf8Error) -> Self {
        PError { message: value.to_string() }
    }
}

impl From<quick_xml::Error> for PError {
    fn from(value: quick_xml::Error) -> Self {
        PError { message: value.to_string() }
    }
}

impl From<AttrError> for PError {
    fn from(value: AttrError) -> Self {
        PError { message: value.to_string() }
    }
}

impl From<ParseIntError> for PError {
    fn from(value: ParseIntError) -> Self {
        PError { message: value.to_string() }
    }
}

impl From<ParseFloatError> for PError {
    fn from(value: ParseFloatError) -> Self {
        PError { message: value.to_string() }
    }
}

impl From<ParseBoolError> for PError {
    fn from(value: ParseBoolError) -> Self {
        PError { message: value.to_string() }
    }
}

impl From<ParseCharError> for PError {
    fn from(value: ParseCharError) -> Self {
        PError { message: value.to_string() }
    }
}

impl From<Infallible> for PError {
    fn from(value: Infallible) -> Self {
        PError { message: value.to_string() }
    }
}

impl From<&AttrError> for PError {
    fn from(attr_error: &AttrError) -> Self {
        PError { message: attr_error.to_string() }
    }
}

impl From<()> for PError {
    fn from(_error: ()) -> Self {
        PError { message: "Unit Error".to_string() }
    }
}