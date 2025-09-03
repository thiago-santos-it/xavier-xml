
pub trait XmlSerializable {
    fn to_xml(&self, headless: bool, root: bool) -> String;
}

