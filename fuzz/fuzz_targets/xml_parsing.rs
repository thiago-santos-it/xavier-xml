#![no_main]
use libfuzzer_sys::fuzz_target;
use xavier::{from_xml, from_obj, XmlDeserializable, XmlSerializable};

#[derive(XmlDeserializable, XmlSerializable, Debug)]
struct FuzzTestStruct {
    pub content: String,
    pub number: Option<u32>,
    pub items: Vec<String>,
}

#[derive(XmlDeserializable, XmlSerializable, Debug)]
struct ComplexFuzzStruct {
    pub id: u64,
    pub name: String,
    pub email: Option<String>,
    pub tags: Vec<String>,
    pub active: bool,
    pub score: f64,
    pub metadata: Option<FuzzTestStruct>,
}

#[derive(XmlDeserializable, XmlSerializable, Debug)]
struct NestedFuzzStruct {
    pub level: u32,
    pub data: String,
    pub children: Vec<NestedFuzzStruct>,
}

fuzz_target!(|data: &[u8]| {
    if let Ok(xml_str) = std::str::from_utf8(data) {
        // Test basic struct
        let _ = from_xml::<FuzzTestStruct>(xml_str);
        
        // Test complex struct
        let _ = from_xml::<ComplexFuzzStruct>(xml_str);
        
        // Test nested struct (with depth limit to avoid stack overflow)
        if xml_str.len() < 10000 {
            let _ = from_xml::<NestedFuzzStruct>(xml_str);
        }
        
        // Test with different XML structures
        let _ = from_xml::<Vec<FuzzTestStruct>>(xml_str);
        let _ = from_xml::<Option<FuzzTestStruct>>(xml_str);
    }
}); 