use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::LitStr;
use crate::common::meta::MetaInfo;
use crate::common::naming::names::XmlNames;
use crate::serialize::parser::extension::XmlExtension;

pub enum XmlTagElement {
    Complex(Ident, XmlExtension),
    Simple(Ident, LitStr, XmlExtension),
    Value(Ident, XmlExtension),
}

impl ToTokens for XmlTagElement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
         let tag_tokens = match self {
            XmlTagElement::Simple(field, name, extensions) => {
                quote! {
                    format!("{}<{}>{}</{}>", #extensions, #name, self.#field.to_xml(false), #name)
                }
            },
            XmlTagElement::Complex(field, extensions) =>  {
                quote! {
                    format!("{}{}", #extensions, self.#field.to_xml(false))
                }
            },
            XmlTagElement::Value(field, extensions) =>  {
                quote! {
                    format!("{}{}", #extensions, self.#field.to_xml(false))
                }
            }
        };
        tokens.extend(tag_tokens);
    }
}

impl XmlTagElement {
    pub fn parse(field: Ident, obj_meta: Option<&MetaInfo>, meta: Option<&MetaInfo>, extension: XmlExtension) -> Option<XmlTagElement> {

        if let Some(meta) = meta {
            if !meta.contains("attribute") && !meta.contains("xmlns") {
                return if meta.contains("tree") {
                    Some(XmlTagElement::Complex(field, extension))
                } else if meta.contains("flatten") || meta.contains("value") {
                    Some(XmlTagElement::Value(field, extension))
                } else {
                    let tag_name = XmlNames::tag(&field, obj_meta, Some(&meta));
                    Some(XmlTagElement::Simple(field, tag_name, extension))
                }
            }
        } else {
            let tag_name = XmlNames::tag(&field, obj_meta, None);
            return Some(XmlTagElement::Simple(field, tag_name, extension))
        }
        None
    }
}