use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, LitStr};
use crate::common::meta::{MetaInfo, MetaName};
use crate::common::naming::names::XmlNames;

use crate::deserialize::parser::mapping::FieldMapping;

pub(crate) struct ParserLoop;

impl ParserLoop {
    pub fn parse(input: &DeriveInput) -> TokenStream {
        let obj_meta_info = MetaInfo::from_name(&input.attrs, MetaName::XML);
        let xml_tag_name = LitStr::new(&XmlNames::root(&input, obj_meta_info.as_ref()), Span::call_site());

        let field_mapping = FieldMapping::field_mapping(input, obj_meta_info.as_ref());
        let declarations = field_mapping.declarations;
        let attribute_setter = field_mapping.attribute_setter;
        let field_setter = field_mapping.field_setter;
        let xmlns_setter = field_mapping.xmlns_setter;
        let constructor =  field_mapping.constructor;


        let gen = quote! {
            let mut name = String::new();
            #(#declarations)*
            loop {
                match reader.read_event() {
                    Err(error) => { return Err(xavier::PError::new(&format!("Error at position {}: {:?}", reader.buffer_position(), error))) },
                    Ok(quick_xml::events::Event::Eof) => { break },
                    Ok(quick_xml::events::Event::Start(event)) => {
                        let tag_name = String::from_utf8(event.name().0.to_vec())?;
                        #(#field_setter)*
                        for attribute in event.attributes() {
                            let attr_name = String::from_utf8(attribute.as_ref()?.key.0.to_vec())?;
                            let attr_value = String::from_utf8(attribute.as_ref()?.value.to_vec())?;
                            #(#attribute_setter)*
                            #xmlns_setter
                        }
                    },
                    Ok(quick_xml::events::Event::End(event)) => {
                        if String::from_utf8(event.name().0.to_vec())? == #xml_tag_name {
                            #constructor
                        }
                    },
                    Ok(quick_xml::events::Event::Empty(_)) => {},
                    Ok(quick_xml::events::Event::Decl(_)) => {},
                    Ok(quick_xml::events::Event::PI(_)) => {},
                    Ok(quick_xml::events::Event::DocType(_)) => {},
                    Ok(quick_xml::events::Event::Text(_)) => {},
                    Ok(quick_xml::events::Event::Comment(_)) => {},
                    Ok(quick_xml::events::Event::CData(_)) => {}
                };
            };
            Err(xavier::PError::new("Error root not found"))
        };
        let debug =  LitStr::new(&gen.to_string(), Span::call_site());
        let mut result = quote! {};
        result.extend(quote! {
            let _ = #debug;
            //println!("Generated Code: \n\n {}", #debug);
        });
        result.extend(gen);
        result
    }
}

