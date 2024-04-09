use quote::quote;
use syn::DeriveInput;
use crate::deserialize::parser::complex::XmlComplexTag;

pub struct XmlDeStream;

pub enum DeStreamType {
    Complex, Simple, Empty, Enum
}

impl XmlDeStream {
    pub(crate) fn stream(input: &DeriveInput, typed: DeStreamType) -> proc_macro2::TokenStream {
        let mut xml_stream = match typed {
            DeStreamType::Complex => XmlComplexTag::parse(input),
            DeStreamType::Simple => quote! {},
            DeStreamType::Empty => quote! {},
            DeStreamType::Enum => quote! {}
        };
        xml_stream
    }
}
