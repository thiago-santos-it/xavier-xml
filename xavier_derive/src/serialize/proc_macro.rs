extern crate proc_macro;

use syn::{DeriveInput, Error};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use crate::serialize::parser::meta::attribute_map;

use crate::serialize::parser::typing::ContainerType::{Complex, Tag, Root, Enumeration, EmptyTag};
use crate::serialize::parser::typing::container_type;
use crate::serialize::streams::{enumeration, complex, tag, root, empty};

pub fn impl_xml_serializable(input: TokenStream) -> TokenStream {
    let mut input  = parse_macro_input!(input as DeriveInput);
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();
    let object_name = &input.ident;
    let attribute_map = attribute_map(&input);
    let container_type = container_type(&input, &attribute_map);

    if let Some(container_type) = container_type {
        let xml_code = match container_type {
            Complex => { complex::stream(&input, &attribute_map) },
            Tag => { tag::stream(&input, &attribute_map) },
            EmptyTag => { empty::stream(&input, &attribute_map) },
            Root => { root::stream(&input, &attribute_map) },
            Enumeration => { enumeration::stream(&input, &attribute_map) }
        };
        let expanded = quote! {
            impl #impl_generics xavier_xml::serialize::parser::XMLSerializable for #object_name #ty_generics #where_clause {
                fn to_xml(&self) -> String {
                    #xml_code
                }
            }
        };
        TokenStream::from(expanded)
    } else {
        let message = "Proc macro 'xml serialize' does not support this type of object config.";
        return Error::new_spanned(object_name, message).to_compile_error().into();
    }
}
