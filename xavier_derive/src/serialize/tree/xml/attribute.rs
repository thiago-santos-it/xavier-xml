use syn::LitStr;
use proc_macro2::{ Ident, TokenStream };
use quote::{ quote, ToTokens };
use crate::common::meta::MetaInfo;
use crate::common::naming::names::XmlNames;

pub struct XmlElementAttr {
    pub field: Ident,
    pub name: LitStr
}

impl ToTokens for XmlElementAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field = &self.field;
        let name = &self.name;
        let attr_tokens = quote! {
            format!(" {}=\"{}\"", #name, self.#field)
        };
        tokens.extend(attr_tokens);
    }
}

impl XmlElementAttr {
    pub fn parse(field: Ident, obj_meta: Option<&MetaInfo>, meta: Option<&MetaInfo>) -> Option<XmlElementAttr> {
        meta.and_then(|meta| {
            if meta.contains("attribute") {
                let name = XmlNames::attribute(&field, obj_meta, &meta);
                Some(XmlElementAttr { field, name })
            } else {
                None
            }
        })
    }
}