use std::str::FromStr;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use crate::deserialize::error::PError;
use crate::deserialize::macro_trait::XmlDeserializable;
use crate::deserialize::decode::{decode_xml, strip_cdata};

trait Primitive {}
impl Primitive for i8 {}
impl Primitive for i16 {}
impl Primitive for i32 {}
impl Primitive for i64 {}
impl Primitive for i128 {}
impl Primitive for u8 {}
impl Primitive for u16 {}
impl Primitive for u32 {}
impl Primitive for u64 {}
impl Primitive for u128 {}
impl Primitive for isize {}
impl Primitive for usize {}
impl Primitive for f32 {}
impl Primitive for f64  {}
impl Primitive for bool {}

// Function to check for malicious characters
fn contains_malicious_characters(input: &str) -> bool {
    // Check for null characters and malicious control characters
    for c in input.chars() {
        match c as u32 {
            0x00..=0x08 | 0x0B | 0x0C | 0x0E..=0x1F | 0x7F => {
                return true;
            }
            _ => {}
        }
    }
    false
}

// Function to check for malicious hexadecimal entities
fn contains_malicious_entities(input: &str) -> bool {
    // Check for malicious hexadecimal entities
    let malicious_patterns = [
        "&#x00;", "&#x01;", "&#x02;", "&#x03;", "&#x04;", "&#x05;", "&#x06;", "&#x07;", "&#x08;",
        "&#x0B;", "&#x0C;", "&#x0E;", "&#x0F;", "&#x10;", "&#x11;", "&#x12;", "&#x13;", "&#x14;",
        "&#x15;", "&#x16;", "&#x17;", "&#x18;", "&#x19;", "&#x1A;", "&#x1B;", "&#x1C;", "&#x1D;",
        "&#x1E;", "&#x1F;", "&#x7F;"
    ];
    
    for pattern in &malicious_patterns {
        if input.contains(pattern) {
            return true;
        }
    }
    
    false
}

// Special implementation for String that handles XML entities
impl XmlDeserializable for String {
    fn from_xml(reader: &mut Reader<&[u8]>, _: Option<&BytesStart>) -> Result<Option<Self>, PError> {
        loop {
            match reader.read_event() {
                Err(error) => { return Err(PError::new(&format!("Error at position {}: {:?}", reader.buffer_position(), error))) },
                Ok(Event::Eof) => { },
                Ok(Event::Start(_)) => {},
                Ok(Event::End(_)) => { return Ok(Some("".to_string())); },
                Ok(Event::Empty(_)) => { return Ok(None); },
                Ok(Event::Comment(_)) => {},
                Ok(Event::Text(event)) => { 
                    let raw_string = String::from_utf8(event.to_vec())?;
                    let trimmed = strip_cdata(raw_string.trim());

                    if contains_malicious_entities(&trimmed) {
                        return Err(PError::new("Malicious XML entities detected"));
                    }
                    
                    let decoded = decode_xml(&trimmed);
                    
                    if contains_malicious_characters(&decoded) {
                        return Err(PError::new("Malicious characters detected in XML content"));
                    }
                    
                    return Ok(Some(decoded));
                },
                Ok(Event::CData(event)) => { 
                    let raw_string = String::from_utf8(event.to_vec())?;
                    let trimmed = raw_string.trim();
                    
                    if contains_malicious_entities(&trimmed) {
                        return Err(PError::new("Malicious XML entities detected"));
                    }
                    
                    let decoded = decode_xml(&trimmed);
                    
                    if contains_malicious_characters(&decoded) {
                        return Err(PError::new("Malicious characters detected in XML content"));
                    }
                    
                    return Ok(Some(decoded));
                },
                Ok(Event::Decl(_)) => {},
                Ok(Event::PI(_)) => {},
                Ok(Event::DocType(_)) => {},
            }
        }
    }
}

// Special implementation for char that handles whitespace correctly
impl XmlDeserializable for char {
    fn from_xml(reader: &mut Reader<&[u8]>, _: Option<&BytesStart>) -> Result<Option<Self>, PError> {
        loop {
            match reader.read_event() {
                Err(error) => { return Err(PError::new(&format!("Error at position {}: {:?}", reader.buffer_position(), error))) },
                Ok(Event::Eof) => { },
                Ok(Event::Start(_)) => {},
                Ok(Event::End(_)) => { return Ok(None); },
                Ok(Event::Empty(_)) => { return Ok(None); },
                Ok(Event::Comment(_)) => {},
                Ok(Event::Text(event)) => { 
                    let raw_string = String::from_utf8(event.to_vec())?;
                    let trimmed = raw_string.trim();
                    if trimmed.is_empty() {
                        return Ok(Some(' '));
                    }
                    if raw_string.chars().count()  > 1 {
                        return Err(PError::new("It's supposed to be a char and string was found!"));
                    }
                    return Ok(Some(trimmed.chars().next().ok_or_else(|| PError::new("Empty string cannot be parsed as char"))?));
                },
                Ok(Event::CData(event)) => { 
                    let raw_string = String::from_utf8(event.to_vec())?;
                    let trimmed = raw_string.trim();
                    if trimmed.is_empty() {
                        return Ok(Some(' '));
                    }
                    return Ok(Some(trimmed.chars().next().ok_or_else(|| PError::new("Empty string cannot be parsed as char"))?));
                },
                Ok(Event::Decl(_)) => {},
                Ok(Event::PI(_)) => {},
                Ok(Event::DocType(_)) => {},
            }
        }
    }
}

impl <T: FromStr + Primitive> XmlDeserializable for T
    where PError: From<<T as FromStr>::Err> {
    fn from_xml(reader: &mut Reader<&[u8]>, _: Option<&BytesStart>)  -> Result<Option<Self>, PError> {
        loop {
            match reader.read_event() {
                Err(error) =>  { return Err(PError::new(&format!("Error at position {}: {:?}", reader.buffer_position(), error))) },
                Ok(Event::Eof) => {},
                Ok(Event::Start(_)) => {},
                Ok(Event::End(_)) => { return Ok(None); },
                Ok(Event::Empty(_)) => { return Ok(None); },
                Ok(Event::Comment(_)) => {},
                Ok(Event::Text(event)) => {
                    return Ok(Some(String::from_utf8(event.to_vec())?.parse()?))
                },
                Ok(Event::CData(event)) => {
                    return Ok(Some(String::from_utf8(event.to_vec())?.parse()?))
                },
                Ok(Event::Decl(_)) => {},
                Ok(Event::PI(_)) => {},
                Ok(Event::DocType(_)) => {},
            }
        }
    }
}
