use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;
use crate::common::meta::{MetaInfo, MetaName};
use crate::common::naming::names::XmlNames;

pub(crate) struct XmlSimpleTag;

impl XmlSimpleTag {
    pub fn parse(input: &DeriveInput) -> TokenStream {
        let meta_info = MetaInfo::from_name(&input.attrs, MetaName::XML);
        let tag = XmlNames::root(&input, meta_info.as_ref());
        quote! {
            let xml = format!("<{}>{}</{}>", #tag, &self.0.to_xml(false), #tag).to_string();
        }
    }
}