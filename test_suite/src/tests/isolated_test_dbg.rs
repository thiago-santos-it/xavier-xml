use xavier::{from_obj, from_xml, from_xml_using_builder, PError, XmlDeserializable, XmlSerializable};

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct IsolatedTest {
    pub true_val: bool,
    pub false_val: bool,
    pub optional_true: Option<bool>,
    pub optional_false: Option<bool>,
    pub optional_none: Option<bool>,
}

#[test]
fn test_xml_boolean_values() -> Result<(), PError> {

    let test_data = IsolatedTest {
        true_val: true,
        false_val: false,
        optional_true: Some(true),
        optional_false: Some(false),
        optional_none: None,
    };

    let xml = from_obj(&test_data);
    println!("XML com valores booleanos: {}", xml);

    assert!(xml.contains("<true_val>true</true_val>"));
    assert!(xml.contains("<false_val>false</false_val>"));
    assert!(xml.contains("<optional_true>true</optional_true>"));
    assert!(xml.contains("<optional_false>false</optional_false>"));

    let parsed: IsolatedTest = from_xml_using_builder(&xml, IsolatedTest::from_xml_dbg)?;
    //let parsed: IsolatedTest = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

    Ok(())
}
impl IsolatedTest {

    pub fn from_xml_dbg(mut reader: &mut ::xavier::quick_xml::Reader<&[u8]>, start_event: Option<&::xavier::quick_xml::events::BytesStart>) -> Result<Self, xavier::PError> {
        if false {
            println!("[{}.Recursion] Parser started", "IsolatedTest");
        }

        let mut name = String::new();
        let mut true_val: Option<bool> = None;
        let mut false_val: Option<bool> = None;
        let mut optional_true: Option<bool> = None;
        let mut optional_false: Option<bool> = None;
        let mut optional_none: Option<bool> = None;

        if let Some(start_event) = start_event {
            for xa_attribute in start_event.attributes() {
                let xa_attr_name = String::from_utf8(xa_attribute.as_ref()?.key.0.to_vec())?;
                let xa_attr_value = String::from_utf8(xa_attribute.as_ref()?.value.to_vec())?;

                if false {
                    println!(
                        "[{}.Attribute] {}=\"{}\"",
                        "IsolatedTest",
                        xa_attr_name,
                        xa_attr_value
                    );
                }
            }
        }

        loop {
            match reader.read_event() {
                Err(error) => {
                    return Err(PError::new(&format!(
                        "Error at position {}: {:?}",
                        reader.buffer_position(),
                        error
                    )))
                }

                Ok(::xavier::quick_xml::events::Event::Start(event)) => {
                    let xa_tag_name = String::from_utf8(event.name().0.to_vec())?;

                    if false {
                        println!(
                            "[{}.{}.Start] Start Event",
                            "IsolatedTest",
                            xa_tag_name
                        );
                    }

                    let should_parse = (xa_tag_name == "true_val");
                    if should_parse {
                        let result = bool::from_xml(&mut reader, Some(&event));
                        match result {
                            Ok(t_value) => {
                                true_val = Some(t_value);
                                continue;
                            }
                            Err(error) => return Err(error),
                        }
                    }

                    let should_parse = (xa_tag_name == "false_val");
                    if should_parse {
                        let result = bool::from_xml(&mut reader, Some(&event));
                        match result {
                            Ok(t_value) => {
                                false_val = Some(t_value);
                                continue;
                            }
                            Err(error) => return Err(error),
                        }
                    }

                    let should_parse = (xa_tag_name == "optional_true");
                    if should_parse {
                        let result = bool::from_xml(&mut reader, Some(&event));
                        match result {
                            Ok(t_value) => {
                                optional_true = Some(t_value);
                                continue;
                            }
                            Err(error) => return Err(error),
                        }
                    }

                    let should_parse = (xa_tag_name == "optional_false");
                    if should_parse {
                        let result = bool::from_xml(&mut reader, Some(&event));
                        match result {
                            Ok(t_value) => {
                                optional_false = Some(t_value);
                                continue;
                            }
                            Err(error) => return Err(error),
                        }
                    }

                    let should_parse = (xa_tag_name == "optional_none");
                    if should_parse {
                        let result = bool::from_xml(&mut reader, Some(&event));
                        match result {
                            Ok(t_value) => {
                                optional_none = Some(t_value);
                                continue;
                            }
                            Err(error) => return Err(error),
                        }
                    }
                }

                Ok(::xavier::quick_xml::events::Event::Empty(event)) => {
                    let xa_tag_name = String::from_utf8(event.name().0.to_vec())?;

                    let should_parse = (xa_tag_name == "true_val");
                    if should_parse {
                        let result = bool::from_xml(&mut reader, Some(&event));
                        match result {
                            Ok(t_value) => {
                                true_val = Some(t_value);
                                continue;
                            }
                            Err(error) => return Err(error),
                        }
                    }

                    let should_parse = (xa_tag_name == "false_val");
                    if should_parse {
                        let result = bool::from_xml(&mut reader, Some(&event));
                        match result {
                            Ok(t_value) => {
                                false_val = Some(t_value);
                                continue;
                            }
                            Err(error) => return Err(error),
                        }
                    }

                    let should_parse = (xa_tag_name == "optional_true");
                    if should_parse {
                        let result = bool::from_xml(&mut reader, Some(&event));
                        match result {
                            Ok(t_value) => {
                                optional_true = Some(t_value);
                                continue;
                            }
                            Err(error) => return Err(error),
                        }
                    }

                    let should_parse = (xa_tag_name == "optional_false");
                    if should_parse {
                        let result = bool::from_xml(&mut reader, Some(&event));
                        match result {
                            Ok(t_value) => {
                                optional_false = Some(t_value);
                                continue;
                            }
                            Err(error) => return Err(error),
                        }
                    }

                    let should_parse = (xa_tag_name == "optional_none");
                    if should_parse {
                        let result = bool::from_xml(&mut reader, Some(&event));
                        match result {
                            Ok(t_value) => {
                                optional_none = Some(t_value);
                                continue;
                            }
                            Err(error) => return Err(error),
                        }
                    }
                }

                Ok(::xavier::quick_xml::events::Event::Text(_)) => {}
                Ok(::xavier::quick_xml::events::Event::CData(_)) => {}

                Ok(::xavier::quick_xml::events::Event::End(event)) => {
                    if String::from_utf8(event.name().0.to_vec())? == "IsolatedTest" {
                        if false {
                            println!("[{}.End] End Event ", "IsolatedTest");
                        }

                        return Ok(Self {
                            true_val: true_val.ok_or_else(|| {
                                PError::new(&format!(
                                    "Field value '{}' not found",
                                    stringify!(true_val)
                                ))
                            })?,
                            false_val: false_val.ok_or_else(|| {
                                PError::new(&format!(
                                    "Field value '{}' not found",
                                    stringify!(false_val)
                                ))
                            })?,
                            optional_true: xavier::deserialize::macro_trait::WrapWith::wrap(optional_true),
                            optional_false: xavier::deserialize::macro_trait::WrapWith::wrap(optional_false),
                            optional_none: xavier::deserialize::macro_trait::WrapWith::wrap(optional_none),
                        });
                    } else {
                        if false {
                            println!(
                                "[{}.{}.End] End Event ",
                                "IsolatedTest",
                                String::from_utf8(event.name().0.to_vec())?
                            );
                        }
                    }
                }

                Ok(::xavier::quick_xml::events::Event::Eof) => break,
                Ok(::xavier::quick_xml::events::Event::Decl(_)) => {}
                Ok(::xavier::quick_xml::events::Event::PI(_)) => {}
                Ok(::xavier::quick_xml::events::Event::DocType(_)) => {}
                Ok(::xavier::quick_xml::events::Event::Comment(_)) => {}
            };
        }

        Err(xavier::PError::new("Error root not found"))
    }
    /*pub fn from_xml_dbg(reader: &mut ::xavier::quick_xml::Reader<&[u8]>, start_event: Option<&::xavier::quick_xml::events::BytesStart>) -> Result<Self, xavier::PError> {
        println!("[{}.Recursion] Parser started", "IsolatedTest");

        let mut true_val: Option<bool> = None;
        let mut false_val: Option<bool> = None;
        let mut optional_true: Option<bool> = None;
        let mut optional_false: Option<bool> = None;
        let mut optional_none: Option<bool> = None;

        if let Some(start_event) = start_event {
            for xa_attribute in start_event.attributes() {
                let xa_attr_name = String::from_utf8(xa_attribute.as_ref()?.key.0.to_vec())?;
                let xa_attr_value = String::from_utf8(xa_attribute.as_ref()?.value.to_vec())?;

                if false {
                    println!(
                        "[{}.Attribute] {}=\"{}\"",
                        "IsolatedTest",
                        xa_attr_name,
                        xa_attr_value
                    );
                }
            }
        }

        loop {
            match reader.read_event() {
                Err(error) => {
                    return Err(PError::new(&format!(
                        "Error at position {}: {:?}",
                        reader.buffer_position(),
                        error
                    )))
                }

                Ok(::xavier::quick_xml::events::Event::Start(event)) => {
                    let xa_tag_name = String::from_utf8(event.name().0.to_vec())?;

                    if false {
                        println!(
                            "[{}.{}.Start] Start Event",
                            "IsolatedTest",
                            xa_tag_name
                        );
                    }

                    let should_parse = xa_tag_name == "true_val";
                    if should_parse {
                        let result = bool::from_xml(reader, Some(&event));
                        match result {
                            Ok(t_value) => {
                                true_val = Some(t_value);
                                continue;
                            }
                            Err(error) => return Err(error),
                        }
                    }

                    let should_parse = xa_tag_name == "false_val";
                    if should_parse {
                        let result = bool::from_xml(reader, Some(&event));
                        match result {
                            Ok(t_value) => {
                                false_val = Some(t_value);
                                continue;
                            }
                            Err(error) => return Err(error),
                        }
                    }

                    let should_parse = xa_tag_name == "optional_true";
                    if should_parse {
                        let result = bool::from_xml(reader, Some(&event));
                        match result {
                            Ok(t_value) => {
                                optional_true = Some(t_value);
                                continue;
                            }
                            Err(error) => return Err(error),
                        }
                    }

                    let should_parse = xa_tag_name == "optional_false";
                    if should_parse {
                        let result = bool::from_xml(reader, Some(&event));
                        match result {
                            Ok(t_value) => {
                                optional_false = Some(t_value);
                                continue;
                            }
                            Err(error) => return Err(error),
                        }
                    }

                    let should_parse = xa_tag_name == "optional_none";
                    if should_parse {
                        // Para campos opcionais que podem ser None, precisamos verificar se o tag está vazio
                        // ou se tem conteúdo
                        let mut has_content = false;
                        let mut depth = 0;

                        loop {
                            match reader.read_event() {
                                Err(error) => return Err(PError::new(&format!("Error at position {}: {:?}", reader.buffer_position(), error))),
                                Ok(::xavier::quick_xml::events::Event::Eof) => break,
                                Ok(::xavier::quick_xml::events::Event::Start(_)) => {
                                    depth += 1;
                                    has_content = true;
                                }
                                Ok(::xavier::quick_xml::events::Event::End(_)) => {
                                    depth -= 1;
                                    if depth <= 0 {
                                        break;
                                    }
                                }
                                Ok(::xavier::quick_xml::events::Event::Empty(_)) => {
                                    // Tag vazia, manter como None
                                    break;
                                }
                                Ok(::xavier::quick_xml::events::Event::Text(event)) => {
                                    let text = String::from_utf8(event.to_vec())?;
                                    let trimmed = text.trim();

                                    if !trimmed.is_empty() {
                                        if let Ok(value) = trimmed.parse::<bool>() {
                                            optional_none = Some(value);
                                        }
                                    }
                                }
                                Ok(::xavier::quick_xml::events::Event::CData(event)) => {
                                    let text = String::from_utf8(event.to_vec())?;
                                    let trimmed = text.trim();

                                    if !trimmed.is_empty() {
                                        if let Ok(value) = trimmed.parse::<bool>() {
                                            optional_none = Some(value);
                                        }
                                    }
                                }
                                Ok(::xavier::quick_xml::events::Event::Comment(_)) => {}
                                Ok(::xavier::quick_xml::events::Event::PI(_)) => {}
                                Ok(::xavier::quick_xml::events::Event::Decl(_)) => {}
                                Ok(::xavier::quick_xml::events::Event::DocType(_)) => {}
                            }
                        }

                        if !has_content {
                            optional_none = None;
                        }

                        continue;
                    }
                }

                Ok(::xavier::quick_xml::events::Event::Empty(event)) => {
                    let xa_tag_name = String::from_utf8(event.name().0.to_vec())?;

                    let should_parse = xa_tag_name == "true_val";
                    if should_parse {
                        let result = bool::from_xml(reader, Some(&event));
                        match result {
                            Ok(t_value) => {
                                true_val = Some(t_value);
                                continue;
                            }
                            Err(error) => return Err(error),
                        }
                    }

                    let should_parse = xa_tag_name == "false_val";
                    if should_parse {
                        let result = bool::from_xml(reader, Some(&event));
                        match result {
                            Ok(t_value) => {
                                false_val = Some(t_value);
                                continue;
                            }
                            Err(error) => return Err(error),
                        }
                    }

                    let should_parse = xa_tag_name == "optional_true";
                    if should_parse {
                        let result = bool::from_xml(reader, Some(&event));
                        match result {
                            Ok(t_value) => {
                                optional_true = Some(t_value);
                                continue;
                            }
                            Err(error) => return Err(error),
                        }
                    }

                    let should_parse = xa_tag_name == "optional_false";
                    if should_parse {
                        let result = bool::from_xml(reader, Some(&event));
                        match result {
                            Ok(t_value) => {
                                optional_false = Some(t_value);
                                continue;
                            }
                            Err(error) => return Err(error),
                        }
                    }

                    let should_parse = xa_tag_name == "optional_none";
                    if should_parse {
                        // Tag vazia, manter como None
                        optional_none = None;
                        continue;
                    }
                }

                Ok(::xavier::quick_xml::events::Event::Text(_)) => {}
                Ok(::xavier::quick_xml::events::Event::CData(_)) => {}

                Ok(::xavier::quick_xml::events::Event::End(event)) => {
                    if String::from_utf8(event.name().0.to_vec())? == "IsolatedTest" {
                        if false {
                            println!("[{}.End] End Event ", "IsolatedTest");
                        }

                        return Ok(Self {
                            true_val: true_val.ok_or_else(|| {
                                PError::new(&format!(
                                    "Field value '{}' not found",
                                    stringify!(true_val)
                                ))
                            })?,
                            false_val: false_val.ok_or_else(|| {
                                PError::new(&format!(
                                    "Field value '{}' not found",
                                    stringify!(false_val)
                                ))
                            })?,
                            optional_true: xavier::deserialize::macro_trait::WrapWith::wrap(optional_true),
                            optional_false: xavier::deserialize::macro_trait::WrapWith::wrap(optional_false),
                            optional_none: xavier::deserialize::macro_trait::WrapWith::wrap(optional_none),
                        });
                    } else {
                        if false {
                            println!(
                                "[{}.{}.End] End Event ",
                                "IsolatedTest",
                                String::from_utf8(event.name().0.to_vec())?
                            );
                        }
                    }
                }

                Ok(::xavier::quick_xml::events::Event::Eof) => break,
                Ok(::xavier::quick_xml::events::Event::Decl(_)) => {}
                Ok(::xavier::quick_xml::events::Event::PI(_)) => {}
                Ok(::xavier::quick_xml::events::Event::DocType(_)) => {}
                Ok(::xavier::quick_xml::events::Event::Comment(_)) => {}
            };
        }

        Err(xavier::PError::new("Error root not found"))
    }*/
}