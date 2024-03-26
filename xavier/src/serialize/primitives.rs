use crate::serialize::macro_trait::XmlSerializable;

impl XmlSerializable for i32 {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XmlSerializable for i64 {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XmlSerializable for f32 {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XmlSerializable for f64 {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XmlSerializable for bool {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XmlSerializable for String {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

pub struct XMLCData(pub String);

impl XmlSerializable for XMLCData {
    fn to_xml(&self, _: bool) -> String {
        format!("<![CDATA[{}]]>", self.0)
    }
}
