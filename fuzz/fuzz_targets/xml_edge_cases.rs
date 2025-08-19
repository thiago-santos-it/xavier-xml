#![no_main]
use libfuzzer_sys::fuzz_target;
use xavier::{from_xml, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
struct EdgeCaseStruct {
    pub empty_string: String,
    pub optional_field: Option<String>,
    pub numbers: Vec<i32>,
    pub boolean: bool,
    pub float: f64,
}

fuzz_target!(|data: &[u8]| {
    if let Ok(xml_str) = std::str::from_utf8(data) {
        // Test edge cases
        let _ = from_xml::<EdgeCaseStruct>(xml_str);
        
        // Test with empty XML
        if xml_str.is_empty() {
            let _ = from_xml::<EdgeCaseStruct>(xml_str);
        }
        
        // Test with whitespace-only XML
        if xml_str.trim().is_empty() {
            let _ = from_xml::<EdgeCaseStruct>(xml_str);
        }
    }
}); 