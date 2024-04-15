use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, LitStr};

use crate::deserialize::parser::mapping::FieldMapping;

pub(crate) struct ParserLoop;

impl ParserLoop {
    pub fn parse(input: &DeriveInput) -> TokenStream {

        let field_mapping = FieldMapping::field_mapping(input);
        let declarations = field_mapping.declarations;
        let attribute_setter = field_mapping.attribute_setter;
        let field_setter = field_mapping.field_setter;
        let xmlns_setter = field_mapping.xmlns_setter;
        let constructor =  field_mapping.constructor;

        let mut result = quote! {
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
                        Ok(quick_xml::events::Event::End(_)) => {},
                        Ok(quick_xml::events::Event::Empty(_)) => {},
                        Ok(quick_xml::events::Event::Decl(_)) => {},
                        Ok(quick_xml::events::Event::PI(_)) => {},
                        Ok(quick_xml::events::Event::DocType(_)) => {},
                        Ok(quick_xml::events::Event::Text(_)) => {},
                        Ok(quick_xml::events::Event::Comment(_)) => {},
                        Ok(quick_xml::events::Event::CData(_)) => {}
                    };
                };
        };
        let debug =  LitStr::new(&result.to_string(), Span::call_site());
        result.extend(quote! {
            let _ = #debug;
            //println!("Generated Code: \n\n {}", #debug);
            #constructor
        });
        result
    }
}

