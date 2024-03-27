use proc_macro2::Ident;
use crate::common::meta::MetaInfo;

pub struct  XmlNamespace;

impl XmlNamespace {
    pub fn parse(field: Ident, _: Option<&MetaInfo>, meta: Option<&MetaInfo>) -> Option<Ident> {
        meta.and_then(|meta| {
            if meta.contains("xmlns") {
                Some(field)
            } else {
                None
            }
        })
    }
}