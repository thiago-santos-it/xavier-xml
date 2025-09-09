use xavier::{from_xml, from_obj, PError, XmlDeserializable, XmlSerializable};

#[test]
fn test_value_handling_basic_roundtrip() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_child")]
    struct TestValueChild {
        pub value: String,
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestValueObject {
        pub id: u32,
        pub name: String,
        #[xml(tree)]
        pub child: TestValueChild,
        pub value_a: String,
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

    assert!(xml.contains("<testObject>"));
    assert!(xml.contains("</testObject>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Value Object</name>"));
    assert!(xml.contains("<child>"));
    assert!(xml.contains("</child>"));
    
    let parsed: TestValueObject = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_value_handling_multiple_values() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_child")]
    struct TestValueChildMultiple {
        pub value: String,
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestValueObjectMultiple {
        pub id: u32,
        pub name: String,
        #[xml(tree)]
        pub child: TestValueChildMultiple,
        pub value_a: String,
        pub value_b: String,
        pub value_c: String,
    }

    let xml = r#"<testObject><id>3</id><name>Multiple Values</name><child><value>Child Content</value></child><valueA>Value A</valueA><valueB>Value B</valueB><valueC>Value C</valueC></testObject>"#;
    
    let obj: TestValueObjectMultiple = from_xml(&xml)?;

    assert_eq!(obj.id, 3);
    assert_eq!(obj.name, "Multiple Values");
    assert_eq!(obj.child.value, "Child Content");
    assert_eq!(obj.value_a, "Value A");
    assert_eq!(obj.value_b, "Value B");
    assert_eq!(obj.value_c, "Value C");
    
    Ok(())
}
