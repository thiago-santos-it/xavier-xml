use xavier::{from_obj, XmlSerializable};
use xavier::encode;

#[derive(XmlSerializable, Debug)]
#[xml(name="my_child")]
struct ChildRecursion {
    #[xml(attribute, name="attr")]
    pub attribute: String,
    pub child_field_a: String,
    #[xml(tree)]
    pub inner: Option<Box<ChildRecursion>>
}

#[derive(XmlSerializable, Debug)]
#[xml(name="object", case="Camel")]
struct XMLObjectRecursion {
    pub field_a: String,
    #[xml(tree)]
    pub child: ChildRecursion
}

#[test]
fn serialize_recursion() {
    let should = r#"<object><fieldA>Some Text</fieldA><my_child attr="Outer Attr"><child_field_a>Outer Value</child_field_a><my_child attr="Inner Attr"><child_field_a>Inner Value</child_field_a><my_child attr="Deep Attr"><child_field_a>Deep Value</child_field_a></my_child></my_child></my_child></object>"#;
    let xml = XMLObjectRecursion { field_a: "Some Text".to_string(), child: ChildRecursion {
        attribute: "Outer Attr".to_string(),
        child_field_a: "Outer Value".to_string(),
        inner: Some(Box::new(ChildRecursion {
            attribute: "Inner Attr".to_string(),
            child_field_a: "Inner Value".to_string(),
            inner: Some(Box::new(ChildRecursion {
                attribute: "Deep Attr".to_string(),
                child_field_a: "Deep Value".to_string(),
                inner: None,
                }))})),
        },
    };
   assert_eq!(from_obj(&xml), should);
}

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
    let xml = XMLObject { field_a: encode!("Some Text"), child: Child { child_field_a: encode!("Other value")} };
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
    let xml = XMLObjectValued { field_a: encode!("Some Text"), child: Child { child_field_a: encode!("Other value")}, value: encode!("Value") };
    assert_eq!(from_obj(&xml), should);
}
