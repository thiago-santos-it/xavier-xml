use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

pub struct FieldXmlnsSetter {
    pub field: Ident
}

impl ToTokens for FieldXmlnsSetter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field = &self.field;
        tokens.extend(quote! {
            if xa_attr_name.starts_with("xmlns") {
                if let Some(value) = #field {
                    #field = Some(format!("{} {}={}", value, xa_attr_name.clone(), xa_attr_value.clone()));
                } else {
                    #field = Some(format!("{}={}", xa_attr_name.clone(), xa_attr_value.clone()));
                }
            }
        })
    }
}