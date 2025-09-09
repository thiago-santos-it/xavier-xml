use xavier::{from_xml, from_obj, PError, XmlDeserializable, XmlSerializable};

#[test]
fn test_self_closing_tags_basic_roundtrip() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object")]
    struct TestSelfClosingBasic {
        #[xml(attribute, name="test_string")]
        pub other_string: String,
        #[xml(attribute)]
        pub test_int: i32,
    }

    let test_data = TestSelfClosingBasic {
        other_string: "Test text".to_string(),
        test_int: 42,
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<test_object"));
    assert!(xml.contains("test_string=\"Test text\""));
    assert!(xml.contains("test_int=\"42\""));
    
    let parsed: TestSelfClosingBasic = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_self_closing_tags_with_attributes() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object")]
    struct TestSelfClosingWithAttr {
        #[xml(attribute, name="some_string")]
        pub other_string: String,
        #[xml(attribute)]
        pub some_int: i32,
    }

    let xml = r#"<test_object some_string="Some text" some_int="11"/>"#;
    let obj: TestSelfClosingWithAttr = from_xml(&xml)?;
    assert_eq!(obj.other_string, "Some text");
    assert_eq!(obj.some_int, 11);
    
    Ok(())
}

#[test]
fn test_self_closing_tags_child_roundtrip() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_child")]
    struct TestSelfClosingChildX {
        #[xml(attribute, name="attr")]
        pub attribute: String,
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestSelfClosingObjectX {
        pub id: u32,
        pub name: String,
        #[xml(tree)]
        pub child: TestSelfClosingChildX,
    }

    let test_data = TestSelfClosingObjectX {
        id: 1,
        name: "Test Object".to_string(),
        child: TestSelfClosingChildX {
            attribute: "Some text".to_string(),
        },
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<testObject>"));
    assert!(xml.contains("</testObject>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Object</name>"));
    assert!(xml.contains("<child"));
    assert!(xml.contains("attr=\"Some text\""));
    
    let parsed: TestSelfClosingObjectX = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_self_closing_tags_child_with_attributes() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_child")]
    struct TestSelfClosingChildWithAttr {
        #[xml(attribute, name="attr")]
        pub attribute: String,
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestSelfClosingObjectChildWithAttr {
        #[xml(tree)]
        pub child: TestSelfClosingChildWithAttr,
    }

    let xml = r#"<testObject><child attr="Some text"/></testObject>"#;
    let obj: TestSelfClosingObjectChildWithAttr = from_xml(&xml)?;
    assert_eq!(obj.child.attribute, "Some text");
    
    Ok(())
}

#[test]
fn test_self_closing_tags_empty_child() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_child")]
    struct TestSelfClosingChildEmpty;

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object")]
    struct TestSelfClosingObjectEmptyChild {
        pub id: u32,
        pub child: Option<TestSelfClosingChildEmpty>,
    }

    let xml = r#"<test_object><id>1</id><child/></test_object>"#;
    let obj: TestSelfClosingObjectEmptyChild = from_xml(&xml)?;
    assert_eq!(obj.id, 1);
    assert_eq!(obj.child.is_some(), true);

    let xml = r#"<test_object><id>2</id></test_object>"#;
    let obj: TestSelfClosingObjectEmptyChild = from_xml(&xml)?;
    assert_eq!(obj.id, 2);
    assert_eq!(obj.child.is_none(), true);
    
    Ok(())
}