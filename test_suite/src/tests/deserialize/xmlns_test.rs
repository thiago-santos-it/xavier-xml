use xavier::{PError, XmlDeserializable};
use xavier::serialize::namespaces::Namespaces;

#[derive(XmlDeserializable, Debug)]
struct XMLObject {
    #[xml(xmlns)]
    pub namespaces: Namespaces,
    #[xml(attribute)]
    pub some_string: String,
    #[xml(attribute)]
    pub some_int: i32,
    pub some_float: f32
}

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = r#"<XMLObject xmlns:xhtml="http://www.w3.org/1999/xhtml" xmlns:xhtml2="http://www.w3.org/1999/xhtml" some_string="Some text" some_int="11"><some_float>10</some_float></XMLObject>"#;
    let mut reader = quick_xml::Reader::from_str(&xml);
    let obj = XMLObject::from_xml(&mut reader, None)?;
    assert_eq!(obj.some_string, "Some text");
    assert_eq!(obj.some_int, 11);
    assert_eq!(obj.some_float, 10.0);
    assert_eq!("xmlns:xhtml=http://www.w3.org/1999/xhtml xmlns:xhtml2=http://www.w3.org/1999/xhtml", obj.namespaces);
    Ok(())
}
