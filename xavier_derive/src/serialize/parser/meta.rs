use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use proc_macro2::TokenStream;
use syn::{Attribute, Error, Ident, LitStr, Meta, Token};
use syn::parse::{Parse, ParseStream};

pub enum MetaName {
    XML,
    Header,
    DTD
}

impl Display for MetaName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            MetaName::XML => { "xml".to_string() },
            MetaName::Header => { "header".to_string() },
            MetaName::DTD => { "dtd".to_string() },
        };
        write!(f, "{}", str)
    }
}

pub struct MetaInfo(pub HashMap<String, String>);

impl Parse for MetaInfo {
    fn parse(input: ParseStream) -> Result<MetaInfo, Error> {
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
        Ok(MetaInfo(result))
    }
}

impl MetaInfo {

    pub fn get_or(&self, meta_name: &str, default: String) -> String {
        self.0.get(meta_name).unwrap_or(&default).to_string()
    }

    pub fn contains(&self, meta_name: &str) -> bool {
        self.0.contains_key(meta_name)
    }

    pub fn from_name(attrs: &Vec<Attribute>, meta_name: MetaName) -> Option<MetaInfo> {
        let object_attr = MetaInfo::attr_by_name(&attrs, meta_name);
        if let Some(object_attr) = object_attr {
            Some(MetaInfo::from_attr(object_attr))
        } else {
            None
        }
    }

    pub fn attr_by_name(attrs: &Vec<Attribute>, meta_name: MetaName) -> Option<&Attribute> {
        let name = meta_name.to_string();
        attrs.iter().find(|attr| { attr.path().is_ident(&name) })
    }

    pub fn empty() -> MetaInfo {
        MetaInfo(HashMap::new())
    }

    fn from_attr(attribute: &Attribute) -> MetaInfo {
        if let Meta::List(meta) = &attribute.meta {
            return syn::parse2(TokenStream::from(meta.clone().tokens)).unwrap();
        }
        MetaInfo(HashMap::new())
    }

}




