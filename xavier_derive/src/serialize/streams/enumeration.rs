use quote::quote;
use syn::DeriveInput;
use crate::serialize::parser::meta::AttributeMap;

pub fn stream(_: &DeriveInput, _: &AttributeMap) -> proc_macro2::TokenStream {
    quote! { self.to_string() }
}
