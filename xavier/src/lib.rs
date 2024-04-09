pub use xavier_derive::XmlSerializable;
pub use xavier_derive::XmlDeserializable;

pub use xavier_internal::serialize::macro_trait::XmlSerializable;
pub use xavier_internal::deserialize::macro_trait::XmlDeserializable;
pub use xavier_internal::deserialize::error::PError;
pub use xavier_internal::encode;
pub use xavier_internal::namespaces;
pub use xavier_internal::cdata;
pub use xavier_internal::declaration;
pub use xavier_internal::doctype;
pub use xavier_internal::decode;
pub use xavier_internal::instructions;

pub use xavier_internal::serialize;
pub use xavier_internal::deserialize;

pub fn from_obj<T: XmlSerializable>(obj: &T) -> String {
    obj.to_xml(true)
}