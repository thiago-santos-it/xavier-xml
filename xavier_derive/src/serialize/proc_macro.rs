extern crate proc_macro;

use syn::{DataEnum, DataUnion, DeriveInput, Error, Fields, FieldsNamed, FieldsUnnamed};
use proc_macro::TokenStream;
use quote::quote;
use syn::Data::{Enum, Struct, Union};
use syn::parse_macro_input;
use crate::serialize::parser::streams::{SerStreamType, XmlSerStream};

pub fn impl_xml_serializable(input: TokenStream) -> TokenStream {
    let input  = parse_macro_input!(input as DeriveInput);
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();
    let object_name = &input.ident;

    let xml_code = match &input.data {
        Struct(obj) => match &obj.fields {
            Fields::Named(FieldsNamed { .. }) => { XmlSerStream::stream(&input, SerStreamType::Complex) },
            Fields::Unnamed(FieldsUnnamed { .. }) => { XmlSerStream::stream(&input, SerStreamType::Simple) }
            Fields::Unit => { XmlSerStream::stream(&input, SerStreamType::Empty) }
        },
        Enum(DataEnum { .. }) => { XmlSerStream::stream(&input, SerStreamType::Enum) },
        Union(DataUnion { .. }) => {
            let message = "Proc macro 'xml serialize' does not support this type of object config.";
            return Error::new_spanned(object_name, message).to_compile_error().into();}
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
