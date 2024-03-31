pub use xavier_derive::XmlSerializable;
pub use xavier_derive::XmlDeserializable;

pub use xavier_internal::serialize::macro_trait::XmlSerializable;
pub use xavier_internal::xtext;
pub use xavier_internal::namespaces;
pub use xavier_internal::xcdata;

pub use xavier_internal::serialize;
pub fn from_obj<T: XmlSerializable>(obj: &T) -> String {
    obj.to_xml(true)
}