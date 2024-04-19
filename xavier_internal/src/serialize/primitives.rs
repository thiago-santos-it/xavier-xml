use crate::serialize::macro_trait::XmlSerializable;

impl XmlSerializable for i8 {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XmlSerializable for i16 {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XmlSerializable for i32 {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XmlSerializable for i64 {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XmlSerializable for i128 {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XmlSerializable for u8 {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XmlSerializable for u16 {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XmlSerializable for u32 {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XmlSerializable for u64 {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XmlSerializable for u128 {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}
impl XmlSerializable for f32 {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XmlSerializable for f64 {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XmlSerializable for bool {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XmlSerializable for String {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XmlSerializable for isize {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XmlSerializable for usize {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}

impl XmlSerializable for char {
    fn to_xml(&self, _: bool) -> String {
        self.to_string()
    }
}


