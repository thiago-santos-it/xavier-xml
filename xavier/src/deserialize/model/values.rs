pub enum XmlTypedValue {
    String(XmlStringValue),
    Int(XmlIntValue),
    Bool(XmlBoolValue),
    Float(XmlFloatValue)
}

pub struct XmlStringValue(String);
pub struct XmlIntValue(String);
pub struct XmlFloatValue(String);
pub struct XmlBoolValue(String);

pub struct XmlValue {
    pub is_cdata: bool,
    pub comment: String,
    pub value: XmlTypedValue
}