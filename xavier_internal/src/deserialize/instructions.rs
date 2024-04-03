use quick_xml::events::BytesText;
use crate::deserialize::error::XmlError;

#[macro_export]
macro_rules! instructions {
    ($expr:expr) => { xavier::deserialize::instructions::parse($expr) };
}

pub fn parse(event: BytesText) -> Result<(String, Vec<(String, Option<String>)>), XmlError>{
    let doc_type = String::from_utf8(event.to_vec())?;
    if let Some(result) = pi_obj(&doc_type) {
        Ok(result)
    } else {
        Err(XmlError::new(r#"Invalid PI check if it's composed by key = "value" or flags"#))
    }
}

fn pi_obj(input: &str) -> Option<(String, Vec<(String, Option<String>)>)> {
    let input = input.trim();
    if !input.starts_with("<?") || !input.ends_with("?>") {
        return None;
    }
    let content = &input[2..input.len() - 2];
    let parts: Vec<&str> = content.split_whitespace().collect();

    if parts.is_empty() {
        return None;
    }

    let name = parts[0].to_string();

    let mut params = Vec::new();
    for part in parts.iter().skip(1) {
        if let Some((key, value)) = key_value(part) {
            params.push((key, value));
        } else {
            return None;
        }
    }

    Some((name, params))
}

fn key_value(param: &str) -> Option<(String, Option<String>)> {
    let mut parts = param.splitn(2, '=');

    if let Some(key) = parts.next() {
        let value = if let Some(value) = parts.next() {
            let value = value.trim_matches('"');
            Some(value.to_string())
        } else {
            None
        };
        Some((key.to_string(), value))
    } else {
        None
    }
}



