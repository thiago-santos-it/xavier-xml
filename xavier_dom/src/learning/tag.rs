use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use crate::deserialize::parser::attribute::XmlAttribute;
use crate::deserialize::parser::comments::XmlComment;
use crate::deserialize::parser::element::XmlElement;
use crate::deserialize::parser::empty_tag::XmlEmptyTag;
use crate::deserialize::parser::error::XmlError;
use crate::deserialize::parser::values::XmlValue;

#[derive(Debug)]
pub struct XmlTag {
    pub attributes: Vec<XmlAttribute>,
    pub comments: Vec<XmlComment>,
    pub name: String,
    pub value: Option<XmlValue>,
    pub children: Vec<XmlElement>
}

impl XmlTag {
    pub fn parser(mut reader: &mut Reader<&[u8]>, event: BytesStart) -> Result<XmlTag, XmlError> {

        let name = String::from_utf8(event.name().0.to_vec())?;

        let mut value: Option<XmlValue> = None;

        let mut children: Vec<XmlElement> = vec!();
        let mut comments: Vec<XmlComment> = vec!();
        let mut attributes: Vec<XmlAttribute> = vec!();

        let mut tag: Option<XmlTag> = None; //Current

        for attribute in event.attributes() {
            attributes.push(XmlAttribute::parse(&attribute?)?);
        }

        loop {
            match reader.read_event() {
                Err(error) => panic!("Error at position {}: {:?}", reader.buffer_position(), error),
                Ok(Event::Eof) => break,
                Ok(Event::Start(event)) => children.push(XmlElement::Tag(XmlTag::parser(&mut reader, event)?)),
                Ok(Event::End(event)) => {
                    if String::from_utf8(event.name().0.to_vec())? == name {
                        tag = Some(XmlTag { children, comments, attributes, name, value });
                        break;
                    }
                },
                Ok(Event::Empty(event)) => children.push(XmlElement::Empty(XmlEmptyTag::parse(event)?)),
                Ok(Event::Comment(event)) => comments.push(XmlComment::parse(event)?),
                Ok(Event::Text(event)) => { },
                Ok(Event::CData(event)) => {},
                Ok(Event::Decl(_)) => {},
                Ok(Event::PI(_)) => {},
                Ok(Event::DocType(_)) => {},
            };
        };
        if let Some(tag) = tag {
            Ok(tag)
        } else {
            Err(XmlError::new(&format!("Fail to parse XML! No end found for tag {}", String::from_utf8(event.name().0.to_vec())?)))
        }
    }
}