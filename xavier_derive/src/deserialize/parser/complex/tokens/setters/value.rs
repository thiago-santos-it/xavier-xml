use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::Type;


pub struct ValueSetter {
    pub field: Ident,
    pub unwrapped_type: Type
}

impl ToTokens for ValueSetter {

    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field = &self.field;
        let ty = &self.unwrapped_type;

        tokens.extend(quote! {
            let result: #ty = String::from_utf8(event.to_vec())?.parse()?;
            #field = Some(result);
        })
    }
}