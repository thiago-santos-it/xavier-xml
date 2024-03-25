
pub trait XMLSerializable {
    fn to_xml(&self, root: bool) -> String;
}

