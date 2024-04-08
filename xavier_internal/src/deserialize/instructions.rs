use quick_xml::events::{BytesText, Event};
use quick_xml::Reader;
use crate::deserialize::error::PError;

#[macro_export]
macro_rules! instructions {
    ($expr:expr, $tag:expr) => { xavier::deserialize::instructions::parse($expr, $tag) };
}

pub fn parse(xml: &str, tag: Option<String>) -> Result<Vec<(String, String)>, PError> {
    let mut reader = Reader::from_str(xml);
    let mut result = vec![];
    let mut current_tag = None;
    loop {
        match reader.read_event() {
            Err(error) => { return Err(PError::new(&format!("Error at position {}: {:?}", reader.buffer_position(), error))) },
            Ok(Event::Eof) => { break },
            Ok(Event::Start(event)) => {
                current_tag = Some(String::from_utf8(event.name().0.to_vec())?);
                continue
            },
            Ok(Event::End(_)) => {},
            Ok(Event::Empty(_)) => {},
            Ok(Event::Decl(_)) => {},
            Ok(Event::PI(event)) => {
                if current_tag != tag { continue }
                let pi_xml = String::from_utf8(event.to_vec())?;
                if let Some((name, params)) = pi_obj(&pi_xml) {
                    result.push((name, params));
                } else {
                    eprintln!(r#"Invalid PI check if it's composed by key = "value" or flags"#);
                     return Err(PError::new("Unsupported content"))
                }},
            Ok(Event::DocType(_)) => {},
            Ok(Event::Text(_)) => {},
            Ok(Event::Comment(_)) => {},
            Ok(Event::CData(_)) => {},
        };
    };
    Ok(result)
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



