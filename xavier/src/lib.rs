use std::panic;
use std::panic::{AssertUnwindSafe, PanicHookInfo};
use std::sync::{Arc, Mutex};
use quick_xml::events::Event;
pub use xavier_derive::XmlSerializable;
pub use xavier_derive::XmlDeserializable;

pub use xavier_internal::serialize::macro_trait::XmlSerializable;
pub use xavier_internal::deserialize::macro_trait::XmlDeserializable;
pub use xavier_internal::deserialize::error::PError;
pub use xavier_internal::encode;
pub use xavier_internal::namespaces;
pub use xavier_internal::cdata;
pub use xavier_internal::declaration;
pub use xavier_internal::doctype;
pub use xavier_internal::decode;
pub use xavier_internal::instructions;

pub use xavier_internal::serialize;
pub use xavier_internal::deserialize;

pub use ::quick_xml;

pub fn from_obj<T: XmlSerializable>(obj: &T) -> String {
    obj.to_xml(true)
}

pub fn from_xml<T: XmlDeserializable>(xml: &str) -> Result<T, PError> {
    from_xml_using_builder(xml, T::from_xml)
}

pub fn from_xml_using_builder<T, B>(xml: &str, builder: B) -> Result<T, PError>
where
    T: XmlDeserializable,
    B: Fn(&mut quick_xml::Reader<&[u8]>, Option<&quick_xml::events::BytesStart<'_>>) -> Result<T, PError>,
{
    if xml.trim().is_empty() {
        return Err(PError::new("Empty XML or whitespace-only content"));
    }

    let panic_info = Arc::new(Mutex::new(String::new()));

    panic::set_hook(Box::new({
        let panic_info = panic_info.clone();
        move |info: &PanicHookInfo| {
            if let Some(payload) = info.payload().downcast_ref::<&str>() {
                panic_info.lock().unwrap().push_str(*payload);
            } else {
                panic_info.lock().unwrap().push_str("Panic occurred");
            }
        }
    }));

    let result = panic::catch_unwind(AssertUnwindSafe( || {

        let mut reader = quick_xml::Reader::from_str(&xml);
        reader.config_mut().expand_empty_elements = true;
        let found_element = false;

        loop {
            match reader.read_event() {
                Err(error) =>  {
                    return Err(PError::new(&format!("Error at position {}: {:?}", reader.buffer_position(), error)))
                },
                Ok(Event::Eof) => {
                    if !found_element {
                        return Err(PError::new("No valid XML element found"));
                    }
                    break;
                },
                Ok(Event::Start(event)) => {
                    return Ok::<T, PError>(builder(&mut reader, Some(&event))?)
                },
                Ok(Event::End(_)) => {},
                Ok(Event::Empty(_)) => {
                    return Ok::<T, PError>(builder(&mut reader, None)?)
                },
                Ok(Event::Comment(_)) => {},
                Ok(Event::Text(_)) => {},
                Ok(Event::CData(_)) => {},
                Ok(Event::Decl(_)) => {},
                Ok(Event::PI(_)) => {},
                Ok(Event::DocType(_)) => {},
            }
        }

        Err(PError::new("No valid XML element found"))
    }));

    if let Err(_error) = result {
        Err(PError::new(&format!("Some error occurred in XML parser. Cause: {}", panic_info.lock().unwrap())))
    } else if let Ok(result) = result {
        Ok(result?)
    } else {
        Err(PError::new("Fail to parse XML, please check the structure and in case of bug please report on GitHub"))
    }
}