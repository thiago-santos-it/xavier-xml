use xavier::{from_obj, from_xml, PError, XmlDeserializable, XmlSerializable};
/*
#[test]
fn test_value_handling_with_attributes() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_child")]
    struct TestValueChildWithAttr {
        #[xml(attribute, name="attr")]
        pub attr: String,
        #[con]
        pub value: String
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object")]
    struct TestValueObjectWithAttr {
        pub id: u32,
        pub description: String,
        #[xml(tree)]
        pub child: crate::tests::core::values::TestValueChildWithAttr,
        #[xml(value)]
        pub value_a: String,
        #[xml(skip)]
        pub value_b: Option<String>
    }

    let data = crate::tests::core::values::TestValueObjectWithAttr {
        id: 2,
        description: "Test Description".to_string(),
        child: crate::tests::core::values::TestValueChildWithAttr { attr: "Attr Value".to_string(), value: "Other value".to_string() },
        value_a: "Something".to_string(),
        value_b: None
    };

    let xml = from_obj(&data); //r#"<test_object><id>2</id><description>Test Description</description><child attr=\"Attr Value\">Other value</child>Something</test_object>"#;
    println!("{:#?}", xml);
    let obj: crate::tests::core::values::TestValueObjectWithAttr = from_xml(&xml)?;
    assert_eq!(obj.id, 2);
    assert_eq!(obj.description, "Test Description");
    assert_eq!(obj.value_a, "Something");
    assert_eq!(obj.value_b, None);
    assert_eq!(obj.child.value, "Other value");

    Ok(())
}

//Sibbling
*/