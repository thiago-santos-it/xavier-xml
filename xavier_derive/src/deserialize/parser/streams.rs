use quote::quote;
use syn::DeriveInput;
use proc_macro2::TokenStream;

use crate::deserialize::parser::complex::stream::XmlComplex;

pub struct XmlDeStream;

pub enum DeStreamType {
    Complex, Simple, Empty, Enum
}

impl XmlDeStream {
    pub fn stream(input: &DeriveInput, typed: DeStreamType) -> TokenStream {
        match typed {
            DeStreamType::Complex => XmlComplex::parse(input),
            DeStreamType::Simple => quote! {},
            DeStreamType::Empty => quote! { Ok(Self {}) },
            DeStreamType::Enum => quote! {}
        }
    }
}
