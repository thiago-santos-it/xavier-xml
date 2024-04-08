use proc_macro2::TokenStream;
use quote::quote;
use proc_macro2::Span;
use syn::{DeriveInput, LitStr, Meta};
use crate::common::meta::{MetaInfo, MetaName};

pub struct XmlDTD;

impl XmlDTD {
    pub fn parse(input: &DeriveInput, tag: &LitStr) -> TokenStream {
        let dtd_def = XmlDTD::dtd_def(input, tag);
        quote! {
            {
                if root { #dtd_def.to_string() } else { "".to_string() }
            }
        }
    }

    fn dtd_def(input: &DeriveInput, tag: &LitStr) -> LitStr {
        let dtd = MetaInfo::attr_by_name(&input.attrs, MetaName::DTD);

        if let Some(dtd) = dtd {
            if let Meta::NameValue(dtd) = &dtd.meta {
                if let syn::Expr::Lit(lit) = &dtd.value {
                    if let syn::Lit::Str(dtd_str) = &lit.lit {
                        return LitStr::new(&format!("<!DOCTYPE {} SYSTEM \"{}\">",
                                                    tag.value(), dtd_str.value()), Span::call_site());
                    }
                }
            }
        }
        LitStr::new(&"", Span::call_site())
    }
}
