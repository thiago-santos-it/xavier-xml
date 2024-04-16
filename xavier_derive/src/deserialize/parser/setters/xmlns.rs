use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

pub struct XmlnsSetter {
    pub field: Ident
}

impl ToTokens for XmlnsSetter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field = &self.field;
        tokens.extend(quote! {
            if attr_name.starts_with("xmlns") {
                if let Some(value) = #field {
                    #field = Some(format!("{} {}={}", value, attr_name.clone(), attr_value.clone()));
                } else {
                    #field = Some(format!("{}={}", attr_name.clone(), attr_value.clone()));
                }
            }
        })
    }
}