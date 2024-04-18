use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub struct XmlEmptyTag;

impl XmlEmptyTag {
    pub fn parse(_: &DeriveInput) -> TokenStream {
        quote! { Ok(Self {}) }
    }
}