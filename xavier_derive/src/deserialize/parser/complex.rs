use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, LitStr};

use crate::common::meta::{MetaInfo, MetaName};
use crate::common::naming::names::XmlNames;
use crate::deserialize::parser::fields::FieldMapping;

pub(crate) struct XmlComplexTag;

impl XmlComplexTag {
    pub fn parse(input: &DeriveInput) -> TokenStream {
        let obj_meta_info = MetaInfo::from_name(&input.attrs, MetaName::XML);
        let tag = LitStr::new(&XmlNames::root(&input, obj_meta_info.as_ref()), Span::call_site());

        let field_mapping = FieldMapping::field_mapping(input);
        let declarations = field_mapping.declarations;
        let attributions = field_mapping.attributions;
        let constructor =  field_mapping.constructor;

        let mut result = quote! {
                #(#declarations)*
                loop {
                    match reader.read_event() {
                        Err(error) => { return Err(xavier::PError::new(&format!("Error at position {}: {:?}", reader.buffer_position(), error))) },
                        Ok(quick_xml::events::Event::Eof) => { break },
                        Ok(quick_xml::events::Event::Start(event)) => {
                            let tag_name = String::from_utf8(event.name().0.to_vec())?;
                            #(#attributions)*
                        },
                        Ok(quick_xml::events::Event::End(_)) => {},
                        Ok(quick_xml::events::Event::Empty(_)) => {},
                        Ok(quick_xml::events::Event::Decl(_)) => {},
                        Ok(quick_xml::events::Event::PI(_)) => {},
                        Ok(quick_xml::events::Event::DocType(_)) => {},
                        Ok(quick_xml::events::Event::Text(_)) => {},
                        Ok(quick_xml::events::Event::Comment(_)) => {},
                        Ok(quick_xml::events::Event::CData(_)) => {},
                    };
                };
            #constructor
        };
        let debug =  LitStr::new(&result.to_string(), Span::call_site());
        result.extend(quote! {
            println!("Generated Code: \n\n {}", #debug);
            Err(xavier::PError::new("Declaration not found!"))
        });
        result
    }
}

