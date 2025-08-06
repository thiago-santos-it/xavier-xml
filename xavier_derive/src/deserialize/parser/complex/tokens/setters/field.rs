use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::LitStr;
use syn::Type;


pub struct FieldSetter {
    pub is_flatten: bool,
    pub name: Ident,
    pub tag_name: LitStr,
    pub inner_type: Type,
}

impl ToTokens for FieldSetter {

    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tag_name = &self.tag_name;
        let field = &self.name;
        let ty = &self.inner_type;
        if self.is_flatten {
            tokens.extend(quote! {
                let should_parse = if let Some(inner_name) = #ty::inner_name() {
                    xa_tag_name == inner_name && #field.is_none()
                } else {
                    false
                };
            });
        } else {
            tokens.extend(quote! {
                let should_parse = (xa_tag_name == #tag_name);
            });
        }

        tokens.extend(quote! {
            if should_parse {
                let result = #ty::from_xml(&mut reader, Some(&event));
                match result {
                    Ok(t_value) => { #field = Some(t_value); continue; }
                    Err(error) => { return Err(error); }
                }
            }
        })
    }
}