use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, LitStr, Meta};
use crate::common::meta::{MetaInfo, MetaName};

pub struct XmlPI;

impl XmlPI {
    pub fn parse(input: &DeriveInput, tag: &LitStr) -> TokenStream {
        let dtd_def = XmlPI::pi_defs(input, tag);
        quote! {
            //let pi = if root { #(pis_def.to_string())* } else { "".to_string() };
        }
    }

    fn pi_defs(input: &DeriveInput, tag: &LitStr) -> Vec<LitStr> {
        // let dtd = MetaInfo::attr_by_name(&input.attrs, MetaName::DTD);
        //
        // if let Some(dtd) = dtd {
        //     if let Meta::NameValue(dtd) = &dtd.meta {
        //         if let syn::Expr::Lit(lit) = &dtd.value {
        //             if let syn::Lit::Str(dtd_str) = &lit.lit {
        //                 return LitStr::new(&format!("<!DOCTYPE {} SYSTEM \"{}\">",
        //                                             tag.value(), dtd_str.value()), Span::call_site());
        //             }
        //         }
        //     }
        // }
        // LitStr::new(&"", Span::call_site())
        vec![]
    }
}