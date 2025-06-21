use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, LitBool, LitStr};
use crate::common::meta::{MetaInfo, MetaName};
use crate::common::naming::names::XmlNames;
use crate::deserialize::parser::complex::tokens::segments::TokenSegments;

pub struct XmlComplex;

impl XmlComplex {
    pub fn parse(input: &DeriveInput) -> TokenStream {

        let obj_meta_info = MetaInfo::from_name(&input.attrs, MetaName::XML);
        let xml_tag_name = LitStr::new(&XmlNames::root(&input, obj_meta_info.as_ref()), Span::call_site());

        let tokens = TokenSegments::tokens_from(input, obj_meta_info.as_ref());
        let declarations = tokens.declarations;
        let attribute_setters = tokens.attribute_setters;
        let field_setters = tokens.field_setters;
        let sibling_setters = tokens.sibling_setters;
        let value_setters = tokens.value_setters;
        let xmlns_setter = tokens.xmlns_setter;
        let constructor =  tokens.constructor;

        let debug = LitBool::new(false, Span::call_site());
        let print_code = LitBool::new(false, Span::call_site());

        let gen = quote! {
            if #debug { println!("[{}.Recursion] Parser started", #xml_tag_name); }

            let mut name = String::new();
            #(#declarations)*

            if let Some(start_event) = start_event {
                for xa_attribute in start_event.attributes() {
                    let xa_attr_name = String::from_utf8(xa_attribute.as_ref()?.key.0.to_vec())?;
                    let xa_attr_value = String::from_utf8(xa_attribute.as_ref()?.value.to_vec())?;
                    if #debug { println!("[{}.Attribute] {}=\"{}\"", #xml_tag_name, xa_attr_name, xa_attr_value); }
                    #(#attribute_setters)*
                    #xmlns_setter
                }
            }

            loop {
                match reader.read_event() {
                    Err(error) =>  { return Err(PError::new(&format!("Error at position {}: {:?}", reader.buffer_position(), error))) },
                    Ok(::xavier::quick_xml::events::Event::Start(event)) => {
                        let xa_tag_name = String::from_utf8(event.name().0.to_vec())?;
                        if #debug { println!("[{}.{}.Start] Start Event", #xml_tag_name, xa_tag_name); }
                        #(#field_setters)*
                        #(#sibling_setters)*
                    },
                    Ok(::xavier::quick_xml::events::Event::Empty(event)) => {
                        let xa_tag_name = String::from_utf8(event.name().0.to_vec())?;
                        #(#field_setters)*
                        #(#sibling_setters)*
                    },
                    Ok(::xavier::quick_xml::events::Event::Text(event)) => {
                        #(#value_setters)*
                    },
                    Ok(::xavier::quick_xml::events::Event::CData(event)) => {
                        #(#value_setters)*
                    },
                    Ok(::xavier::quick_xml::events::Event::End(event)) => {
                        if String::from_utf8(event.name().0.to_vec())? == #xml_tag_name {
                            if #debug { println!("[{}.End] End Event ", #xml_tag_name); }
                            #constructor
                        } else {
                            if #debug { println!("[{}.{}.End] End Event ", #xml_tag_name, String::from_utf8(event.name().0.to_vec())?); }
                        }
                    },
                    Ok(::xavier::quick_xml::events::Event::Eof) => { break },
                    Ok(::xavier::quick_xml::events::Event::Decl(_)) => {},
                    Ok(::xavier::quick_xml::events::Event::PI(_)) => {},
                    Ok(::xavier::quick_xml::events::Event::DocType(_)) => {},
                    Ok(::xavier::quick_xml::events::Event::Comment(_)) => {}
                };
            };
            Err(xavier::PError::new("Error root not found"))
        };

        let gen_code_lit =  LitStr::new(&gen.to_string(), Span::call_site());
        let mut result = quote! {};
        result.extend(quote! {
            let _ = #gen_code_lit;
            if #print_code { println!("\n\n Generated Code: {} \n\n  {}", #xml_tag_name, #gen_code_lit); }
        });
        result.extend(gen);
        result
    }
}
