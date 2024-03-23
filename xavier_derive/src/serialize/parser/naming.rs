use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, LitStr};

use crate::serialize::parser::meta::AttributeMap;

pub fn tag_name(input: &DeriveInput, attribute_map: &AttributeMap) -> LitStr {
    let  tag_name = Some(format!("{}:{}",
                        attribute_map.0.get("ns").unwrap_or(&"".to_string()),
                        attribute_map.0.get("name").unwrap_or(&input.ident.to_string())));
    LitStr::new(&tag_name.unwrap_or(input.ident.to_string()), proc_macro2::Span::call_site())
}


pub fn fields_xml(input: &DeriveInput) -> TokenStream {
    quote!{}
}