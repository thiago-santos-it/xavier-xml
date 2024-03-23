use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;
use crate::serialize::parser::meta::AttributeMap;

use crate::serialize::parser::naming::tag_name;

pub fn stream(input: &DeriveInput, attribute_map: &AttributeMap) -> TokenStream {
    let tag = tag_name(&input, attribute_map);
    return  quote! {
        format!("<{}><{}>", #tag, #tag).to_string()
    }
}

