use proc_macro::TokenStream;

mod serialize;

mod deserialize;

mod common;

use crate::serialize::proc_macro::impl_xml_serializable;

use crate::deserialize::proc_macro::impl_xml_deserializable;

#[proc_macro_derive(XmlSerializable, attributes(xml, declaration, dtd, pi))]
pub fn xml_serializable(input: TokenStream) -> TokenStream {
    impl_xml_serializable(input)
}

#[proc_macro_derive(XmlDeserializable, attributes(xml))]
pub fn xml_deserializable(input: TokenStream) -> TokenStream {
    impl_xml_deserializable(input)
}

