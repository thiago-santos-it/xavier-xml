use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{LitStr, Type};


pub struct SiblingSetter {
    pub name: Ident,
    pub tag_name: Option<LitStr>,
    pub inner_type: Type,
}

impl ToTokens for SiblingSetter {

    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tag_name = &self.tag_name;
        let field = &self.name;
        let ty = &self.inner_type;

        tokens.extend(quote! {
            let _dbg = "Sibling";
            let should_parse = xa_tag_name == #tag_name;
            if should_parse {
                match #ty::from_xml(&mut reader, Some(&event), Some(&xa_tag_name)) {
                    Ok(t_value) => {
                        #field.get_or_insert_with(Vec::new).push(t_value.unwrap());
                        continue;
                    },
                    Err(err) => {
                        let msg = "Error parsing XML object";
                        return Err(PError::new(&format!("{}{}", if format!("{}", err).starts_with(msg) { "" } else { msg }, err )))
                    },
                }
            }
        })
    }
}