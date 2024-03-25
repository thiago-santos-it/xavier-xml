use syn::Data::{Enum, Struct, Union};
use syn::{DataEnum, DataUnion, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};

//Type of XML container being created
pub enum Container {
    Complex,
    Enumeration,
    Tag,
    EmptyTag
}

impl Container {
    pub fn type_of(input: &DeriveInput) -> Option<Container> {
        return match &input.data {
            Struct(obj) => match &obj.fields {
                Fields::Named(FieldsNamed { .. }) => { Some(Container::Complex) },
                Fields::Unnamed(FieldsUnnamed { .. }) => { Some(Container::Tag) }
                Fields::Unit => { Some(Container::Tag) }
            },
            Enum(DataEnum { .. }) => { Some(Container::Enumeration) },
            Union(DataUnion { .. }) => { None }
        }
    }
}


