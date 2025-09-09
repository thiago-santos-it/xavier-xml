use xavier::{from_obj, from_xml, PError, XmlDeserializable, XmlSerializable};

#[test]
fn test_tag_with_attributes() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_child")]
    struct TestValueChildWithAttr {
        #[xml(attribute, name="attr")]
        pub attr: String,
        #[xml(content)]
        pub value: String
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object")]
    struct TestValueObjectWithAttr {
        pub id: u32,
        pub description: String,
        #[xml(tree)]
        pub child: TestValueChildWithAttr,
        #[xml(content)]
        pub value_a: String,
        #[xml(content)]
        pub value_b: String,
        #[xml(skip)]
        pub value_c: Option<String>
    }

    let data = TestValueObjectWithAttr {
        id: 2,
        description: "Test Description".to_string(),
        child: TestValueChildWithAttr { attr: "Attr Value".to_string(), value: "Other value".to_string() },
        value_a: "Something".to_string(),
        value_b: "".to_string(),
        value_c: None
    };

    let xml = from_obj(&data); //r#"<test_object><id>2</id><description>Test Description</description><child attr=\"Attr Value\">Other value</child>Something</test_object>"#;
    println!("{:#?}", xml);
    let obj: TestValueObjectWithAttr = from_xml(&xml)?;
    assert_eq!(obj.id, 2);
    assert_eq!(obj.description, "Test Description");
    assert_eq!(obj.value_a, "Something");
    assert_eq!(obj.value_a, obj.value_b);
    assert_eq!(obj.value_c, None);
    assert_eq!(obj.child.value, "Other value");

    Ok(())
}


#[test]
fn test_sibling_tags() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    struct TestValueChildWithAttr {
        #[xml(attribute, name="attr")]
        pub attr: String,
        #[xml(content)]
        pub value: String
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object")]
    struct TestValueObjectWithAttr {
        pub id: u32,
        pub description: String,
        #[xml(flatten, name="tag")]
        pub children: Vec<TestValueChildWithAttr>,
    }

    let data = TestValueObjectWithAttr {
        id: 2,
        description: "Test Description".to_string(),
        children: vec![TestValueChildWithAttr { attr: "Attr1".to_string(), value: "One value".to_string() }, TestValueChildWithAttr { attr: "Attr2".to_string(), value: "Other value".to_string() }],
    };

    let xml = from_obj(&data); //r#"<test_object><id>2</id><description>Test Description</description><child attr=\"Attr Value\">Other value</child>Something</test_object>"#;
    println!("{:#?}", xml);
    let obj: TestValueObjectWithAttr = from_xml(&xml)?;
    assert_eq!(obj.id, 2);
    assert_eq!(obj.description, "Test Description");
    assert_eq!(obj.children.len(), 2);
    assert_eq!(obj.children.first().unwrap().value, "One value");
    assert_eq!(obj.children.last().unwrap().value, "Other value");
    Ok(())
}


#[test]
fn test_sibling_tags_primitive() -> Result<(), PError> {

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object")]
    struct TestValueObjectWithAttr {
        pub id: u32,
        pub description: String,
        #[xml(flatten, name="tag")]
        pub children: Vec<String>,
    }

    let data = TestValueObjectWithAttr {
        id: 2,
        description: "Test Description".to_string(),
        children: vec!["One value".to_string() , "Other value".to_string()],
    };

    let xml = from_obj(&data);

    let obj: TestValueObjectWithAttr = from_xml(&xml)?;
    assert_eq!(obj.id, 2);
    assert_eq!(obj.description, "Test Description");
    assert_eq!(obj.children.len(), 2);
    assert_eq!(obj.children.first().unwrap(), "One value");
    assert_eq!(obj.children.last().unwrap(), "Other value");
    Ok(())
}