use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::LitStr;
use syn::Type;


pub struct FieldSetter {
    pub is_flatten: bool,
    pub name: Ident,
    pub tag_name: Option<LitStr>,
    pub inner_tag_name: Option<LitStr>,
    pub inner_type: Type,
}

impl ToTokens for FieldSetter {

    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tag_name = &self.tag_name;
        let field = &self.name;
        let ty = &self.inner_type;

        if self.is_flatten {
            tokens.extend(quote! { let xa_stop_on_tag = Some(#tag_name); });
        } else if let Some(name) = &self.inner_tag_name {
            tokens.extend(quote! { let xa_stop_on_tag: Option<&str> = Some(#name);});
        } else {
            tokens.extend(quote! { let xa_stop_on_tag: Option<&str> = None;});
        }

        tokens.extend(quote! {
            let _dbg = "Field";
            let should_parse = xa_tag_name == #tag_name;
            if should_parse {
                match #ty::from_xml(&mut reader, Some(&event), xa_stop_on_tag) {
                    Ok(t_value) => { #field = t_value; continue; },
                    Err(err) => {
                        let msg = "Error parsing XML field - ";
                        return Err(PError::new(&format!("{}{}", if format!("{}", err).starts_with(msg) { "" } else { msg }, err )))
                    },
                }
            }
        })
    }
}