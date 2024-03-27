use crate::serialize::macro_trait::XmlSerializable;

impl <T: XmlSerializable> XmlSerializable for Vec<T> {
    fn to_xml(&self, _: bool) -> String {
        self.iter().fold(String::new(), |mut acc, item| {
            acc.push_str(&item.to_xml(false));
            acc
        })
    }
}