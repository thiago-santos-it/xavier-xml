use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{DeriveInput, LitStr};
use crate::common::meta::{MetaInfo, MetaName};

pub struct XmlPI;

impl XmlPI {
    pub fn parse(input: &DeriveInput) -> TokenStream {
        let pis_def = XmlPI::pi_defs(input);
        quote! {
            let mut pi = String::new();
            if root {
                #(pi.push_str(&#pis_def);)*
            }
        }
    }

    fn pi_defs(input: &DeriveInput) -> Vec<LitStr> {
        let pi_attr_vec = MetaInfo::vec_attr_by_name(&input.attrs, MetaName::PI);
        pi_attr_vec.iter().map(| attr | {
            let pi_meta = &attr.meta.to_token_stream().to_string();
            let mut pi_meta = pi_meta.replace("pi(", "<?");
            pi_meta.pop();
            pi_meta.push_str("?>");
            LitStr::new(&pi_meta, Span::call_site())
        }).collect()
    }
}