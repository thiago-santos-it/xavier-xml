use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub struct XmlSimpleTag;

impl XmlSimpleTag {

    pub fn parse(_: &DeriveInput) -> TokenStream {
        quote!{
            let mut current_value = String::new();
            loop {
                match reader.read_event() {
                    Err(error) =>  { return Err(PError::new(&format!("Error at position {}: {:?}", reader.buffer_position(), error))) },
                    Ok(::xavier::quick_xml::events::Event::Eof) => { },
                    Ok(::xavier::quick_xml::events::Event::Start(_)) => {},
                    Ok(::xavier::quick_xml::events::Event::End(_)) => { return Ok(Some(Self(current_value.parse()?)));  },
                    Ok(::xavier::quick_xml::events::Event::Empty(_)) => {},
                    Ok(::xavier::quick_xml::events::Event::Comment(_)) => {},
                    Ok(::xavier::quick_xml::events::Event::Text(event)) => { current_value.push_str(String::from_utf8(event.to_vec().unwrap_or(String::new()))); },
                    Ok(::xavier::quick_xml::events::Event::CData(event)) => { current_value.push_str(String::from_utf8(event.to_vec().unwrap_or(String::new()))); },
                    Ok(::xavier::quick_xml::events::Event::Decl(_)) => {},
                    Ok(::xavier::quick_xml::events::Event::PI(_)) => {},
                    Ok(::xavier::quick_xml::events::Event::DocType(_)) => {},
                }
            }
        }
    }
}
