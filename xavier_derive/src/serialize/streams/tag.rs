use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;
use crate::serialize::parser::meta::{MetaInfo, MetaName};
use crate::serialize::parser::naming::object_name;

pub fn stream(input: &DeriveInput) -> TokenStream {
    let meta_info = MetaInfo::from_name(&input.attrs, MetaName::XML);
    let tag = object_name(&input, meta_info.as_ref());
    return  quote! {
        format!("<{}>{}</{}>", #tag, &self.0.to_xml(), #tag).to_string()
    }
}