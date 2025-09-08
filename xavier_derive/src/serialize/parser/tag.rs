use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{LitStr, Type};
use crate::common::meta::MetaInfo;
use crate::common::naming::names::XmlNames;
use crate::serialize::parser::extension::XmlExtension;
use crate::serialize::parser::types::is_outer_option;

pub enum XmlTagElement {
    Complex(Ident, XmlExtension),
    Simple(Ident, Type, LitStr, XmlExtension),
    Value(Ident, XmlExtension),
    Collection(Ident, Type, LitStr, LitStr, XmlExtension),
}

impl ToTokens for XmlTagElement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tag_tokens = match self {
            XmlTagElement::Simple(field, ty, name, extensions) => {
                if is_outer_option(&ty) {
                    quote! {
                        if self.#field.is_none()  {
                            "".to_string()
                        } else {
                            format!("{}<{}>{}</{}>", #extensions, #name, self.#field.to_xml(xa_headless, false), #name)
                        }
                    }
                } else {
                    quote! { format!("{}<{}>{}</{}>", #extensions, #name, self.#field.to_xml(xa_headless, false), #name) }
                }
            },
            XmlTagElement::Complex(field, extensions) =>  {
                quote! {
                    format!("{}{}", #extensions, self.#field.to_xml(xa_headless, false))
                }
            },
            XmlTagElement::Value(field, extensions) =>  {
                quote! {
                    format!("{}{}", #extensions, self.#field.to_xml(xa_headless, false))
                }
            },
            XmlTagElement::Collection(field, ty, tag_name, inner_name, extensions) => {
                let items_constructor = if is_outer_option(&ty) {
                    quote!{
                        let render = self.#field.is_some();
                        let xa_items = if let Some(xa_value_item) = &self.#field {
                            xa_value_item
                        } else {
                            &Vec::new()
                        };
                    }
                } else {
                    quote!{
                        let render = true;
                        let xa_items = &self.#field;
                    }
                };
                quote! {
                    {
                        #items_constructor
                        let mut collection_xml = String::new();
                        if render {
                            collection_xml.push_str(&format!("<{}>", #tag_name));
                            for item in xa_items {
                                collection_xml.push_str(&format!("<{}>{}</{}>", #inner_name, item.to_xml(true, false), #inner_name));
                            }
                            collection_xml.push_str(&format!("</{}>", #tag_name));
                            format!("{}{}", #extensions, collection_xml)
                        } else {
                            "".to_string()
                        }
                    }
                }
            }
        };
        tokens.extend(tag_tokens);
    }
}

impl XmlTagElement {
    pub fn parse(field: Ident, ty: Type, obj_meta: Option<&MetaInfo>, meta: Option<&MetaInfo>, extension: XmlExtension) -> Option<XmlTagElement> {

        if let Some(meta) = meta {
            if !meta.contains("attribute") && !meta.contains("xmlns") {

                return if meta.contains("skip") {
                    None
                } else if meta.contains("tree") {
                    Some(XmlTagElement::Complex(field, extension))
                } else if meta.contains("flatten") || meta.contains("value") {
                    Some(XmlTagElement::Value(field, extension))
                } else if meta.contains("inner") { //TODO Work on these question marks
                    let tag_name = XmlNames::tag(&field, obj_meta, Some(&meta))?;
                    let inner_name = XmlNames::inner(obj_meta, Some(&meta))?;
                    Some(XmlTagElement::Collection(field, ty, tag_name, inner_name, extension))
                } else {
                    let tag_name = XmlNames::tag(&field, obj_meta, Some(&meta))?;
                    Some(XmlTagElement::Simple(field, ty, tag_name, extension))
                }
            }
        } else {
            let tag_name = XmlNames::tag(&field, obj_meta, None)?;
            return Some(XmlTagElement::Simple(field, ty, tag_name, extension))
        }
        None
    }
}