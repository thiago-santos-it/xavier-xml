use std::str::FromStr;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use crate::deserialize::error::PError;
use crate::deserialize::macro_trait::XmlDeserializable;

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
impl Primitive for String {}
impl Primitive for f32 {}
impl Primitive for f64  {}
impl Primitive for bool {}
impl Primitive for char {}

impl <T: FromStr + Primitive> XmlDeserializable for T
    where PError: From<<T as FromStr>::Err> {
    fn from_xml(reader: &mut Reader<&[u8]>, _: Option<&BytesStart>)  -> Result<Self, PError> {
        loop {
            match reader.read_event() {
                Err(error) =>  { return Err(PError::new(&format!("Error at position {}: {:?}", reader.buffer_position(), error))) },
                Ok(Event::Eof) => { },
                Ok(Event::Start(_)) => {},
                Ok(Event::End(_)) => {},
                Ok(Event::Empty(_)) => {},
                Ok(Event::Comment(_)) => {},
                Ok(Event::Text(event)) => { return Ok(String::from_utf8(event.to_vec())?.parse()?) },
                Ok(Event::CData(event)) => { return Ok(String::from_utf8(event.to_vec())?.parse()?) },
                Ok(Event::Decl(_)) => {},
                Ok(Event::PI(_)) => {},
                Ok(Event::DocType(_)) => {},
            }
        }
    }
}
