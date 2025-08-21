use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub struct XmlEnum;

impl XmlEnum {
    pub fn parse(_: &DeriveInput) -> TokenStream {
        quote!{
            loop {
                match reader.read_event() {
                    Err(error) =>  { return Err(PError::new(&format!("Error at position {}: {:?}", reader.buffer_position(), error))) },
                    Ok(::xavier::quick_xml::events::Event::Eof) => { },
                    Ok(::xavier::quick_xml::events::Event::Start(_)) => {},
                    Ok(::xavier::quick_xml::events::Event::End(_)) => {},
                    Ok(::xavier::quick_xml::events::Event::Empty(_)) => {},
                    Ok(::xavier::quick_xml::events::Event::Comment(_)) => {},
                    Ok(::xavier::quick_xml::events::Event::Text(event)) => { return Ok(Some(String::from_utf8(event.to_vec())?.parse()?)); },
                    Ok(::xavier::quick_xml::events::Event::CData(event)) => { return Ok(Some(String::from_utf8(event.to_vec())?.parse()?)); },
                    Ok(::xavier::quick_xml::events::Event::Decl(_)) => {},
                    Ok(::xavier::quick_xml::events::Event::PI(_)) => {},
                    Ok(::xavier::quick_xml::events::Event::DocType(_)) => {},
                }
            }
        }
    }
}
