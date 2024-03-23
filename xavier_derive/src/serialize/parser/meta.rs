use std::collections::HashMap;
use proc_macro2::TokenStream;
use syn::{Attribute, DeriveInput, Error, Ident, LitStr, Meta, Token};
use syn::parse::{Parse, ParseStream};

pub struct AttributeMap(pub(crate) HashMap<String, String>);

impl Parse for AttributeMap {
    fn parse(input: ParseStream) -> Result<AttributeMap, Error> {
        let mut result = HashMap::new();
        while input.peek(Ident) {
            if input.peek(Ident) {
                let key: Ident = input.parse()?;
                let mut value: Option<LitStr> = None;
                if input.peek(Token![=]) {
                    let _: Token![=] = input.parse()?;
                    value = input.parse()?;
                }
                if let Some(value) = value {
                    result.insert(key.to_string(), value.value());
                } else {
                    result.insert(key.to_string(), "true".to_string());
                }
            }
            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            }
        }
        Ok(AttributeMap(result))
    }
}

impl AttributeMap {
    pub(crate) fn from_attr(attribute: &Attribute) -> AttributeMap {
        if let Meta::List(meta) = &attribute.meta {
            return syn::parse2(TokenStream::from(meta.clone().tokens)).unwrap();
        }
        AttributeMap(HashMap::new())
    }
}


pub fn attribute_map(input: &DeriveInput) -> AttributeMap {
    let object_attr = input.attrs.iter().find(|attr| { attr.path().is_ident("xml") });
    if let Some(object_attr) = object_attr {
        AttributeMap::from_attr(object_attr)
    } else {
        AttributeMap(HashMap::new())
    }
}
