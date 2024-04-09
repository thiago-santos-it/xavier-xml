
use proc_macro::TokenStream;
use quote::quote;
use syn::Data::{Enum, Struct, Union};
use syn::parse_macro_input;
use syn::{DataEnum, DataUnion, DeriveInput, Error, Fields, FieldsNamed, FieldsUnnamed};

use crate::deserialize::parser::streams::{DeStreamType, XmlDeStream};

pub fn impl_xml_deserializable(input: TokenStream) -> TokenStream {
    let input  = parse_macro_input!(input as DeriveInput);
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();
    let object_name = &input.ident;

    let xml_code = match &input.data {
        Struct(obj) => match &obj.fields {
            Fields::Named(FieldsNamed { .. }) => { XmlDeStream::stream(&input, DeStreamType::Complex) },
            Fields::Unnamed(FieldsUnnamed { .. }) => { XmlDeStream::stream(&input, DeStreamType::Simple) }
            Fields::Unit => { XmlDeStream::stream(&input, DeStreamType::Empty) }
        },
        Enum(DataEnum { .. }) => { XmlDeStream::stream(&input, DeStreamType::Enum) },
        Union(DataUnion { .. }) => {
            let message = "Proc macro 'xml deserialize' does not support this type of object config.";
            return Error::new_spanned(object_name, message).to_compile_error().into();}
    };

    let expanded = quote! {
        impl #impl_generics xavier::deserialize::macro_trait::XmlDeserializable for #object_name #ty_generics #where_clause {
            fn from_xml(mut reader: &mut quick_xml::Reader<&[u8]>, event: Option<&quick_xml::events::BytesStart>) -> Result<Self, xavier::PError> {
                #xml_code
            }
        }
    };
    return TokenStream::from(expanded)
}
