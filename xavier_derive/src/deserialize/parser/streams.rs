use quote::quote;
use syn::DeriveInput;

pub struct XmlDeStream;

pub enum DeStreamType {
    Complex, Simple, Empty, Enum
}

impl XmlDeStream {
    pub(crate) fn stream(input: &DeriveInput, typed: DeStreamType) -> proc_macro2::TokenStream {
        let mut xml_stream = match typed {
            DeStreamType::Complex => quote! {},
            DeStreamType::Simple => quote! {},
            DeStreamType::Empty => quote! {},
            DeStreamType::Enum => quote! {}
        };
        xml_stream
    }
}
