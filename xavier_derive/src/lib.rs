use proc_macro::TokenStream;

mod serialize;

use crate::serialize::proc_macro::impl_xml_serializable;

#[proc_macro_derive(XMLSerializable, attributes(xml))]
pub fn xml_serializable(input: TokenStream) -> TokenStream {
    impl_xml_serializable(input)
}

#[proc_macro_derive(XMLDeserializable, attributes(xml))]
pub fn xml_deserializable(input: TokenStream) -> TokenStream {
    impl_xml_serializable(input)
}

