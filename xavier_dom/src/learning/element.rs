use crate::deserialize::parser::empty_tag::XmlEmptyTag;
use crate::deserialize::parser::tag::XmlTag;

#[derive(Debug)]
pub enum XmlElement {
    Tag(XmlTag),
    Empty(XmlEmptyTag)
}




