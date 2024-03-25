use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::Data::Struct;
use syn::{ DeriveInput, Fields, FieldsNamed, LitStr };
use crate::serialize::parser::meta::{MetaInfo, MetaName};
use crate::serialize::parser::naming::{attribute_name, tag_name};

pub enum TagElement {
    Complex(Ident),
    Simple(Ident, LitStr)
}

pub struct ElementAttr {
    pub field: Ident,
    pub name: LitStr
}

pub struct XMLElements {
    pub namespace: LitStr,
    pub tags: Vec<TagElement>,
    pub attributes: Vec<ElementAttr>,
}

impl ToTokens for TagElement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tag_tokens = match self {
            TagElement::Simple(field, name) => {
                quote! {
                    format!("<{}>{}</{}> ", #name, self.#field.to_xml(false), #name)
                }
            },
            TagElement::Complex(field) =>  {
                quote! {
                    self.#field.to_xml(false)
                }
            }
        };
        tokens.extend(tag_tokens);
    }
}

impl ToTokens for ElementAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field = &self.field;
        let name = &self.name;
        let attr_tokens = quote! {
            format!(" {} = \"{}\" ", #name, self.#field)
        };
        tokens.extend(attr_tokens);
    }
}



pub fn parse(input: &DeriveInput, obj_meta_info: Option<&MetaInfo>) -> Option<XMLElements>{
    if let Struct(struct_item) = &input.data {
        if let Fields::Named(fields) = &struct_item.fields {
            return Some(XMLElements{
                namespace: parse_namespace(fields),
                tags: parse_tags(fields, obj_meta_info),
                attributes: parse_attributes(fields, obj_meta_info)
            })
        }
    }
    None
}

fn parse_fields<T, P>(fields: &FieldsNamed, parse: P) -> Vec<T>
    where P: Fn(Ident, Option<MetaInfo>) -> Option<T> {
    fields.named.iter().filter_map(|field| {
        let name = field.ident.clone()?;
        parse(name, MetaInfo::from_name(&field.attrs, MetaName::XML) )
    }).collect()
}

fn parse_namespace(fields: &FieldsNamed) -> LitStr {
    let namespace = parse_fields(fields, {
        |name, meta|
            meta.and_then(|meta| {
                if meta.contains("xmlsns") {
                    Some(name.to_string())
                } else {
                    None
                }
            })
    });
    if let Some(first_ns) = namespace.first() {
        if namespace.len() > 1 {
            println!("[WARNING] You have more than one namespace definition, we will pick the first one.");
        }
        LitStr::new(first_ns, proc_macro2::Span::call_site())
    } else {
        LitStr::new("", proc_macro2::Span::call_site())
    }
}

fn parse_attributes(fields: &FieldsNamed, obj_meta_info: Option<&MetaInfo>) -> Vec<ElementAttr> {
    parse_fields(fields, { |field, meta|
        meta.and_then(|meta| {
            if meta.contains("attribute") {
                let attr_name = attribute_name(&field, obj_meta_info, &meta);
                Some(ElementAttr { field: field, name: attr_name})
            } else {
                None
            }
        })
    })
}

fn parse_tags(fields: &FieldsNamed, obj_meta_info: Option<&MetaInfo>) -> Vec<TagElement> {
    parse_fields(fields, |field, meta| {
        if let Some(meta) = meta {
            if !meta.contains("attribute") {
                if meta.contains("complex") || meta.contains("flatten") {
                    Some(TagElement::Complex(field))
                } else {
                    let tag_name = tag_name(&field, obj_meta_info, Some(&meta));
                    Some(TagElement::Simple(field, tag_name))
                }
            } else {
                None
            }
        } else {
            let tag_name = tag_name(&field, obj_meta_info, None);
            Some(TagElement::Simple(field, tag_name))
        }
    })
}
