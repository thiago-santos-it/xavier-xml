use crate::deserialize::model::attribute::XmlAttribute;

pub struct XmlTag {
    pub name: String,
    pub attributes: Vec<XmlAttribute>,
    pub value: String,
    pub children: Vec<XmlTag>
}