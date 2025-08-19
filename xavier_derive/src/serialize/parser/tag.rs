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
    Collection(Ident, LitStr, LitStr, XmlExtension), // field, tag_name, inner_name, extension
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
            },
            XmlTagElement::Collection(field, tag_name, inner_name, extensions) => {
                quote! {
                    {
                        let mut collection_xml = String::new();
                        collection_xml.push_str(&format!("<{}>", #tag_name));
                        for item in &self.#field {
                            collection_xml.push_str(&format!("<{}>{}</{}>", #inner_name, item.to_xml(false), #inner_name));
                        }
                        collection_xml.push_str(&format!("</{}>", #tag_name));
                        format!("{}{}", #extensions, collection_xml)
                    }
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
                } else if meta.contains("inner") {
                    // Handle collection with custom inner tag name
                    let tag_name = XmlNames::tag(&field, obj_meta, Some(&meta));
                    let inner_name = LitStr::new(&meta.get_or("inner", "item".to_string()), proc_macro2::Span::call_site());
                    Some(XmlTagElement::Collection(field, tag_name, inner_name, extension))
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