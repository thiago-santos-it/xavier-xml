use xavier::{from_obj, XmlSerializable};
use xavier::encode;

#[derive(XmlSerializable)]
#[pi(something key="value" flag)]
#[pi(something key="value" flag2)]
#[xml(name="xml")]
struct XMLObject {
    pub some_string: String,
    #[pi(something key="value" flag3)]
    pub some_int: i32,
    pub some_float: f32
}

#[test]
fn serialize() {
    let should = r#"<?something key="value" flag?><?something key="value" flag2?><xml><some_string>Some Text</some_string><?something key="value" flag3?><some_int>0</some_int><some_float>0</some_float></xml>"#;
    let xml = XMLObject { some_string: encode!("Some Text"), some_int: 0, some_float: 0.0 };
    assert_eq!(from_obj(&xml), should);
}
