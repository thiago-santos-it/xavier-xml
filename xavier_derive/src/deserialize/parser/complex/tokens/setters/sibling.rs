use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::Type;


pub struct SiblingSetter {
    pub name: Ident,
    pub inner_type: Type,
}

impl ToTokens for SiblingSetter {

    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field = &self.name;
        let ty = &self.inner_type;

        tokens.extend(quote! {
            let should_parse = if let Some(inner_name) = #ty::inner_name() {
                xa_tag_name == inner_name
            } else {
                false
            };
            if should_parse {
                let result = #ty::from_xml(&mut reader, Some(&event));
                match result {
                    Ok(t_value) => {
                        #field.get_or_insert_with(Vec::new).push(t_value);
                        continue;
                    }
                    Err(error) => return Err(error),
                }
            }
        })
    }
}