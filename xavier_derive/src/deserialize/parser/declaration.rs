use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

pub struct FieldDecl {
    pub name: Ident,
    pub optional_type: TokenStream
}

impl ToTokens for FieldDecl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = &self.optional_type;
        let field = &self.name;
        tokens.extend( quote! {
           let mut #field: #ty = None;
        });
    }
}
