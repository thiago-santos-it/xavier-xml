use crate::serialize::macro_trait::XmlSerializable;

impl <T: XmlSerializable> XmlSerializable for Option<T> {
    fn to_xml(&self, tag_name: Option<&str>, _: bool) -> String {
        if let Some(value) = &self {
            value.to_xml(tag_name, false)
        } else {
            "".to_string()
        }
    }
}
