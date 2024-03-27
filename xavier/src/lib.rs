pub mod serialize;
mod deserialize;

pub use xavier_derive::XmlSerializable;

pub use xavier_derive::XmlDeserializable;
use crate::serialize::macro_trait::XmlSerializable;

pub fn from_obj<T: XmlSerializable>(obj: &T) -> String {
    obj.to_xml(true)
}