extern crate proc_macro;

use syn::{DataEnum, DataUnion, DeriveInput, Error, Fields, FieldsNamed, FieldsUnnamed};
use proc_macro::TokenStream;
use quote::quote;
use syn::Data::{Enum, Struct, Union};
use syn::parse_macro_input;
use crate::serialize::tree::complex;
use crate::serialize::tag::empty;
use crate::serialize::tag::simple;
use crate::serialize::value::enumeration;

pub enum InputType {
    StructNamed,
    StructUnnamed,
    StructUnit,
    Enumeration
}

impl InputType {
    fn type_of(input: &DeriveInput) -> Option<InputType> {
        return match &input.data {
            Struct(obj) => match &obj.fields {
                Fields::Named(FieldsNamed { .. }) => { Some(InputType::StructNamed) },
                Fields::Unnamed(FieldsUnnamed { .. }) => { Some(InputType::StructUnnamed) }
                Fields::Unit => { Some(InputType::StructUnit) }
            },
            Enum(DataEnum { .. }) => { Some(InputType::Enumeration) },
            Union(DataUnion { .. }) => { None }
        }
    }
}

pub fn impl_xml_serializable(input: TokenStream) -> TokenStream {
    let input  = parse_macro_input!(input as DeriveInput);
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();
    let object_name = &input.ident;

    if let Some(container_type) = InputType::type_of(&input) {
        let xml_code = match container_type {
            InputType::StructNamed => { complex::stream(&input) },
            InputType::StructUnnamed => { simple::stream(&input) },
            InputType::StructUnit => { empty::stream(&input) },
            InputType::Enumeration => { enumeration::stream(&input) }
        };
        let expanded = quote! {
            impl #impl_generics xavier::serialize::macro_trait::XmlSerializable for #object_name #ty_generics #where_clause {
                fn to_xml(&self, root: bool) -> String {
                    #xml_code
                }
            }
        };
        return TokenStream::from(expanded)
    }
    let message = "Proc macro 'xml serialize' does not support this type of object config.";
    return Error::new_spanned(object_name, message).to_compile_error().into();
}
