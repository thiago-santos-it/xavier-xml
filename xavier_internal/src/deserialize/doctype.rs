use quick_xml::events::Event;
use quick_xml::Reader;
use crate::deserialize::error::XmlError;

#[macro_export]
macro_rules! doctype {
    ($expr:expr) => { xavier::deserialize::doctype::parse($expr) };
}

pub fn parse(xml: &str) -> Result<Option<(String, String)>, XmlError> {
    let mut reader = Reader::from_str(xml);
    loop {
        match reader.read_event() {
            Err(error) => { return Err(XmlError::new(&format!("Error at position {}: {:?}", reader.buffer_position(), error))) },
            Ok(Event::Eof) => { break },
            Ok(Event::Start(_)) => { break },
            Ok(Event::End(_)) => {},
            Ok(Event::Empty(_)) => {},
            Ok(Event::Decl(_)) => {},
            Ok(Event::PI(_)) => {},
            Ok(Event::DocType(event)) => {
                let doc_type = String::from_utf8(event.to_vec())?;
                if let Some((name, file)) = doctype_obj(&doc_type) {
                    return Ok(Some((name, file)))
                } else {
                    eprintln!("Parse of inline doc type is not implemented yet!");
                    return Ok(None)
                }
            },
            Ok(Event::Text(_)) => {},
            Ok(Event::Comment(_)) => {},
            Ok(Event::CData(_)) => {},
        };
    };
    Ok(None)
}

fn doctype_obj(input: &str) -> Option<(String, String)> {
    let input = input.trim();
    if !input.starts_with("<!DOCTYPE") || !input.ends_with('>') {
        return None;
    }
    let content = &input[9..input.len() - 1];

    let parts: Vec<&str> = content.split_whitespace().collect();

    if parts.is_empty() {
        return None;
    }

    let tag_name = parts[0].to_string();

    // Find file
    let file = parts.iter().find(|&&part| part.starts_with("\""));

    match file {
        Some(file) => {
            let file = file.trim_start_matches('"').trim_end_matches('"');
            Some((tag_name, file.to_string()))
        }
        None => None,
    }
}
