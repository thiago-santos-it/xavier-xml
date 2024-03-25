use proc_macro2::TokenStream;
use quote::{quote};
use syn::{ DeriveInput, LitStr, Meta};
use crate::serialize::parser::meta::{MetaInfo, MetaName};
use crate::serialize::parser::naming::object_name;
use crate::serialize::parser::fields::{parse, TagElement};


pub fn stream(input: &DeriveInput) -> TokenStream {
    let obj_meta_info = MetaInfo::from_name(&input.attrs, MetaName::XML);
    let elements = parse(&input, obj_meta_info.as_ref());
    let tag = LitStr::new(&object_name(&input, obj_meta_info.as_ref()), proc_macro2::Span::call_site());
    let header_tokens = header_quote(&input, &tag);

    if let Some(elements) = elements {
        let attributes = elements.attributes;
        let fields = elements.tags;
        let namespace = elements.namespace;

        quote! {
            #header_tokens
            let attributes = "".to_string() +
                #(#attributes + )* "";
            let fields = "".to_string() +
                #(#fields + )* "";
            let namespace = #namespace;
            let tag = #tag;
            format!("{}{}<{} {} {}>{}{}</{}>", header, dtd, tag, namespace, attributes, fields, complex, tag).to_string()
        }
    } else {
        quote! {
            #header_tokens
            let tag = #tag;
            format!("{}{}<{}></{}>", header, dtd, tag, tag).to_string()
        }
    }

}

fn header_quote(input: &DeriveInput, tag: &LitStr) -> TokenStream {
    let header = parse_header(&input);
    let dtd = parse_dtd_file(&input);
    quote! {
        let header = if root { #header } else { "" }
        let dtd = if root { format!("<!DOCTYPE {} SYSTEM \"{}\">", #tag, #dtd) } else { "" }
    }
}

fn parse_header(input: &DeriveInput) -> LitStr {
   if let Some(header) = MetaInfo::from_name(&input.attrs, MetaName::Header) {
        let version = header.get_or("version", "1.0".to_string());
        let encoding = header.get_or("encoding", "UTF-8".to_string());
        let standalone = header.get_or("standalone", "no".to_string());
        let header_tag = format!("<?xml version = \"{}\" encoding = \"{}\" standalone = \"{}\" ?>",
                                 version, encoding, standalone);
        LitStr::new(&header_tag, proc_macro2::Span::call_site())
    } else {
        LitStr::new(&"", proc_macro2::Span::call_site())
    }
}

fn parse_dtd_file(input: &DeriveInput) -> LitStr {
    let dtd = MetaInfo::attr_by_name(&input.attrs, MetaName::DTD);
    if let Some(dtd) = dtd {
        if let Meta::NameValue(dtd) = &dtd.meta {
            if let syn::Expr::Lit(lit) = &dtd.value {
                if let syn::Lit::Str(dtd_str) = &lit.lit {
                    return dtd_str.clone()
                }
            }
        }
    }
    LitStr::new(&"", proc_macro2::Span::call_site())
}




