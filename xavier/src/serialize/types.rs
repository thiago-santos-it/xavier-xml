use crate::serialize::parser::XMLSerializable;

impl XMLSerializable for i32 {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XMLSerializable for i64 {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XMLSerializable for f32 {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XMLSerializable for f64 {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XMLSerializable for bool {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XMLSerializable for String {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

pub struct XMLCData(pub String);

impl XMLSerializable for XMLCData {
    fn to_xml(&self, _: bool) -> String {
        format!("<![CDATA[{}]]>", self.0)
    }
}

#[macro_export]
macro_rules! xcdata {
    ($expr:expr) => { XMLCData($expr.to_string()) };
}

#[macro_export]
macro_rules! xtext {
    ($expr:expr) => { $expr.to_string() };
}