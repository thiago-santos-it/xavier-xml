use xavier::{from_xml, from_obj, PError, XmlDeserializable, XmlSerializable};

#[test]
fn test_value_handling_basic_roundtrip() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_child")]
    struct TestValueChild {
        #[xml(value)]
        pub value: String,
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestValueObject {
        pub id: u32,
        pub name: String,
        #[xml(tree)]
        pub child: TestValueChild,
        #[xml(value)]
        pub value_a: String,
        #[xml(value)]
        pub value_b: String,
    }

    let test_data = TestValueObject {
        id: 1,
        name: "Test Value Object".to_string(),
        child: TestValueChild {
            value: "Child Value".to_string(),
        },
        value_a: "Value A".to_string(),
        value_b: "Value B".to_string(),
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<test_object>"));
    assert!(xml.contains("</test_object>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Value Object</name>"));
    assert!(xml.contains("<test_child>"));
    assert!(xml.contains("</test_child>"));
    
    let parsed: TestValueObject = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_value_handling_with_attributes() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_child")]
    struct TestValueChildWithAttr {
        #[xml(value)]
        pub value: String,
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestValueObjectWithAttr {
        pub id: u32,
        pub description: String,
        #[xml(tree)]
        pub child: TestValueChildWithAttr,
        #[xml(value)]
        pub value_a: String,
        #[xml(value)]
        pub value_b: String,
    }

    let xml = r#"<test_object><id>2</id><description>Test Description</description><test_child attr="Attr Value">Other value</test_child>Something</test_object>"#;
    
    let obj: TestValueObjectWithAttr = from_xml(&xml)?;
    assert_eq!(obj.id, 2);
    assert_eq!(obj.description, "Test Description");
    assert_eq!(obj.value_a, "Something");
    assert_eq!(obj.value_a, obj.value_b);
    assert_eq!(obj.child.value, "Other value");
    
    Ok(())
}

#[test]
fn test_value_handling_multiple_values() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_child")]
    struct TestValueChildMultiple {
        #[xml(value)]
        pub value: String,
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestValueObjectMultiple {
        pub id: u32,
        pub name: String,
        #[xml(tree)]
        pub child: TestValueChildMultiple,
        #[xml(value)]
        pub value_a: String,
        #[xml(value)]
        pub value_b: String,
        #[xml(value)]
        pub value_c: String,
    }

    let xml = r#"<test_object><id>3</id><name>Multiple Values</name><test_child>Child Content</test_child>Value AValue BValue C</test_object>"#;
    
    let obj: TestValueObjectMultiple = from_xml(&xml)?;
    assert_eq!(obj.id, 3);
    assert_eq!(obj.name, "Multiple Values");
    assert_eq!(obj.child.value, "Child Content");
    assert_eq!(obj.value_a, "Value A");
    assert_eq!(obj.value_b, "Value B");
    assert_eq!(obj.value_c, "Value C");
    
    Ok(())
}
