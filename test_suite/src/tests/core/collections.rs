use xavier::{from_xml, from_obj, PError, XmlDeserializable, XmlSerializable};

#[test]
fn test_collection_handling_basic_roundtrip() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_child")]
    struct TestCollectionChild {
        pub id: u32,
        pub name: String,
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestCollectionObject {
        pub id: u32,
        pub name: String,
        pub children: Vec<TestCollectionChild>,
    }

    let test_data = TestCollectionObject {
        id: 1,
        name: "Test Collection Object".to_string(),
        children: vec![
            TestCollectionChild { id: 1, name: "Child A".to_string() },
            TestCollectionChild { id: 2, name: "Child B".to_string() },
        ],
    };

    let xml = from_obj(&test_data);
    println!("{:#?}", xml);
    assert!(xml.contains("<testObject>"));
    assert!(xml.contains("</testObject>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Collection Object</name>"));
    assert!(xml.contains("<children>"));
    assert!(xml.contains("</children>"));
    assert!(xml.contains("<test_child>"));
    assert!(xml.contains("</test_child>"));
    
    let parsed: TestCollectionObject = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_collection_handling_with_data() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_child")]
    struct TestCollectionChild {
        pub id: u32,
        pub name: String
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestCollectionObject {
        pub id: u32,
        pub name: String,
        pub children: Vec<TestCollectionChild>
    }

    let xml = r#"
    <testObject>
        <id>1</id>
        <name>Test Name</name>
        <children>
            <test_child>
                <id>1</id>
                <name>Child A</name>
            </test_child>
            <test_child>
                <id>2</id>
                <name>Child B</name>
            </test_child>
        </children>
    </testObject>"#;

    let obj: TestCollectionObject = from_xml(&xml)?;
    assert_eq!(obj.id, 1);
    assert_eq!(obj.name, "Test Name");
    assert_eq!(obj.children[0].id, 1);
    assert_eq!(obj.children[0].name, "Child A");
    assert_eq!(obj.children[1].id, 2);
    assert_eq!(obj.children[1].name, "Child B");
    
    Ok(())
}

#[test]
fn test_collection_handling_option_roundtrip() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_child")]
    struct TestCollectionChild {
        pub id: u32,
        pub name: String,
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestCollectionObjectOption {
        pub id: u32,
        pub name: String,
        pub children: Option<Vec<TestCollectionChild>>,
    }

    let test_data = TestCollectionObjectOption {
        id: 1,
        name: "Test Option Collection".to_string(),
        children: Some(vec![
            TestCollectionChild { id: 1, name: "Child A".to_string() },
            TestCollectionChild { id: 2, name: "Child B".to_string() },
        ]),
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<testObject>"));
    assert!(xml.contains("</testObject>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Option Collection</name>"));
    assert!(xml.contains("<children>"));
    assert!(xml.contains("</children>"));
    
    let parsed: TestCollectionObjectOption = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_collection_handling_option_with_data() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_child")]
    struct TestCollectionChild {
        pub id: u32,
        pub name: String,
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestCollectionObjectOption {
        pub id: u32,
        pub name: String,
        pub children: Option<Vec<TestCollectionChild>>,
    }

    let xml = r#"
    <testObject>
        <id>1</id>
        <name>Test Name</name>
        <children>
            <test_child>
                <id>1</id>
                <name>Child A</name>
            </test_child>
            <test_child>
                <id>2</id>
                <name>Child B</name>
            </test_child>
        </children>
    </testObject>"#;

    let obj: TestCollectionObjectOption = from_xml(&xml)?;
    assert_eq!(obj.id, 1);
    assert_eq!(obj.name, "Test Name");
    let children = obj.children.unwrap();
    assert_eq!(children[0].id, 1);
    assert_eq!(children[0].name, "Child A");
    assert_eq!(children[1].id, 2);
    assert_eq!(children[1].name, "Child B");
    
    Ok(())
}

#[test]
fn test_collection_handling_empty_collection() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_child")]
    struct TestCollectionChild {
        pub id: u32,
        pub name: String,
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestCollectionObject {
        pub id: u32,
        pub name: String,
        pub children: Vec<TestCollectionChild>,
    }

    let xml = r#"
    <testObject>
        <id>1</id>
        <name>Test Name</name>
        <children>
        </children>
    </testObject>"#;

    let obj: TestCollectionObject = from_xml(&xml)?;
    assert_eq!(obj.id, 1);
    assert_eq!(obj.name, "Test Name");
    assert_eq!(obj.children.len(), 0);
    
    Ok(())
}

#[test]
fn test_collection_handling_option_empty_collection() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_child")]
    struct TestCollectionChild {
        pub id: u32,
        pub name: String,
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestCollectionObjectOption {
        pub id: u32,
        pub name: String,
        pub children: Option<Vec<TestCollectionChild>>,
    }

    let xml = r#"
    <testObject>
        <id>1</id>
        <name>Test Name</name>
        <children>
        </children>
    </testObject>"#;

    let obj: TestCollectionObjectOption = from_xml(&xml)?;
    assert_eq!(obj.id, 1);
    assert_eq!(obj.name, "Test Name");
    let children = obj.children.unwrap();
    assert_eq!(children.len(), 0);
    
    Ok(())
}

#[test]
fn test_collection_handling_option_missing_collection() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_child")]
    struct TestCollectionChild {
        pub id: u32,
        pub name: String,
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestCollectionObjectOption {
        pub id: u32,
        pub name: String,
        pub children: Option<Vec<TestCollectionChild>>,
    }

    let xml = r#"
    <testObject>
        <id>1</id>
        <name>Test Name</name>
    </testObject>"#;

    let obj: TestCollectionObjectOption = from_xml(&xml)?;
    assert_eq!(obj.id, 1);
    assert_eq!(obj.name, "Test Name");
    assert_eq!(obj.children, None);
    
    Ok(())
}