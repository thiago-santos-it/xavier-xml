use crate::serialize::macro_trait::XmlSerializable;

impl <T: XmlSerializable> XmlSerializable for Option<T> {
    fn to_xml(&self, _: bool) -> String {
        if let Some(value) = &self {
            value.to_xml(false)
        } else {
            "".to_string()
        }
    }
}
