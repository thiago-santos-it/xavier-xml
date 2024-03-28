use quick_xml::events::{BytesCData, BytesText};

#[derive(Debug)]
pub struct XmlValue {
    pub raw: String
}

impl XmlValue {
    pub fn append_cdata(&mut self, bytes: BytesCData) {

    }

    pub fn append_text(&mut self, bytes: BytesText) {

    }
}

