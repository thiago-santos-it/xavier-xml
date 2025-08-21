
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::Data::{Enum, Struct, Union};
use syn::{LitStr, parse_macro_input};
use syn::{DataEnum, DataUnion, DeriveInput, Error, Fields, FieldsNamed, FieldsUnnamed};
use crate::common::meta::{MetaInfo, MetaName};
use crate::common::naming::names::XmlNames;

use crate::deserialize::parser::streams::{DeStreamType, XmlDeStream};

pub fn impl_xml_deserializable(input: TokenStream) -> TokenStream {

    let input  = parse_macro_input!(input as DeriveInput);
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();
    let object_name = &input.ident;

    let obj_meta_info = MetaInfo::from_name(&input.attrs, MetaName::XML);
    let xml_tag_name = LitStr::new(&XmlNames::root(&input, obj_meta_info.as_ref()), Span::call_site());

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
            fn from_xml(mut reader: &mut ::xavier::quick_xml::Reader<&[u8]>, start_event: Option<&::xavier::quick_xml::events::BytesStart>) -> Result<Option<Self>, xavier::PError> {
                #xml_code
            }
            fn inner_name() -> Option<String> {
                Some(#xml_tag_name.to_string())
            }
        }
    };

    TokenStream::from(expanded)
}
