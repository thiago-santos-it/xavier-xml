use quote::quote;
use syn::DeriveInput;

pub fn stream(_: &DeriveInput) -> proc_macro2::TokenStream {
    quote! { self.to_string() }
}
