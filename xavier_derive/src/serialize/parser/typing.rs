use syn::Data::{Enum, Struct, Union};
use syn::{DataEnum, DataUnion, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};
use crate::serialize::parser::meta::AttributeMap;

//Type of XML container being created
pub enum ContainerType {
    Complex,
    Enumeration,
    Tag,
    EmptyTag,
    Root
}

pub fn container_type(input: &DeriveInput, attribute_map: &AttributeMap) -> Option<ContainerType> {
    if let Some(_) = attribute_map.0.get("root") {
        Some(ContainerType::Root)
    } else {
        return match &input.data {
            Struct(obj) => match &obj.fields {
                Fields::Named(FieldsNamed { .. }) => { Some(ContainerType::Complex) },
                Fields::Unnamed(FieldsUnnamed { .. }) => { Some(ContainerType::Tag) }
                Fields::Unit => { Some(ContainerType::Tag) }
            },
            Enum(DataEnum { .. }) => { Some(ContainerType::Enumeration) },
            Union(DataUnion { .. }) => { None }
        }
    }
}