use crate::deserialize::model::doc_type::XmlDocType;
use crate::deserialize::model::element::XmlElement;
use crate::deserialize::XmlDeclaration;

pub struct XmlRoot {
    pub declaration: XmlDeclaration,
    pub doc_type: XmlDocType,
    pub elements: Vec<XmlElement>
}
