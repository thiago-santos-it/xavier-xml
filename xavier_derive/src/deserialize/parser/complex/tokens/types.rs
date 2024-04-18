use syn::{parse_quote, Type};

pub struct TypeParser;

impl TypeParser {

    pub fn is_option_type(ty: &Type) -> bool {
        if let Type::Path(type_path) = ty {
            if let Some(path) = &type_path.path.segments.first() {
                return path.ident == "Option";
            }
        }
        false
    }

    pub fn unwrapped_type(ty: &Type) -> Type {
        if let Type::Path(type_path) = ty {
            if let Some(segment) = type_path.path.segments.first() {
                if segment.ident.to_string() == "Option" {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(inner_ty) = args.args.first() {
                            if let syn::GenericArgument::Type(inner_type) = inner_ty {
                                return Self::handle_vec_generics(inner_type);
                            }
                        }
                    }
                }
            }
        }
        Self::handle_vec_generics(ty)
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

    // Vec<Int> must be handled as Vec::<Int> in code
    fn handle_vec_generics(ty: &Type) -> Type {
        if let Type::Path(type_path) = ty {
            if let Some(segment) = type_path.path.segments.first() {
                println!("Path {}", segment.ident.to_string());
                if segment.ident.to_string() == "Vec" {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(inner_ty) = args.args.first() {
                            if let syn::GenericArgument::Type(inner_type) = inner_ty {
                                return parse_quote! { Vec::<#inner_type> }
                            }
                        }
                    }
                }
            }
        }
        ty.clone()
    }
}