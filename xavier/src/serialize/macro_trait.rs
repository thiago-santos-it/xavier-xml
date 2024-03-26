
pub trait XmlSerializable {
    fn to_xml(&self, root: bool) -> String;
}

