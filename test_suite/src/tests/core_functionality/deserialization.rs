use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
struct XMLObject {
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = r#"<XMLObject>
                    <some_string>Some String</some_string>
                    <some_int>10</some_int>
                    <some_float>11.1</some_float>
               </XMLObject>"#;
    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.some_string, "Some String");
    assert_eq!(obj.some_int, 10);
    assert_eq!(obj.some_float, 11.1);
    Ok(())
}

#[test]
fn deserialize_with_xml_declaration() -> Result<(), PError> {
    let xml = r#"<?xml version="1.0" encoding = "UTF-8" ?>
                       <XMLObject>
                            <some_string>Some String</some_string>
                            <some_int>10</some_int>
                            <some_float>11.1</some_float>
                       </XMLObject>"#;
    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.some_string, "Some String");
    assert_eq!(obj.some_int, 10);
    assert_eq!(obj.some_float, 11.1);
    Ok(())
}

#[test]
fn deserialize_with_doctype() -> Result<(), PError> {
    let xml = r#"<!DOCTYPE xml SYSTEM "Note.dtd">
                       <XMLObject>
                            <some_string>Some String</some_string>
                            <some_int>10</some_int>
                            <some_float>11.1</some_float>
                       </XMLObject>"#;
    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.some_string, "Some String");
    assert_eq!(obj.some_int, 10);
    assert_eq!(obj.some_float, 11.1);
    Ok(())
}

#[test]
fn deserialize_with_processing_instructions() -> Result<(), PError> {
    let xml = r#"<?PI Some Thing In The Way...?>
                       <XMLObject>
                            <some_string>Some String</some_string>
                            <some_int>10</some_int>
                            <some_float>11.1</some_float>
                       </XMLObject>"#;
    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.some_string, "Some String");
    assert_eq!(obj.some_int, 10);
    assert_eq!(obj.some_float, 11.1);
    Ok(())
}

#[test]
fn deserialize_with_comments() -> Result<(), PError> {
    let xml = r#"<?-- This section must be ignored--?>
                       <XMLObject>
                            <some_string>Some String</some_string>
                            <some_int>10</some_int>
                            <some_float>11.1</some_float>
                       </XMLObject>
                       <?-- This section must be ignored--?>"#;
    let obj: XMLObject = from_xml(&xml)?;
    assert_eq!(obj.some_string, "Some String");
    assert_eq!(obj.some_int, 10);
    assert_eq!(obj.some_float, 11.1);
    Ok(())
}