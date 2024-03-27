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
    pub children: Vec<Child>
}

#[test]
fn serialize() {
    let should = r#"<object><fieldA>Some Text</fieldA><children><my_child><child_field_a>Child A</child_field_a></my_child><my_child><child_field_a>Child B</child_field_a></my_child></children></object>"#;
    let xml = XMLObject { field_a: xtext!("Some Text"), children:
        vec![Child { child_field_a: xtext!("Child A")}, Child { child_field_a: xtext!("Child B")}] };
    assert_eq!(from_obj(&xml), should);
}
