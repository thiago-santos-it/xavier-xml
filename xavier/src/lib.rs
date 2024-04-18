use std::panic;
use std::panic::PanicInfo;
use std::sync::{Arc, Mutex};
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

pub fn from_obj<T: XmlSerializable>(obj: &T) -> String {
    obj.to_xml(true)
}

pub fn from_xml<T: XmlDeserializable>(xml: &str) -> Result<T, PError> {

    let panic_info = Arc::new(Mutex::new(String::new()));

    panic::set_hook(Box::new({
        let panic_info = panic_info.clone();
        move |info: &PanicInfo| {
            if let Some(payload) = info.payload().downcast_ref::<&str>() {
                panic_info.lock().unwrap().push_str(*payload);
            } else {
                panic_info.lock().unwrap().push_str("Panic occurred");
            }
        }
    }));

    let result = panic::catch_unwind(|| {
        let mut reader = quick_xml::Reader::from_str(&xml);
        return Ok::<T, PError>(T::from_xml(&mut reader, None)?)
    });

    return if let Err(error) = result {
        Err(PError::new(&format!("Some error occurred in XML parser. Cause: {}", panic_info.lock().unwrap())))
    } else if let Ok(result) = result {
        Ok(result?)
    } else {
        Err(PError::new("Fail to parse XML, please check the structure and in case of bug please report on GitHub"))
    }
}