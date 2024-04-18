use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

pub struct ConstructorField {
    pub is_option: bool,
    pub is_box: bool,
    pub field: Ident
}

pub struct Constructor {
    pub values: Vec<ConstructorField>
}

impl ToTokens for Constructor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let values: Vec<TokenStream> = self.values.iter().map(|item| {
            let field = item.field.clone();
            if !item.is_option {
                if item.is_box {
                    quote! { #field: Box::new(#field.unwrap()) }
                } else {
                    quote! { #field: #field.unwrap() }
                }
            } else {
                if item.is_box {
                    quote! { #field: #field.map(|x| Box::new(x)) }
                } else {
                    quote! { #field }
                }
            }
        }).collect();
        tokens.extend(quote! { return Ok(Self{ #(#values,)* }); })
    }
}
