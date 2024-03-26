use proc_macro2::TokenStream;
use quote::quote;
use proc_macro2::Span;
use syn::{DeriveInput, LitStr, Meta};
use crate::common::meta::{MetaInfo, MetaName};

pub struct XmlHeader;

impl XmlHeader {
    pub fn parse(input: &DeriveInput, tag: &LitStr) -> TokenStream {
        let xml_def = XmlHeader::xml_def(input);
        let dtd_def = XmlHeader::dtd_def(input, tag);
        quote! {
            let header = if root { #xml_def.to_string() } else { "".to_string() };
            let dtd = if root { #dtd_def.to_string() } else { "".to_string() };
        }
    }

    fn xml_def(input: &DeriveInput) -> LitStr {
        if let Some(header) = MetaInfo::from_name(&input.attrs, MetaName::Header) {
            let version = header.get_or("version", "1.0".to_string());
            let encoding = header.get_or("encoding", "UTF-8".to_string());
            let standalone = header.get_or("standalone", "no".to_string());

            let header_tag = format!("<?xml version=\"{}\" encoding=\"{}\" standalone=\"{}\"?>",
                                     version, encoding, standalone);

            LitStr::new(&header_tag, Span::call_site())
        } else {
            LitStr::new(&"", Span::call_site())
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
