use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};

#[test]
fn test_collections_basic_roundtrip() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCollectionsChild {
        pub id: u32,
        pub name: String,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCollectionsBasic {
        pub id: u32,
        pub name: String,
        pub children: Vec<TestCollectionsChild>,
    }

    let test_data = TestCollectionsBasic {
        id: 1,
        name: "Test Collections".to_string(),
        children: vec![
            TestCollectionsChild { id: 1, name: "Child 1".to_string() },
            TestCollectionsChild { id: 2, name: "Child 2".to_string() },
            TestCollectionsChild { id: 3, name: "Child 3".to_string() },
        ],
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestCollectionsBasic>"));
    assert!(xml.contains("</TestCollectionsBasic>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Collections</name>"));
    assert!(xml.contains("<children>"));
    assert!(xml.contains("</children>"));
    assert!(xml.contains("<TestCollectionsChild>"));
    assert!(xml.contains("</TestCollectionsChild>"));
    
    let parsed: TestCollectionsBasic = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_collections_recursive_structures() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCollectionsRecursiveChild {
        #[xml(attribute, name="attr")]
        pub attribute: String,
        pub id: u32,
        pub name: String,
        #[xml(tree)]
        pub inner: Option<Box<TestCollectionsRecursiveChild>>,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCollectionsRecursive {
        pub id: u32,
        pub name: String,
        #[xml(tree)]
        pub child: TestCollectionsRecursiveChild,
    }

    let test_data = TestCollectionsRecursive {
        id: 1,
        name: "Recursive Test".to_string(),
        child: TestCollectionsRecursiveChild {
            attribute: "root".to_string(),
            id: 1,
            name: "Root Child".to_string(),
            inner: Some(Box::new(TestCollectionsRecursiveChild {
                attribute: "nested".to_string(),
                id: 2,
                name: "Nested Child".to_string(),
                inner: None,
            })),
        },
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestCollectionsRecursive>"));
    assert!(xml.contains("</TestCollectionsRecursive>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Recursive Test</name>"));
    assert!(xml.contains("attr=\"root\""));
    assert!(xml.contains("attr=\"nested\""));
    
    let parsed: TestCollectionsRecursive = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    assert_eq!(test_data.child.attribute, parsed.child.attribute);
    assert_eq!(test_data.child.id, parsed.child.id);
    assert_eq!(test_data.child.name, parsed.child.name);
    
    Ok(())
}

#[test]
fn test_collections_tree_structures() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    #[xml(name="inner")]
    struct TestCollectionsChildA {
        #[xml(attribute, name="attr")]
        pub attribute: String,
        pub id: u32,
        pub name: String,
        #[xml(tree)]
        pub inner: TestCollectionsChildB,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    #[xml(name="inner")]
    struct TestCollectionsChildB {
        #[xml(attribute, name="attr")]
        pub attribute: String,
        pub id: u32,
        pub name: String,
        #[xml(tree)]
        pub inner: TestCollectionsChildC,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    #[xml(name="inner")]
    struct TestCollectionsChildC {
        #[xml(attribute, name="attr")]
        pub attribute: String,
        pub id: u32,
        pub name: String,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    #[xml(name="child")]
    struct TestCollectionsChildRoot {
        #[xml(tree)]
        pub inner: TestCollectionsChildA,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCollectionsTree {
        pub id: u32,
        pub name: String,
        #[xml(tree)]
        pub child: TestCollectionsChildRoot,
    }

    let test_data = TestCollectionsTree {
        id: 1,
        name: "Tree Test".to_string(),
        child: TestCollectionsChildRoot {
            inner: TestCollectionsChildA {
                attribute: "level1".to_string(),
                id: 1,
                name: "Level 1".to_string(),
                inner: TestCollectionsChildB {
                    attribute: "level2".to_string(),
                    id: 2,
                    name: "Level 2".to_string(),
                    inner: TestCollectionsChildC {
                        attribute: "level3".to_string(),
                        id: 3,
                        name: "Level 3".to_string(),
                    },
                },
            },
        },
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestCollectionsTree>"));
    assert!(xml.contains("</TestCollectionsTree>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Tree Test</name>"));
    assert!(xml.contains("attr=\"level1\""));
    assert!(xml.contains("attr=\"level2\""));
    assert!(xml.contains("attr=\"level3\""));
    
    let parsed: TestCollectionsTree = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    
    Ok(())
}

#[test]
fn test_collections_sibling_structures() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCollectionsSibling {
        pub id: u32,
        pub name: String,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCollectionsSiblings {
        pub id: u32,
        pub name: String,
        #[xml(flatten)]
        pub sibling: Vec<TestCollectionsSibling>
    }

    let test_data = TestCollectionsSiblings {
        id: 1,
        name: "Siblings Test".to_string(),
        sibling: vec![
            TestCollectionsSibling { id: 1, name: "Sibling 1".to_string() },
            TestCollectionsSibling { id: 2, name: "Sibling 2".to_string() },
            TestCollectionsSibling { id: 3, name: "Sibling 3".to_string() },
        ],
    };

    let xml = from_obj(&test_data);
    println!("{:#?}", xml);
    assert!(xml.contains("<TestCollectionsSiblings>"));
    assert!(xml.contains("</TestCollectionsSiblings>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Siblings Test</name>"));
    assert!(xml.contains("<sibling>"));
    assert!(xml.contains("</sibling>"));
    
    let parsed: TestCollectionsSiblings = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    assert_eq!(test_data.sibling.len(), parsed.sibling.len());
    
    Ok(())
}

#[test]
fn test_collections_empty_collections() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCollectionsChild {
        pub id: u32,
        pub name: String,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCollectionsEmpty {
        pub id: u32,
        pub name: String,
        pub children: Vec<TestCollectionsChild>,
    }

    let test_data = TestCollectionsEmpty {
        id: 1,
        name: "Empty Collections".to_string(),
        children: vec![],
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestCollectionsEmpty>"));
    assert!(xml.contains("</TestCollectionsEmpty>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Empty Collections</name>"));
    assert!(xml.contains("<children>"));
    assert!(xml.contains("</children>"));
    
    let parsed: TestCollectionsEmpty = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
} 