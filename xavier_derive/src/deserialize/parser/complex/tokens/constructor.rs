use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

pub struct ConstructorField {
    pub path_idents: Vec<Ident>,
    pub field: Ident
}

pub struct Constructor {
    pub values: Vec<ConstructorField>
}

impl ToTokens for Constructor {

    fn to_tokens(&self, tokens: &mut TokenStream) {
        let values: Vec<TokenStream> = self.values.iter().map(|item| {

            let field = item.field.clone();

            let path_idents = item.path_idents.clone();
            let mut var_field= quote! { #field };
            let has_option = path_idents.iter().any(|ident| ident.to_string() == "Option");

            if path_idents.is_empty() || !has_option  {
                var_field= quote! { #field.ok_or_else(|| PError::new(&format!("Field value '{}' not found", stringify!(#field))))? };
            }

            for ident in path_idents.iter().rev() {

                if ident.to_string() == "Vec"  {
                    var_field = quote! { #var_field.unwrap_or_else(Vec::new) }
                }

                if ident.to_string() == "Box" {
                    var_field = quote! {  Box::new(#var_field) }
                }

                if ident.to_string() == "Rc" {
                    var_field = quote! {  Rc::new(#var_field) }
                }

                if ident.to_string() == "Arc" {
                    var_field = quote! {  Arc::new(#var_field) }
                }

                if ident.to_string() == "RefCell" {
                    var_field = quote! {  RefCell::new(#var_field) }
                }

                if ident.to_string() == "Mutex" {
                    var_field = quote! {  Mutex::new(#var_field) }
                }

                if ident.to_string() == "RwLock" {
                    var_field = quote! {  RwLock::new(#var_field) }
                }

                if ident.to_string() == "Option" {
                    var_field = quote! { xavier::deserialize::macro_trait::WrapWith::wrap(#field) };
                }
            }

            var_field = quote! { #field : #var_field };
            var_field
        }).collect();
        tokens.extend(quote! { return Ok(Self{ #(#values,)* }); })
    }
}