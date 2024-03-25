extern crate proc_macro;

use syn::{DeriveInput, Error};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use crate::serialize::parser::typing::Container;
use crate::serialize::parser::typing::Container::{Complex, Tag, Enumeration, EmptyTag};
use crate::serialize::streams::{enumeration, complex, tag, empty};

pub fn impl_xml_serializable(input: TokenStream) -> TokenStream {
    let mut input  = parse_macro_input!(input as DeriveInput);
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();
    let object_name = &input.ident;

    if let Some(container_type) = Container::type_of(&input) {
        let xml_code = match container_type {
            Complex => { complex::stream(&input) },
            Tag => { tag::stream(&input) },
            EmptyTag => { empty::stream(&input) },
            Enumeration => { enumeration::stream(&input) }
        };
        let expanded = quote! {
            impl #impl_generics xavier_xml::serialize::parser::XMLSerializable for #object_name #ty_generics #where_clause {
                fn to_xml(&self, root: bool) -> String {
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
