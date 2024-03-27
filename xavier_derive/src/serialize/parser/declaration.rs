use proc_macro2::TokenStream;
use quote::quote;
use proc_macro2::Span;
use syn::{DeriveInput, LitStr};
use crate::common::meta::{MetaInfo, MetaName};

pub struct XmlDeclaration;

impl XmlDeclaration {
    pub fn parse(input: &DeriveInput, _: &LitStr) -> TokenStream {
        let xml_declaration = XmlDeclaration::xml_declaration(input);
        quote! {
            let declaration = if root { #xml_declaration.to_string() } else { "".to_string() };
        }
    }

    fn xml_declaration(input: &DeriveInput) -> LitStr {
        if let Some(declaration) = MetaInfo::from_name(&input.attrs, MetaName::Header) {
            let version = declaration.get_or("version", "1.0".to_string());
            let encoding = declaration.get_or("encoding", "UTF-8".to_string());
            let standalone = declaration.get_or("standalone", "no".to_string());

            let declaration_tag = format!("<?xml version=\"{}\" encoding=\"{}\" standalone=\"{}\"?>",
                                     version, encoding, standalone);

            LitStr::new(&declaration_tag, Span::call_site())
        } else {
            LitStr::new(&"", Span::call_site())
        }
    }
}
