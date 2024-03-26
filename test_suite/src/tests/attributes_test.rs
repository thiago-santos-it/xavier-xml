use xavier::{from_obj, XmlSerializable};

#[derive(XmlSerializable)]
#[xml(ns="a", name="object", case="Camel")]
struct XMLObject {
    #[xml(attribute, name="just_string")]
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}

#[test]
fn serialize() {
    let should = r#"<a:object justString="Some Text"><a:someInt>0</a:someInt><a:someFloat>0</a:someFloat></a:object>"#;
    let xml = XMLObject { some_string: "Some Text".to_string(), some_int: 0, some_float: 0.0 };
    assert_eq!(from_obj(&xml), should);
}


#[derive(XmlSerializable)]
#[xml(ns="a", name="object", case="Camel", suffix="su", prefix="pre")]
struct XMLObjectFix {
    #[xml(attribute, name="just_string", use_suffix, use_prefix)]
    pub some_string: String,
    #[xml(attribute, name="just_int", ignore_case)]
    pub some_int: i32,
    pub some_float: f32
}

#[test]
fn prefix_some_suffix() {
    let should = r#"<a:preobjectsu prejustStringsu="Some Text" just_int="0"><a:presomeFloatsu>0</a:presomeFloatsu></a:preobjectsu>"#;
    let xml = XMLObjectFix { some_string: "Some Text".to_string(), some_int: 0, some_float: 0.0 };
    assert_eq!(from_obj(&xml), should);
}
