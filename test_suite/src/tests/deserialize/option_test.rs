use xavier::{PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
struct XMLObject {
    #[xml(attribute)]
    pub attr_string: String,
    #[xml(attribute)]
    pub opt_attr_string: Option<String>,
    #[xml(attribute)]
    pub none_string: Option<String>,
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32,
    pub opt_some_string: Option<String>,
    pub opt_some_int: Option<i32>,
    pub opt_some_float: Option<f32>,
    pub none_some_string: Option<String>,
    pub none_some_int: Option<i32>,
    pub none_some_float: Option<f32>
}

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = r#"
    <XMLObject attr_string="Hi!" opt_attr_string="Some(Hi!)">
        <some_string>Some String</some_string>
        <some_int>11</some_int>
        <some_float>10</some_float>
        <opt_some_string>Some String</opt_some_string>
        <opt_some_int>11</opt_some_int>
        <opt_some_float>10</opt_some_float>
    </XMLObject>"#;
    let mut reader = quick_xml::Reader::from_str(&xml);
    let obj = XMLObject::from_xml(&mut reader, None)?;
    assert_eq!(obj.some_string, "Some String");
    assert_eq!(obj.some_int, 11);
    assert_eq!(obj.some_float, 10.0);
    assert_eq!(obj.attr_string, "Hi!".to_string());
    assert_eq!(obj.opt_some_string, Some("Some String".to_string()));
    assert_eq!(obj.opt_some_int, Some(11));
    assert_eq!(obj.opt_some_float, Some(10.0));
    assert_eq!(obj.opt_attr_string, Some("Some(Hi!)".to_string()));
    assert_eq!(obj.none_some_string, None);
    assert_eq!(obj.none_some_int, None);
    assert_eq!(obj.none_some_float, None);
    assert_eq!(obj.none_string, None);
    Ok(())
}
