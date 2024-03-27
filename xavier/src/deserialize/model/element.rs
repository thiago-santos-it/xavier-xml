use crate::deserialize::model::empty_tag::XmlEmpty;
use crate::deserialize::model::tag::XmlTag;

pub enum XmlElement {
    Tag(XmlTag),
    Empty(XmlEmpty)
}




