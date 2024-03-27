use proc_macro::TokenStream;

mod serialize;
mod common;

use crate::serialize::proc_macro::impl_xml_serializable;

#[proc_macro_derive(XmlSerializable, attributes(xml, header, dtd, pi))]
pub fn xml_serializable(input: TokenStream) -> TokenStream {
    impl_xml_serializable(input)
}

#[proc_macro_derive(XmlDeserializable, attributes(xml, header, dtd, pi))]
pub fn xml_deserializable(input: TokenStream) -> TokenStream {
    impl_xml_serializable(input)
}

