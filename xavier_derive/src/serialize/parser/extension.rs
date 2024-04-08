use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Field, LitStr};
use crate::common::meta::{MetaInfo, MetaName};
use crate::serialize::parser::instructions::XmlPI;

/* Any literal set that must be rendered with the object */
pub struct XmlExtension(Vec<LitStr>);

impl ToTokens for XmlExtension {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let extensions = &self.0;
        tokens.extend(quote! {
             {
                 let mut extensions = String::new();
                 #(extensions.push_str(&#extensions);)*
                 extensions
             }
        });
    }
}

impl XmlExtension {
    pub fn from_field(field: &Field) -> XmlExtension {
        //Handle special tags that can generate extension rendering ex. PI.
        XmlExtension(XmlPI::literals_from_attrs(MetaInfo::vec_attr_by_name(&field.attrs, MetaName::PI)))
    }
}