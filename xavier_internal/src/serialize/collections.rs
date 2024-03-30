use std::collections::HashMap;
use crate::serialize::macro_trait::XmlSerializable;

impl <T: XmlSerializable> XmlSerializable for Vec<T> {
    fn to_xml(&self, _: bool) -> String {
        self.iter().fold(String::new(), |mut acc, item| {
            acc.push_str(&item.to_xml(false));
            acc
        })
    }
}

impl <T: XmlSerializable> XmlSerializable for HashMap<String, T> {
    fn to_xml(&self, _: bool) -> String {
        self.iter().fold(String::new(), |mut acc, item| {
            acc.push_str(&format!("<{}>", &item.0));
            acc.push_str(&item.1.to_xml(false));
            acc.push_str(&format!("</{}>", &item.0));
            acc
        })
    }
}