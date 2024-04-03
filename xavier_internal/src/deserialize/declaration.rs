use quick_xml::events::{BytesDecl, Event};
use quick_xml::Reader;
use crate::deserialize::error::XmlError;

#[macro_export]
macro_rules! declaration {
    ($expr:expr) => { xavier::deserialize::declaration::parse($expr) };
}

pub fn parse(xml: &str) -> Result<Option<(String, Option<String>, Option<String>)>, XmlError> {
    let mut reader = Reader::from_str(xml);
    loop {
        match reader.read_event() {
            Err(error) => { return Err(XmlError::new(&format!("Error at position {}: {:?}", reader.buffer_position(), error))) },
            Ok(Event::Eof) => { break },
            Ok(Event::Start(_)) => { break },
            Ok(Event::End(_)) => {},
            Ok(Event::Empty(_)) => {},
            Ok(Event::Decl(event)) => { return Ok(Some(event_object(event)?)) },
            Ok(Event::PI(_)) => {},
            Ok(Event::DocType(_)) => {},
            Ok(Event::Text(_)) => {},
            Ok(Event::Comment(_)) => {},
            Ok(Event::CData(_)) => {},
        };
    };
    Ok(None)
}
fn event_object(event: BytesDecl) -> Result<(String, Option<String>, Option<String>), XmlError> {

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

    Ok((version, encoding, standalone))
}



