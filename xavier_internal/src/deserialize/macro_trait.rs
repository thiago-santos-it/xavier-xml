use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::deserialize::error::PError;


pub trait XmlDeserializable {
    fn from_xml(reader: &mut Reader<&[u8]>, event: Option<&BytesStart>) -> Result<Self, PError> where Self: Sized;
}