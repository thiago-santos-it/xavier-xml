use proc_macro2::Span;
use syn::{DeriveInput, LitStr};
use crate::common::meta::{MetaInfo, MetaName};

pub struct XmlEncoding;

impl XmlEncoding {
    pub fn parse(input: &DeriveInput) -> LitStr {
        let default = "UTF-8".to_string();
        let result = if let Some(declaration) = MetaInfo::from_name(&input.attrs, MetaName::Header) {
            declaration.get_or("encoding", default)
        } else {
            default
        };
        LitStr::new(&result, Span::call_site())
    }
}