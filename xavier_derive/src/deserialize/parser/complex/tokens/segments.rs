use proc_macro2::Ident;
use quote::quote;
use syn::Data::Struct;
use syn::{DeriveInput, Fields};
use crate::common::meta::{MetaInfo, MetaName};
use crate::common::naming::names::XmlNames;
use crate::deserialize::parser::complex::tokens::constructor::{Constructor, ConstructorField};
use crate::deserialize::parser::complex::tokens::declaration::FieldDecl;
use crate::deserialize::parser::complex::tokens::setters::attribute::FieldAttributeSetter;
use crate::deserialize::parser::complex::tokens::setters::field::FieldSetter;
use crate::deserialize::parser::complex::tokens::setters::sibling::SiblingSetter;
use crate::deserialize::parser::complex::tokens::setters::value::ValueSetter;
use crate::deserialize::parser::complex::tokens::setters::xmlns::FieldXmlnsSetter;
use crate::deserialize::parser::complex::tokens::types::TypeParser;

pub struct TokenSegments {
    pub declarations: Vec<FieldDecl>,
    pub attribute_setters: Vec<FieldAttributeSetter>,
    pub field_setters: Vec<FieldSetter>,
    pub sibling_setters: Vec<SiblingSetter>,
    pub value_setters: Vec<ValueSetter>,
    pub xmlns_setter: Option<FieldXmlnsSetter>,
    pub constructor: Constructor
}

impl TokenSegments {

    pub fn tokens_from(input: &DeriveInput, obj_meta_info: Option<&MetaInfo>) -> TokenSegments {

        let mut declarations: Vec<FieldDecl> = vec![];

        let mut field_setters: Vec<FieldSetter> = vec![];
        let mut sibling_setters: Vec<SiblingSetter> = vec![];
        let mut attribute_setters: Vec<FieldAttributeSetter> = vec![];
        let mut value_setters: Vec<ValueSetter> = vec![];
        let mut xmlns_setter: Option<FieldXmlnsSetter> = None;

        let mut constructors: Vec<ConstructorField> = vec![];
        let mut field_names: Vec<Ident> = vec![];

        if let Struct(struct_item) = &input.data {
            if let Fields::Named(fields) = &struct_item.fields {

                for field in fields.named.iter() {

                    if let Some(ident) = &field.ident {

                        let field_meta = MetaInfo::from_name(&field.attrs, MetaName::XML).unwrap_or(MetaInfo::empty());
                        let inner_type = TypeParser::unbox_and_unwrap_type(&field.ty);
                        let is_flatten = field_meta.contains("tree") || field_meta.contains("flatten");
                        let is_sibling = TypeParser::is_vec(&field.ty) && is_flatten;

                        declarations.push(FieldDecl {
                            name: ident.clone(),
                            optional_type: quote! { Option<#inner_type> },
                        });

                        if field_meta.contains("attribute") {
                            let field_attr_name = XmlNames::attribute(&ident, obj_meta_info, &field_meta);
                            attribute_setters.push(FieldAttributeSetter {
                                is_string: TypeParser::is_string_type(&inner_type),
                                name: ident.clone(),
                                attr_name: field_attr_name
                            });
                        } else if field_meta.contains("xmlns") {
                            xmlns_setter = Some(FieldXmlnsSetter { field: ident.clone() })
                        } else if field_meta.contains("value") {
                            value_setters.push(ValueSetter { field: ident.clone(), unwrapped_type: TypeParser::unwrapped_type(&field.ty) })
                        } else if is_sibling {
                            sibling_setters.push(SiblingSetter {
                                name: ident.clone(),
                                inner_type: TypeParser::ty_from_vec(&TypeParser::unbox_and_unwrap_type(&field.ty)),
                            });
                        } else {
                            let field_tag_name = XmlNames::tag(&ident, obj_meta_info, Some(&field_meta));
                            field_setters.push(FieldSetter {
                                name: ident.clone(),
                                is_flatten,
                                tag_name: field_tag_name,
                                inner_type: TypeParser::unbox_and_unwrap_type(&field.ty),
                            });
                        }

                        field_names.push(ident.clone());
                        constructors.push(ConstructorField {
                            is_sibling,
                            is_option: TypeParser::is_option_type(&field.ty),
                            is_box: TypeParser::is_box_type(&field.ty),
                            field: ident.clone(),
                        })
                    }
                }
            }
        }
        Self { declarations, field_setters, sibling_setters, attribute_setters, value_setters, xmlns_setter, constructor: Constructor { values: constructors } }
    }
}
