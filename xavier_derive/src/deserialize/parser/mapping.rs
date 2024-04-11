use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Data::Struct;
use syn::{DeriveInput, Fields, Type};
use crate::common::meta::{MetaInfo, MetaName};
use crate::common::naming::names::XmlNames;
use crate::deserialize::parser::constructor::Constructor;
use crate::deserialize::parser::declaration::FieldDecl;
use crate::deserialize::parser::setters::{AttributeSetter, FieldSetter};

pub struct FieldMapping {
    pub declarations: Vec<FieldDecl>,
    pub attribute_setter: Vec<AttributeSetter>,
    pub field_setter: Vec<FieldSetter>,
    pub constructor: Constructor
}

impl FieldMapping {

    pub fn field_mapping(input: &DeriveInput) -> FieldMapping {
        let obj_meta_info = MetaInfo::from_name(&input.attrs, MetaName::XML);

        let mut declarations: Vec<FieldDecl> = vec![];
        let mut field_setter: Vec<FieldSetter> = vec![];
        let mut attribute_setter: Vec<AttributeSetter> = vec![];
        let mut constructors: Vec<TokenStream> = vec![];
        let mut field_names: Vec<Ident> = vec![];

        if let Struct(struct_item) = &input.data {
            if let Fields::Named(fields) = &struct_item.fields {

                for field in fields.named.iter() {

                    if let Some(ident) = &field.ident {

                        let field_meta = MetaInfo::from_name(&field.attrs, MetaName::XML).unwrap_or(MetaInfo::empty());
                        let field_is_option = FieldMapping::is_option_type(&field.ty);
                        let ty = field.ty.clone();

                        declarations.push(FieldDecl {
                            name: ident.clone(),
                            optional_type: if field_is_option { quote! { #ty } } else { quote! { Option<#ty> }},
                        });

                        if field_meta.contains("attribute") {
                            let field_attr_name = XmlNames::attribute(&ident, obj_meta_info.as_ref(), &field_meta);
                            attribute_setter.push(AttributeSetter {
                                is_string: FieldMapping::is_string_type(&FieldMapping::unwrapped_type(&ty)),
                                name: ident.clone(),
                                attr_name: field_attr_name
                            });
                        } else {
                            let field_tag_name = XmlNames::tag(&ident, obj_meta_info.as_ref(), Some(&field_meta));
                            field_setter.push(FieldSetter {
                                name: ident.clone(),
                                tag_name: field_tag_name,
                                unwrapped_type: FieldMapping::unwrapped_type(&field.ty),
                            });
                        }

                        field_names.push(ident.clone());
                        constructors.push(if field_is_option { quote! { #ident } } else { quote! { #ident: #ident.unwrap() }})
                    }
                }
            }
        }
        FieldMapping { declarations, field_setter, attribute_setter, constructor: Constructor { values: constructors} }
    }

    fn is_option_type(ty: &Type) -> bool {
        if let Type::Path(type_path) = ty {
            if let Some(path) = &type_path.path.segments.first() {
                return path.ident == "Option";
            }
        }
        false
    }

    fn unwrapped_type(ty: &Type) -> Type {
        if let Type::Path(type_path) = ty {
            if let Some(segment) = type_path.path.segments.first() {
                if segment.ident.to_string() == "Option" {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(inner_ty) = args.args.first() {
                            if let syn::GenericArgument::Type(inner_type) = inner_ty {
                                return inner_type.clone();
                            }
                        }
                    }
                }
            }
        }
        ty.clone()
    }

    fn is_string_type(ty: &Type) -> bool {
        match ty {
            Type::Path(path) => {
                path.path.segments.last().map_or(false, |segment| {
                    segment.ident == "String"
                })
            }
            _ => false,
        }
    }
}
