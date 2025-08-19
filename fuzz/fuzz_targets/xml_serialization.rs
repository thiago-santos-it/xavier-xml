#![no_main]
use libfuzzer_sys::fuzz_target;
use xavier::{from_xml, from_obj, XmlDeserializable, XmlSerializable};

#[derive(XmlDeserializable, XmlSerializable, Debug)]
struct SerializationFuzzStruct {
    pub id: u64,
    pub name: String,
    pub content: String,
    pub tags: Vec<String>,
    pub metadata: Option<String>,
}

fuzz_target!(|data: &[u8]| {
    if let Ok(xml_str) = std::str::from_utf8(data) {
        // Test serialization round-trip
        if let Ok(parsed) = from_xml::<SerializationFuzzStruct>(xml_str) {
            let serialized = from_obj(&parsed);
            // Verify that serialization doesn't panic
            assert!(!serialized.is_empty());
        }
    }
}); 