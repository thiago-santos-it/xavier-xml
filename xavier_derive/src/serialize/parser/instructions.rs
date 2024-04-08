use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Attribute, DeriveInput, LitStr};
use crate::common::meta::{MetaInfo, MetaName};

pub struct XmlPI;

impl XmlPI {
    pub fn parse(input: &DeriveInput) -> TokenStream {
        let pis_def = XmlPI::literals_from_attrs(MetaInfo::vec_attr_by_name(&input.attrs, MetaName::PI));
        quote! {
            {   let mut pi = String::new();
                #(pi.push_str(&#pis_def);)*
                pi
            }
        }
    }

    pub fn literals_from_attrs(attributes: Vec<&Attribute>) -> Vec<LitStr> {
        attributes.iter().map(| attr | {
            let pi_meta = &attr.meta.to_token_stream().to_string();
            let mut pi_meta = pi_meta.replace("pi(", "<?");
            pi_meta.pop();
            pi_meta.push_str("?>");
            LitStr::new(&pi_meta, Span::call_site())
        }).collect()
    }
}