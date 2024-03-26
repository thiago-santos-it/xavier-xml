use proc_macro2::TokenStream;
use quote::{quote};
use syn::{DeriveInput, LitBool, LitStr};
use proc_macro2::Span;
use crate::common::meta::{MetaInfo, MetaName};
use crate::common::naming::names::XmlNames;
use crate::serialize::tree::xml::element::XmlElementDef;
use crate::serialize::tree::xml::header::XmlHeader;

pub fn stream(input: &DeriveInput) -> TokenStream {
    let obj_meta_info = MetaInfo::from_name(&input.attrs, MetaName::XML);
    let elements = XmlElementDef::parse(&input, obj_meta_info.as_ref());
    let tag = LitStr::new(&XmlNames::root(&input, obj_meta_info.as_ref()), Span::call_site());
    let header_tokens = XmlHeader::parse(&input, &tag);

    if let Some(elements) = elements {
        let attributes = elements.attributes;
        let children = elements.tags;
        let flatten = LitBool::new(obj_meta_info.unwrap_or(MetaInfo::empty()).contains("flatten"), Span::call_site());

        let namespace_tokens = if let Some(namespace) = elements.namespace {
            quote! { let namespace = &self.#namespace; }
        } else {
            quote! { let namespace = ""; }
        };

        quote! {
            #header_tokens
            #namespace_tokens

            let mut xml = String::new();
            let tag = #tag;

            xml.push_str(&header);
            xml.push_str(&dtd);

           let mut attributes = String::new();
            #(attributes.push_str(&#attributes);)*

          if !#flatten {
                xml.push_str("<");
                xml.push_str(&tag);
                if !namespace.is_empty() {
                    xml.push_str(&namespace);
                }
                if !attributes.is_empty() {
                    xml.push_str(&attributes);
                }
                xml.push_str(">");
            }

            let mut children = String::new();
            #(children.push_str(&#children);)*
            xml.push_str(&children);

            if !#flatten {
               xml.push_str(&format!("</{}>", tag));
            }
            xml
        }

    } else {
        quote! {
            #header_tokens
            let tag = #tag;
            format!("{}{}<{}></{}>", header, dtd, tag, tag).to_string()
        }
    }

}
