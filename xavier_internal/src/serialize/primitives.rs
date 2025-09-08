use crate::serialize::macro_trait::XmlSerializable;

fn tag_to_string<T>(obj: &T, tag_name: Option<&str>) -> String where T: ToString{
    if let Some(tag_name) = tag_name {
        format!("<{}>{}</{}>", tag_name, obj.to_string(), tag_name)
    } else {
        obj.to_string()
    }
}

impl XmlSerializable for i8 {
    fn to_xml(&self,  tag_name: Option<&str>, _: bool) -> String {
        tag_to_string(self, tag_name)
    }
}

impl XmlSerializable for i16 {
    fn to_xml(&self, tag_name: Option<&str>, _: bool) -> String {
        tag_to_string(self, tag_name)
    }
}

impl XmlSerializable for i32 {
    fn to_xml(&self, tag_name: Option<&str>, _: bool) -> String {
        tag_to_string(self, tag_name)
    }
}

impl XmlSerializable for i64 {
    fn to_xml(&self, tag_name: Option<&str>, _: bool) -> String {
        tag_to_string(self, tag_name)
    }
}

impl XmlSerializable for i128 {
    fn to_xml(&self, tag_name: Option<&str>, _: bool) -> String {
        tag_to_string(self, tag_name)
    }
}

impl XmlSerializable for u8 {
    fn to_xml(&self, tag_name: Option<&str>, _: bool) -> String {
        tag_to_string(self, tag_name)
    }
}

impl XmlSerializable for u16 {
    fn to_xml(&self, tag_name: Option<&str>, _: bool) -> String {
        tag_to_string(self, tag_name)
    }
}

impl XmlSerializable for u32 {
    fn to_xml(&self, tag_name: Option<&str>, _: bool) -> String {
        tag_to_string(self, tag_name)
    }
}

impl XmlSerializable for u64 {
    fn to_xml(&self, tag_name: Option<&str>, _: bool) -> String {
        tag_to_string(self, tag_name)
    }
}

impl XmlSerializable for u128 {
    fn to_xml(&self, tag_name: Option<&str>, _: bool) -> String {
        tag_to_string(self, tag_name)
    }
}
impl XmlSerializable for f32 {
    fn to_xml(&self, tag_name: Option<&str>, _: bool) -> String {
        tag_to_string(self, tag_name)
    }
}

impl XmlSerializable for f64 {
    fn to_xml(&self, tag_name: Option<&str>, _: bool) -> String {
        tag_to_string(self, tag_name)
    }
}

impl XmlSerializable for bool {
    fn to_xml(&self, tag_name: Option<&str>, _: bool) -> String {
        tag_to_string(self, tag_name)
    }
}

impl XmlSerializable for String {
    fn to_xml(&self, tag_name: Option<&str>, _: bool) -> String {
        tag_to_string(self, tag_name)
    }
}

impl XmlSerializable for isize {
    fn to_xml(&self, tag_name: Option<&str>, _: bool) -> String {
        tag_to_string(self, tag_name)
    }
}

impl XmlSerializable for usize {
    fn to_xml(&self, tag_name: Option<&str>, _: bool) -> String {
        tag_to_string(self, tag_name)
    }
}

impl XmlSerializable for char {
    fn to_xml(&self, tag_name: Option<&str>, _: bool) -> String {
        if let Some(tag_name) = tag_name {
            format!("<{}>{}</{}>", tag_name, format!("{}", self), tag_name)
        } else {
            format!("{}", self)
        }
    }
}


