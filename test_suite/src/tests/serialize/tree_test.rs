use xavier::{from_obj, XmlSerializable};
use xavier::xtext;

#[derive(XmlSerializable)]
#[xml(name="my_child")]
struct Child {
    pub child_field_a: String,
}

#[derive(XmlSerializable)]
#[xml(name="object", case="Camel")]
struct XMLObject {
    pub field_a: String,
    #[xml(tree)]
    pub child: Child
}

#[test]
fn serialize() {
    let should = r#"<object><fieldA>Some Text</fieldA><my_child><child_field_a>Other value</child_field_a></my_child></object>"#;
    let xml = XMLObject { field_a: xtext!("Some Text"), child: Child { child_field_a: xtext!("Other value")} };
    assert_eq!(from_obj(&xml), should);
}


#[derive(XmlSerializable)]
#[xml(name="object", case="Camel")]
struct XMLObjectValued {
    pub field_a: String,
    #[xml(tree)]
    pub child: Child,
    #[xml(value)]
    pub value: String
}

#[test]
fn valued() {
    let should = r#"<object><fieldA>Some Text</fieldA><my_child><child_field_a>Other value</child_field_a></my_child>Value</object>"#;
    let xml = XMLObjectValued { field_a: xtext!("Some Text"), child: Child { child_field_a: xtext!("Other value")}, value: xtext!("Value") };
    assert_eq!(from_obj(&xml), should);
}
