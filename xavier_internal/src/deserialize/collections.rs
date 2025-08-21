use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use crate::deserialize::error::PError;
use crate::deserialize::macro_trait::XmlDeserializable;


impl <T: XmlDeserializable> XmlDeserializable for Vec<T>  {

    fn from_xml(reader: &mut Reader<&[u8]>, start_event: Option<&BytesStart>) -> Result<Option<Self>, PError> {
        let mut children: Vec<T> = vec!();
        let tag_name = if let Some(start_event) = start_event {
            String::from_utf8(start_event.name().0.to_vec())?
        } else {
            return Err(PError::new("No tag name found for collection"));
        };

        loop {
            match reader.read_event() {
                Err(error) =>  { return Err(PError::new(&format!("Error at position {}: {:?}", reader.buffer_position(), error))) },
                Ok(Event::Eof) => { },
                Ok(Event::Start(event)) => {
                    children.push(
                        T::from_xml(reader, Some(&event))?
                            .ok_or_else(|| PError::new("Expected child element but got None"))?
                    );
                },
                Ok(Event::End(event)) => {
                    if String::from_utf8(event.name().0.to_vec())? == tag_name {
                        return Ok(Some(children))
                    }
                },
                Ok(Event::Empty(_)) => {},
                Ok(Event::Comment(_)) => {},
                Ok(Event::Text(_)) => {},
                Ok(Event::CData(_)) => {},
                Ok(Event::Decl(_)) => {},
                Ok(Event::PI(_)) => {},
                Ok(Event::DocType(_)) => {},
            }
        }
    }
}
