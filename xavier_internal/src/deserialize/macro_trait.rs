use quick_xml::events::{ BytesText, BytesCData, BytesStart, BytesDecl };
use quick_xml::events::attributes::Attribute;
use crate::deserialize::error::XmlError;

pub enum Context<'a> {
    BytesText(BytesText<'a>),
    BytesCData(BytesCData<'a>),
    BytesStart(BytesStart<'a>),
    Attribute(Attribute<'a>),
    BytesDecl(BytesDecl<'a>),
    String(String)
}

pub trait XmlDeserializable {
    fn from_xml(root: bool, context: Context) -> Result<Self, XmlError> where Self: Sized;
}