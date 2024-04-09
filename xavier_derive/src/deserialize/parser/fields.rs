use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::Data::Struct;
use syn::{DeriveInput, Fields, LitStr};
use syn::Type;

pub struct FieldDecl {
    pub name: Ident,
    pub optional_type: TokenStream
}

impl ToTokens for FieldDecl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = &self.optional_type;
        let field = &self.name;
        tokens.extend( quote! {
           let mut #field: #ty = None;
        });
    }
}

pub struct FieldAttr {
    pub is_option: bool,
    pub name: Ident,
    pub tag_name: LitStr,
    pub unwrapped_type: Type
}

impl ToTokens for FieldAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tag_name = &self.tag_name;
        let field = &self.name;
        let ty = &self.unwrapped_type;
        tokens.extend(quote! {
            if tag_name == #tag_name {
                let result = #ty::from_xml(&mut reader, Some(&event));
                match result {
                    Ok(value) => { #field = Some(value); }
                    Err(error) => { return Err(error); }
                }
            }
        })
    }
}

pub struct StructConstructor {
    pub values: Vec<TokenStream>
}

impl ToTokens for StructConstructor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let names = &self.values;
        tokens.extend(quote! { return Ok(Self{ #(#names,)* }); })
    }
}

pub struct FieldMapping {
    pub declarations: Vec<FieldDecl>,
    pub attributions: Vec<FieldAttr>,
    pub constructor: StructConstructor
}

impl FieldMapping {

    pub fn field_mapping(input: &DeriveInput) -> FieldMapping {
        let mut declarations: Vec<FieldDecl> = vec![];
        let mut attributions: Vec<FieldAttr> = vec![];
        let mut constructors: Vec<TokenStream> = vec![];
        let mut field_names: Vec<Ident> = vec![];

        if let Struct(struct_item) = &input.data {
            if let Fields::Named(fields) = &struct_item.fields {
                for field in fields.named.iter() {
                    if let Some(ident) = &field.ident {
                        let field_is_option = FieldMapping::is_option_type(&field.ty);
                        let ty = field.ty.clone();

                        declarations.push(FieldDecl {
                            name: ident.clone(),
                            optional_type: if field_is_option { quote! { #ty } } else { quote! { Option<#ty> }},
                        });

                        let field_tag_name = LitStr::new(&ident.to_string(), ident.span());
                        attributions.push(FieldAttr {
                            is_option: field_is_option,
                            name: ident.clone(),
                            tag_name: field_tag_name,
                            unwrapped_type: FieldMapping::unwrapped_type(&field.ty),
                        });
                        field_names.push(ident.clone());

                        constructors.push(if field_is_option { quote! { #ident } } else { quote! { #ident: #ident.unwrap() }})
                    }
                }
            }
        }
        FieldMapping { declarations, attributions, constructor: StructConstructor { values: constructors} }
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
}
