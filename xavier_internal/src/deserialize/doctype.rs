use quick_xml::events::Event;
use quick_xml::Reader;
use crate::deserialize::error::PError;

#[macro_export]
macro_rules! doctype {
    ($expr:expr) => { xavier::deserialize::doctype::parse($expr) };
}

pub fn parse(xml: &str) -> Result<(String, String), PError> {
    let mut reader = Reader::from_str(xml);
    loop {
        match reader.read_event() {
            Err(error) => { return Err(PError::new(&format!("Error at position {}: {:?}", reader.buffer_position(), error))) },
            Ok(Event::Eof) => { break },
            Ok(Event::Start(_)) => { break },
            Ok(Event::End(_)) => {},
            Ok(Event::Empty(_)) => {},
            Ok(Event::Decl(_)) => {},
            Ok(Event::PI(_)) => {},
            Ok(Event::DocType(event)) => {
                let doc_type = String::from_utf8(event.to_vec())?;
                return if let Some((name, file)) = doctype_obj(&doc_type) {
                    Ok((name, file))
                } else {
                    Err(PError::new("Unsupported content"))
                }
            },
            Ok(Event::Text(_)) => {},
            Ok(Event::Comment(_)) => {},
            Ok(Event::CData(_)) => {},
        };
    };
    Err(PError::new("Doctype not found!"))
}

fn doctype_obj(input: &str) -> Option<(String, String)> {

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.is_empty() { return None; }

    let tag_name = parts[0].to_string();
    let file = parts.iter().find(|&&part| part.starts_with("\""));

    match file {
        Some(file) => {
            let file = file.trim_start_matches('"').trim_end_matches('"');
            Some((tag_name, file.to_string()))
        }
        None => None,
    }
}
