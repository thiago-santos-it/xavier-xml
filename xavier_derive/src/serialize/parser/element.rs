use syn::Data::Struct;
use syn::{DeriveInput, Fields, FieldsNamed, Type};
use proc_macro2::Ident;
use crate::common::meta::{ MetaInfo, MetaName };
use crate::serialize::parser::attribute::XmlElementAttr;
use crate::serialize::parser::namespace::XmlNamespace;
use crate::serialize::parser::tag::XmlTagElement;
use crate::serialize::parser::extension::XmlExtension;

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
        where P: Fn(Ident, Type, Option<&MetaInfo>, Option<&MetaInfo>, XmlExtension) -> Option<T> {

        fields.named.iter().filter_map(|field| {
            parse(field.ident.clone()?, field.ty.clone(), obj_meta, MetaInfo::from_name(&field.attrs, MetaName::XML).as_ref(), XmlExtension::from_field(field))
        }).collect()
    }

    fn from_fields_first<T, P>(fields: &FieldsNamed, obj_meta: Option<&MetaInfo>, parse: P) -> Option<T>
        where P: Fn(Ident, Type, Option<&MetaInfo>, Option<&MetaInfo>, XmlExtension) -> Option<T>, T: Clone {

        let result = Self::from_fields(fields, obj_meta, parse);
        if let Some(first) = result.first().cloned() {
            if result.len() > 1 {

            }
            Some(first)
        } else {
            None
        }
    }
}
