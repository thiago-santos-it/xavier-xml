use xavier::{from_obj, XmlSerializable};

#[derive(XmlSerializable, Debug)]
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
fn serialize() {
    let should = r#"<XMLObject attr_string="attr_string" opt_attr_string="opt_attr_string" none_string=""><some_string></some_string><some_int>1</some_int><some_float>10</some_float><opt_some_string>opt_some_string</opt_some_string><opt_some_int>12</opt_some_int><opt_some_float>11.1</opt_some_float><none_some_string></none_some_string><none_some_int></none_some_int><none_some_float></none_some_float></XMLObject>"#;
    let xml = XMLObject {
        attr_string: "attr_string".to_string(),
        opt_attr_string: Some("opt_attr_string".to_string()),
        none_string: None,
        some_string: "".to_string(),
        some_int: 1,
        some_float: 10.0,
        opt_some_string: Some("opt_some_string".to_string()),
        opt_some_int: Some(12),
        opt_some_float: Some(11.1),
        none_some_string: None,
        none_some_int: None,
        none_some_float: None,
    };
    assert_eq!(from_obj(&xml), should);
}
