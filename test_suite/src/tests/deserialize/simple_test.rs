use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
struct XMLObject {
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = r#"<?-- This section must be ignored--?>
                       <?xml version="1.0" encoding = "UTF-8" ?>
                       <!DOCTYPE xml SYSTEM "Note.dtd">
                       <?PI Some Thing In The Way...?>
                       <?-- This section must be ignored--?>
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
