use quote::quote;
use syn::DeriveInput;

pub(crate) struct XmlEnumValue;

impl XmlEnumValue {
    pub fn parse(_: &DeriveInput) -> proc_macro2::TokenStream {
        quote! { let xml = self.to_string(); }
    }
}
