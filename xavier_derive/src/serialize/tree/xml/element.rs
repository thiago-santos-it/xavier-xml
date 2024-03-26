use syn::Data::Struct;
use syn::{ DeriveInput, Fields, FieldsNamed };
use proc_macro2::Ident;
use crate::common::meta::{ MetaInfo, MetaName };
use crate::serialize::tree::xml::attribute::XmlElementAttr;
use crate::serialize::tree::xml::namespace::XmlNamespace;
use crate::serialize::tree::xml::tag::XmlTagElement;

pub struct XmlElementDef {
    pub namespace: Option<Ident>,
    pub tags: Vec<XmlTagElement>,
    pub attributes: Vec<XmlElementAttr>,
}

impl XmlElementDef {
    pub fn parse(input: &DeriveInput, obj_meta: Option<&MetaInfo>) -> Option<XmlElementDef> {
        if let Struct(struct_item) = &input.data {
            if let Fields::Named(fields) = &struct_item.fields {
                return Some(XmlElementDef {
                    namespace: XmlElementDef::from_fields_first(&fields, obj_meta, XmlNamespace::parse),
                    tags: XmlElementDef::from_fields(&fields, obj_meta, XmlTagElement::parse),
                    attributes: XmlElementDef::from_fields(&fields, obj_meta, XmlElementAttr::parse)
                })
            }
        }
        None
    }

    fn from_fields<T, P>(fields: &FieldsNamed, obj_meta: Option<&MetaInfo>, parse: P) -> Vec<T>
        where P: Fn(Ident, Option<&MetaInfo>, Option<&MetaInfo>) -> Option<T> {

        fields.named.iter().filter_map(|field| {
            parse(field.ident.clone()?, obj_meta, MetaInfo::from_name(&field.attrs, MetaName::XML).as_ref())
        }).collect()
    }

    fn from_fields_first<T, P>(fields: &FieldsNamed, obj_meta: Option<&MetaInfo>, parse: P) -> Option<T>
        where P: Fn(Ident, Option<&MetaInfo>, Option<&MetaInfo>) -> Option<T>, T: Clone {

        let result = Self::from_fields(fields, obj_meta, parse);
        if let Some(first) = result.first().cloned() {
            if result.len() > 1 {
                println!("[WARNING] You configured more than one helper when should exists only one. We will pick the first one!");
            }
            Some(first)
        } else {
            None
        }
    }
}
