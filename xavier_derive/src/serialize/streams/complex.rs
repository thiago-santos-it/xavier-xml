use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;
use crate::serialize::parser::meta::AttributeMap;

use crate::serialize::parser::naming::{fields_xml, tag_name};

pub fn stream(input: &DeriveInput, attribute_map: &AttributeMap) -> TokenStream {
    let tag = tag_name(&input, attribute_map);
    let fields_xml = fields_xml(&input);
    return  quote! {
        format!("<{}>{}<{}>", #tag, #fields_xml, #tag).to_string()
    }
}
