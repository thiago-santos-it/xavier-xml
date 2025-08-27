
pub trait XmlSerializable {
    fn to_xml(&self, tag_name: Option<&str>, headless: bool, root: bool) -> String;
}

