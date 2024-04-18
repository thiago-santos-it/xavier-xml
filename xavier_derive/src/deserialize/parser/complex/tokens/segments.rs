use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Data::Struct;
use syn::{DeriveInput, Fields};
use crate::common::meta::{MetaInfo, MetaName};
use crate::common::naming::names::XmlNames;
use crate::deserialize::parser::complex::tokens::constructor::Constructor;
use crate::deserialize::parser::complex::tokens::declaration::FieldDecl;
use crate::deserialize::parser::complex::tokens::setters::attribute::AttributeSetter;
use crate::deserialize::parser::complex::tokens::setters::field::FieldSetter;
use crate::deserialize::parser::complex::tokens::setters::xmlns::XmlnsSetter;
use crate::deserialize::parser::complex::tokens::types::TypeParser;

pub struct TokenSegments {
    pub declarations: Vec<FieldDecl>,
    pub attribute_setter: Vec<AttributeSetter>,
    pub field_setter: Vec<FieldSetter>,
    pub xmlns_setter: Option<XmlnsSetter>,
    pub constructor: Constructor
}

impl TokenSegments {

    pub fn tokens_from(input: &DeriveInput, obj_meta_info: Option<&MetaInfo>) -> TokenSegments {

        let mut declarations: Vec<FieldDecl> = vec![];
        let mut field_setter: Vec<FieldSetter> = vec![];
        let mut attribute_setter: Vec<AttributeSetter> = vec![];
        let mut xmlns_setter: Option<XmlnsSetter> = None;
        let mut constructors: Vec<TokenStream> = vec![];
        let mut field_names: Vec<Ident> = vec![];

        if let Struct(struct_item) = &input.data {
            if let Fields::Named(fields) = &struct_item.fields {

                for field in fields.named.iter() {

                    if let Some(ident) = &field.ident {

                        let field_meta = MetaInfo::from_name(&field.attrs, MetaName::XML).unwrap_or(MetaInfo::empty());
                        let field_is_option = TypeParser::is_option_type(&field.ty);
                        let ty = field.ty.clone();

                        declarations.push(FieldDecl {
                            name: ident.clone(),
                            optional_type: if field_is_option { quote! { #ty } } else { quote! { Option<#ty> }},
                        });

                        if field_meta.contains("attribute") {
                            let field_attr_name = XmlNames::attribute(&ident, obj_meta_info, &field_meta);
                            attribute_setter.push(AttributeSetter {
                                is_string: TypeParser::is_string_type(&TypeParser::unwrapped_type(&ty)),
                                name: ident.clone(),
                                attr_name: field_attr_name
                            });
                        } else if field_meta.contains("xmlns") {
                            xmlns_setter = Some(XmlnsSetter { field: ident.clone() })
                        } else {
                            let field_tag_name = XmlNames::tag(&ident, obj_meta_info, Some(&field_meta));
                            field_setter.push(FieldSetter {
                                is_flatten: field_meta.contains("tree") || field_meta.contains("flatten"),
                                name: ident.clone(),
                                tag_name: field_tag_name,
                                unwrapped_type: TypeParser::unwrapped_type(&field.ty),
                            });
                        }

                        field_names.push(ident.clone());
                        constructors.push(if field_is_option { quote! { #ident } } else { quote! { #ident: #ident.unwrap() }})
                    }
                }
            }
        }
        Self { declarations, field_setter, attribute_setter, xmlns_setter, constructor: Constructor { values: constructors} }
    }
}
