use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::LitStr;

pub struct FieldAttributeSetter {
    pub is_string: bool,
    pub name: Ident,
    pub attr_name: LitStr,
}

impl ToTokens for FieldAttributeSetter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attr_name = &self.attr_name;
        let field = &self.name;
        if self.is_string {
            tokens.extend(quote! {
                if attr_name == #attr_name {
                     #field = Some(attr_value.clone());
                }
            })
        } else {
            tokens.extend(quote! {
                if attr_name == #attr_name {
                    #field = Some(attr_value.parse()?);
                }
            })
        }
    }
}
