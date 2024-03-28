use quick_xml::events::BytesText;
use crate::deserialize::parser::error::XmlError;

#[derive(Debug)]
pub struct XmlDocType {
    pub name: String,
    pub file: String
}

impl XmlDocType {
    pub fn parse(event: BytesText) -> Result<Option<XmlDocType>, XmlError> {
        let doc_type = String::from_utf8(event.to_vec())?;
        if let Some((name, file)) = XmlDocType::parse_doctype(&doc_type) {
            Ok(Some(XmlDocType { name, file }))
        } else {
            eprintln!("Parse of inline doc type is not implemented yet!");
            Ok(None)
        }
    }

    fn parse_doctype(input: &str) -> Option<(String, String)> {
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
}