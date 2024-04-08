use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::Reader;
use crate::deserialize::error::PError;

#[macro_export]
macro_rules! instructions {
    ($expr:expr, $tag:expr) => { xavier::deserialize::instructions::parse($expr, $tag) };
}

pub fn parse(xml: &str, function: fn(tag: String, instruction: String, params: String) -> ()) -> Result<(), PError> {
    let mut reader = Reader::from_str(xml);
    let mut stack: Vec<String> = vec![];
    loop {
        match reader.read_event() {
            Err(error) => { return Err(PError::new(&format!("Error at position {}: {:?}", reader.buffer_position(), error))) },
            Ok(Event::Eof) => { break },
            Ok(Event::Start(event)) => {
                for context in &stack {
                    if let Some(context) = pi_obj(&context) {
                        function(String::from_utf8(event.name().0.to_vec())?, context.0.to_string(), context.1.to_string());
                    }
                }
                stack.clear()
            },
            Ok(Event::End(_)) => {},
            Ok(Event::Empty(_)) => {},
            Ok(Event::Decl(_)) => {},
            Ok(Event::PI(event)) => {
                stack.push(String::from_utf8(event.to_vec())?);
            }
            Ok(Event::DocType(_)) => {},
            Ok(Event::Text(_)) => {},
            Ok(Event::Comment(_)) => {},
            Ok(Event::CData(_)) => {},
        };
    };
    Ok(())
}

fn pi_obj(input: &str) -> Option<(String, String)> {

    let start_index = match input.find(' ') {
        Some(idx) => idx,
        None => {
            return None;
        }
    };
    let name = &input[0..start_index];
    let params = &input[start_index + 1..input.len()];
    Some((name.to_string(), params.to_string()))
}



