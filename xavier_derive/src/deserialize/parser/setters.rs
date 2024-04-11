use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::LitStr;
use syn::Type;


pub struct FieldSetter {
    pub name: Ident,
    pub tag_name: LitStr,
    pub unwrapped_type: Type
}

impl ToTokens for FieldSetter {

    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tag_name = &self.tag_name;
        let field = &self.name;
        let ty = &self.unwrapped_type;
        tokens.extend(quote! {
            if tag_name == #tag_name {
                let result = #ty::from_xml(&mut reader, Some(&event));
                match result {
                    Ok(value) => { #field = Some(value); }
                    Err(error) => { return Err(error); }
                }
            }
        })
    }

}

pub struct AttributeSetter {
    pub is_string: bool,
    pub name: Ident,
    pub attr_name: LitStr,
}

impl ToTokens for AttributeSetter {
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
