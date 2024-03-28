use quick_xml::events::Event;
use quick_xml::Reader;
use crate::deserialize::parser::declaration::XmlDeclaration;
use crate::deserialize::parser::doc_type::XmlDocType;
use crate::deserialize::parser::element::XmlElement;
use crate::deserialize::parser::empty_tag::XmlEmptyTag;
use crate::deserialize::parser::error::XmlError;
use crate::deserialize::parser::instructions::XmlPI;
use crate::deserialize::parser::tag::XmlTag;

#[derive(Debug)]
pub struct XmlRoot {
    pub declaration: Option<XmlDeclaration>,
    pub doc_type: Option<XmlDocType>,
    pub pis: Vec<XmlPI>,
    pub element: XmlElement
}

impl XmlRoot {
    pub fn parser(xml: &str) -> Result<XmlRoot, XmlError> {
        let mut reader = Reader::from_str(xml);
        let mut declaration: Option<XmlDeclaration> = None;
        let mut doc_type: Option<XmlDocType> = None;
        let mut pis: Vec<XmlPI> = vec!();
        let mut element: Option<XmlElement> = None;
        let mut root: Option<XmlRoot> = None;
        loop {
            match reader.read_event() {
                Err(error) => panic!("Error at position {}: {:?}", reader.buffer_position(), error),
                Ok(Event::Eof) =>  {
                    root = Some(XmlRoot { declaration, doc_type, pis, element: element.unwrap() });
                    break
                },
                Ok(Event::Start(event)) => element = Some(XmlElement::Tag(XmlTag::parser(&mut reader, event)?)),
                Ok(Event::End(_)) => {},
                Ok(Event::Empty(event)) => element = Some(XmlElement::Empty(XmlEmptyTag::parse(event)?)),
                Ok(Event::Decl(event)) => declaration = Some(XmlDeclaration::parse(event)?),
                Ok(Event::PI(event)) => pis.push(XmlPI::parse(event)?),
                Ok(Event::DocType(event)) => doc_type = XmlDocType::parse(event)?,
                Ok(Event::Text(event)) => {},
                Ok(Event::Comment(event)) => {},
                Ok(Event::CData(event)) => {},
            };
        };
        if let Some(root) = root  {
            Ok(root)
        } else {
            Err(XmlError::new("Fail to parse XML root! No end found."))
        }
    }
}