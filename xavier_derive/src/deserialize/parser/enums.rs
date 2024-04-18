use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub struct XmlEnum;

impl XmlEnum {
    pub fn parse(_: &DeriveInput) -> TokenStream {
        quote!{}
    }
}