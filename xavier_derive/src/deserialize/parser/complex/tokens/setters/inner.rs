use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{LitStr, Type};

pub struct InnerSetter {
    pub name: Ident,
    pub inner_type: Type,
    pub inner_tag_name: LitStr,
}

impl ToTokens for InnerSetter {

    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field = &self.name;
        let ty = &self.inner_type;
        let inner_tag = &self.inner_tag_name;

        tokens.extend(quote! {
            if xa_tag_name == #inner_tag {
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