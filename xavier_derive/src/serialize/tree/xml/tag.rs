use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::LitStr;
use crate::common::meta::MetaInfo;
use crate::common::naming::names::XmlNames;

pub enum XmlTagElement {
    Complex(Ident),
    Simple(Ident, LitStr),
    Value(Ident),
}

impl ToTokens for XmlTagElement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tag_tokens = match self {
            XmlTagElement::Simple(field, name) => {
                quote! {
                    format!("<{}>{}</{}>", #name, self.#field.to_xml(false), #name)
                }
            },
            XmlTagElement::Complex(field) =>  {
                quote! {
                    self.#field.to_xml(false)
                }
            },
            XmlTagElement::Value(field) =>  {
                quote! {
                    self.#field.to_xml(false)
                }
            }
        };
        tokens.extend(tag_tokens);
    }
}

impl XmlTagElement {
    pub fn parse(field: Ident, obj_meta: Option<&MetaInfo>, meta: Option<&MetaInfo>) -> Option<XmlTagElement> {
        if let Some(meta) = meta {
            if !meta.contains("attribute") && !meta.contains("xmlns") {
                return if meta.contains("tree") {
                    Some(XmlTagElement::Complex(field))
                } else if meta.contains("flatten") || meta.contains("value") {
                    Some(XmlTagElement::Value(field))
                } else {
                    let tag_name = XmlNames::tag(&field, obj_meta, Some(&meta));
                    Some(XmlTagElement::Simple(field, tag_name))
                }
            }
        } else {
            let tag_name = XmlNames::tag(&field, obj_meta, None);
            return Some(XmlTagElement::Simple(field, tag_name))
        }
        None
    }
}