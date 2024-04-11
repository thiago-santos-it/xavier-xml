use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub struct Constructor {
    pub values: Vec<TokenStream>
}

impl ToTokens for Constructor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let names = &self.values;
        tokens.extend(quote! { return Ok(Self{ #(#names,)* }); })
    }
}
