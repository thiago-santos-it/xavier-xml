
pub trait XmlSerializable {
    fn to_xml(&self, tag_name: Option<&str>, root: bool) -> String;
}

