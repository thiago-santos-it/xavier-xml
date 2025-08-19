use proc_macro2::Ident;
use syn::{parse_quote, PathArguments, PathSegment, Type, TypePath};
use syn::GenericArgument;
use syn::PathArguments::AngleBracketed;

pub struct TypeParser;

impl TypeParser {

    pub fn type_path_idents(ty: &Type) -> Vec<Ident> {
        let mut idents = Vec::new();
        if let Type::Path(TypePath { path, .. }) = ty {
            for segment in &path.segments {
                if &segment.ident == "Option" || &segment.ident == "Box" {
                    idents.push(segment.ident.clone());
                }
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    for arg in &args.args {
                        if let GenericArgument::Type(inner_ty) = arg {
                            idents.extend(TypeParser::type_path_idents(inner_ty));
                        }
                    }
                }
            }
        }
        idents
    }

    pub fn is_vec(ty: &Type) -> bool {
        if let Type::Path(type_path) = ty {
            if let Some(path) = &type_path.path.segments.first() {
                return path.ident == "Vec";
            }
        }
        false
    }

    pub fn is_string_type(ty: &Type) -> bool {
        match ty {
            Type::Path(path) => {
                path.path.segments.last().map_or(false, |segment| {
                    segment.ident == "String"
                })
            }
            _ => false,
        }
    }

    pub fn unwrapped_type(ty: &Type) -> Type {
        Self::remove_from_type(ty, "Option")
    }

    pub fn unbox_and_unwrap_type(ty: &Type) -> Type {
        Self::remove_from_type(&Self::remove_from_type(ty, "Option"), "Box")
    }
    
    pub fn ty_from_vec(ty: &Type) -> Type {
        Self::remove_from_type(ty, "Vec")
    }

    fn remove_from_type(ty: &Type, remove: &str) -> Type {
        if let Some(segment)  = Self::first_path_segment(ty) {
            if segment.ident.to_string() == remove {
                if let Some(inner_type) = Self::type_from_segment_args(&segment) {
                    return Self::with_tuborfish(&inner_type);
                }
            }
        }
        Self::with_tuborfish(ty)
    }

    fn with_tuborfish(ty: &Type) -> Type{
        let boxed = TypeParser::handle_turbofish(&ty, syn::parse_quote! { Box }, "Box");
        return TypeParser::handle_turbofish(&boxed, syn::parse_quote! { Vec }, "Vec")
    }

    // Example: Vec<Int> must be handled as Vec::<Int> in code
    fn handle_turbofish(ty: &Type, segment_type: Type, segment_name: &str) -> Type {
        if let Some(segment)  = Self::first_path_segment(ty) {
            if segment.ident.to_string() == segment_name {
                if let Some(inner_type) = Self::type_from_segment_args(&segment) {
                    return parse_quote! { #segment_type::<#inner_type> }
                }
            }
        }
        ty.clone()
    }

    fn first_path_segment(ty: &Type) -> Option<PathSegment> {
        if let Type::Path(type_path) = ty {
            if let Some(path) = &type_path.path.segments.first() {
                let path = *path;
                return Some(path.clone())
            }
        }
        None
    }

    fn type_from_segment_args(segment: &PathSegment) -> Option<Type> {
        if let AngleBracketed(args) = &segment.arguments {
            if let Some(inner_ty) = args.args.first() {
                if let GenericArgument::Type(inner_type) = inner_ty {
                    return Some(inner_type.clone())
                }
            }
        }
        None
    }
}

#[test]
fn test_turbofish() {
    let ty = syn::parse_quote! { Box<Vec<i32>> };
    let result = TypeParser::handle_turbofish(
        &TypeParser::handle_turbofish(&ty, syn::parse_quote! { Box }, "Box"), syn::parse_quote! { Vec }, "Vec");
    assert_eq!(quote::quote! { #result }.to_string(), "Box :: < Vec < i32 > >");

    let ty = syn::parse_quote! { Box<i32> };
    let result = TypeParser::handle_turbofish(&TypeParser::handle_turbofish(
        &ty, syn::parse_quote! { Box }, "Box"), syn::parse_quote! { Vec }, "Vec");
    assert_eq!(quote::quote! { #result }.to_string(), "Box :: < i32 >");

    let ty = syn::parse_quote! { Vec<i32> };
    let result = TypeParser::handle_turbofish(&TypeParser::handle_turbofish(
        &ty, syn::parse_quote! { Box }, "Box"), syn::parse_quote! { Vec }, "Vec");
    assert_eq!(quote::quote! { #result }.to_string(), "Vec :: < i32 >");

    let ty = syn::parse_quote! { Box<i32> };
    let result = TypeParser::handle_turbofish(&TypeParser::handle_turbofish(
        &ty, syn::parse_quote! { Box }, "Box"), syn::parse_quote! { Vec }, "Vec");
    assert_eq!(quote::quote! { #result }.to_string(), "Box :: < i32 >");

    let ty = syn::parse_quote! { i32 };
    let result = TypeParser::handle_turbofish(&TypeParser::handle_turbofish(
        &ty, syn::parse_quote! { Box }, "Box"), syn::parse_quote! { Vec }, "Vec");
    assert_eq!(quote::quote! { #result }.to_string(), "i32");
}

#[test]
fn test_unbox() {
    let ty = syn::parse_quote! { Box<Vec<i32>> };
    let result = TypeParser::unbox_and_unwrap_type(&ty);
    assert_eq!(quote::quote! { #result }.to_string(), "Vec :: < i32 >");

    let ty = syn::parse_quote! { Box<i32> };
    let result = TypeParser::unbox_and_unwrap_type(&ty);
    assert_eq!(quote::quote! { #result }.to_string(), "i32");

    let ty = syn::parse_quote! { Option<Box<Vec<i32>>> };
    let result = TypeParser::unbox_and_unwrap_type(&ty);
    assert_eq!(quote::quote! { #result }.to_string(), "Vec :: < i32 >");

    let ty = syn::parse_quote! { Option<Box<i32>> };
    let result = TypeParser::unbox_and_unwrap_type(&ty);
    assert_eq!(quote::quote! { #result }.to_string(), "i32");
}
