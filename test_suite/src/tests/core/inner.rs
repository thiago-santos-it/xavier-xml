use xavier::{from_obj, from_xml, XmlSerializable, XmlDeserializable, PError};

#[test]
fn test_inner_handling_vec_roundtrip() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestInnerHandlingVec {
        pub id: u32,
        pub name: String,
        #[xml(inner="item")]
        pub items: Vec<String>,
    }

    let test_data = TestInnerHandlingVec {
        id: 1,
        name: "Test Collection".to_string(),
        items: vec!["item1".to_string(), "item2".to_string(), "item3".to_string()],
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestInnerHandlingVec>"));
    assert!(xml.contains("</TestInnerHandlingVec>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Collection</name>"));
    assert!(xml.contains("<items>"));
    assert!(xml.contains("</items>"));
    assert!(xml.contains("<item>item1</item>"));
    assert!(xml.contains("<item>item2</item>"));
    assert!(xml.contains("<item>item3</item>"));

    let parsed: TestInnerHandlingVec = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_inner_handling_option_vec_some() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestInnerHandlingOptionVec {
        pub id: u32,
        pub name: String,
        #[xml(inner="inner_item")]
        pub items: Option<Vec<String>>,
    }

    let test_data = TestInnerHandlingOptionVec {
        id: 2,
        name: "Test Option Collection".to_string(),
        items: Some(vec!["option1".to_string(), "option2".to_string()]),
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestInnerHandlingOptionVec>"));
    assert!(xml.contains("</TestInnerHandlingOptionVec>"));
    assert!(xml.contains("<id>2</id>"));
    assert!(xml.contains("<name>Test Option Collection</name>"));
    assert!(xml.contains("<items>"));
    assert!(xml.contains("</items>"));
    assert!(xml.contains("<inner_item>option1</inner_item>"));
    assert!(xml.contains("<inner_item>option2</inner_item>"));
    
    let parsed: TestInnerHandlingOptionVec = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_inner_handling_option_vec_none() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestInnerHandlingOptionVec {
        pub id: u32,
        pub name: String,
        #[xml(inner="inner_item")]
        pub items: Option<Vec<String>>,
    }

    let test_data = TestInnerHandlingOptionVec {
        id: 3,
        name: "Test Option Collection None".to_string(),
        items: None,
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestInnerHandlingOptionVec>"));
    assert!(xml.contains("</TestInnerHandlingOptionVec>"));
    assert!(xml.contains("<id>3</id>"));
    assert!(xml.contains("<name>Test Option Collection None</name>"));
    assert!(!xml.contains("<items>"));
    
    let parsed: TestInnerHandlingOptionVec = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_inner_handling_empty_vec() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestInnerHandlingEmptyVec {
        pub id: u32,
        pub name: String,
        #[xml(inner="item")]
        pub items: Vec<String>,
    }

    let test_data = TestInnerHandlingEmptyVec {
        id: 4,
        name: "Test Empty Collection".to_string(),
        items: vec![],
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestInnerHandlingEmptyVec>"));
    assert!(xml.contains("</TestInnerHandlingEmptyVec>"));
    assert!(xml.contains("<id>4</id>"));
    assert!(xml.contains("<name>Test Empty Collection</name>"));
    assert!(xml.contains("<items>"));
    assert!(xml.contains("</items>"));
    
    let parsed: TestInnerHandlingEmptyVec = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_inner_handling_option_empty_vec() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestInnerHandlingOptionEmptyVec {
        pub id: u32,
        pub name: String,
        #[xml(inner="inner_item")]
        pub items: Option<Vec<String>>,
    }

    let test_data = TestInnerHandlingOptionEmptyVec {
        id: 5,
        name: "Test Option Empty Collection".to_string(),
        items: Some(vec![]),
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestInnerHandlingOptionEmptyVec>"));
    assert!(xml.contains("</TestInnerHandlingOptionEmptyVec>"));
    assert!(xml.contains("<id>5</id>"));
    assert!(xml.contains("<name>Test Option Empty Collection</name>"));
    assert!(xml.contains("<items>"));
    assert!(xml.contains("</items>"));
    
    let parsed: TestInnerHandlingOptionEmptyVec = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_inner_handling_complex_structures() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestInnerHandlingChild {
        pub id: u32,
        pub name: String,
        pub value: String,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestInnerHandlingComplex {
        pub id: u32,
        pub name: String,
        #[xml(inner="child")]
        pub children: Vec<TestInnerHandlingChild>,
    }

    let test_data = TestInnerHandlingComplex {
        id: 1,
        name: "Complex Test".to_string(),
        children: vec![
            TestInnerHandlingChild {
                id: 1,
                name: "Child 1".to_string(),
                value: "Value 1".to_string(),
            },
            TestInnerHandlingChild {
                id: 2,
                name: "Child 2".to_string(),
                value: "Value 2".to_string(),
            },
        ],
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestInnerHandlingComplex>"));
    assert!(xml.contains("</TestInnerHandlingComplex>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Complex Test</name>"));
    assert!(xml.contains("<children>"));
    assert!(xml.contains("</children>"));
    assert!(xml.contains("<child>"));
    assert!(xml.contains("</child>"));
    
    let parsed: TestInnerHandlingComplex = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_inner_handling_different_types() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestInnerHandlingDifferentTypes {
        pub id: u32,
        pub name: String,
        #[xml(inner="string_item")]
        pub strings: Vec<String>,
        #[xml(inner="number_item")]
        pub numbers: Vec<i32>,
        #[xml(inner="bool_item")]
        pub booleans: Vec<bool>,
    }

    let test_data = TestInnerHandlingDifferentTypes {
        id: 1,
        name: "Different Types Test".to_string(),
        strings: vec!["string1".to_string(), "string2".to_string()],
        numbers: vec![1, 2, 3],
        booleans: vec![true, false, true],
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestInnerHandlingDifferentTypes>"));
    assert!(xml.contains("</TestInnerHandlingDifferentTypes>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Different Types Test</name>"));
    assert!(xml.contains("<strings>"));
    assert!(xml.contains("<numbers>"));
    assert!(xml.contains("<booleans>"));
    assert!(xml.contains("<string_item>string1</string_item>"));
    assert!(xml.contains("<number_item>1</number_item>"));
    assert!(xml.contains("<bool_item>true</bool_item>"));
    
    let parsed: TestInnerHandlingDifferentTypes = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}


#[test]
fn test_collection_inner_roundtrip() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCollectionInner {
        pub id: u32,
        pub name: String,
        #[xml(inner="item")]
        pub items: Vec<String>,
    }

    let test_data = TestCollectionInner {
        id: 1,
        name: "Test Collection".to_string(),
        items: vec!["item1".to_string(), "item2".to_string(), "item3".to_string()],
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestCollectionInner>"));
    assert!(xml.contains("</TestCollectionInner>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Collection</name>"));

    assert!(xml.contains("<items>"));
    assert!(xml.contains("</items>"));
    assert!(xml.contains("<item>item1</item>"));
    assert!(xml.contains("<item>item2</item>"));
    assert!(xml.contains("<item>item3</item>"));

    let parsed: TestCollectionInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

    Ok(())
}

#[test]
fn test_collection_option_inner_with_data() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCollectionOptionInner {
        pub id: u32,
        pub name: String,
        #[xml(inner="item")]
        pub items: Option<Vec<String>>,
    }

    let test_data = TestCollectionOptionInner {
        id: 2,
        name: "Test Option Collection".to_string(),
        items: Some(vec!["option1".to_string(), "option2".to_string()]),
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestCollectionOptionInner>"));
    assert!(xml.contains("</TestCollectionOptionInner>"));
    assert!(xml.contains("<id>2</id>"));
    assert!(xml.contains("<name>Test Option Collection</name>"));

    assert!(xml.contains("<items>"));
    assert!(xml.contains("</items>"));
    assert!(xml.contains("<item>option1</item>"));
    assert!(xml.contains("<item>option2</item>"));

    let parsed: TestCollectionOptionInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

    Ok(())
}

#[test]
fn test_collection_option_inner_empty() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCollectionOptionInner {
        pub id: u32,
        pub name: String,
        #[xml(inner="item")]
        pub items: Option<Vec<String>>,
    }

    let test_data = TestCollectionOptionInner {
        id: 3,
        name: "Test Option Collection None".to_string(),
        items: None,
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestCollectionOptionInner>"));
    assert!(xml.contains("</TestCollectionOptionInner>"));
    assert!(xml.contains("<id>3</id>"));
    assert!(xml.contains("<name>Test Option Collection None</name>"));

    assert!(!xml.contains("<items>"));
    assert!(!xml.contains("</items>"));
    assert!(!xml.contains("<item>"));

    let parsed: TestCollectionOptionInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

    Ok(())
}

#[test]
fn test_collection_complex_inner_roundtrip() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    #[xml(name="child_item")]
    struct TestChildItem {
        pub value: String,
        pub count: i32,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCollectionComplexInner {
        pub id: u32,
        pub name: String,
        pub children: Vec<TestChildItem>,
    }

    let test_data = TestCollectionComplexInner {
        id: 4,
        name: "Test Complex Collection".to_string(),
        children: vec![
            TestChildItem { value: "child1".to_string(), count: 10 },
            TestChildItem { value: "child2".to_string(), count: 20 },
        ],
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestCollectionComplexInner>"));
    assert!(xml.contains("</TestCollectionComplexInner>"));
    assert!(xml.contains("<id>4</id>"));
    assert!(xml.contains("<name>Test Complex Collection</name>"));

    assert!(xml.contains("<children>"));
    assert!(xml.contains("</children>"));
    assert!(xml.contains("<child_item>"));
    assert!(xml.contains("</child_item>"));
    assert!(xml.contains("<value>child1</value>"));
    assert!(xml.contains("<count>10</count>"));
    assert!(xml.contains("<value>child2</value>"));
    assert!(xml.contains("<count>20</count>"));

    let parsed: TestCollectionComplexInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

    Ok(())
}

#[test]
fn test_collection_complex_option_inner_with_data() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    #[xml(name="child_item")]
    struct TestChildItem {
        pub value: String,
        pub count: i32,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCollectionComplexOptionInner {
        pub id: u32,
        pub name: String,
        pub children: Option<Vec<TestChildItem>>,
    }

    let test_data = TestCollectionComplexOptionInner {
        id: 5,
        name: "Test Option Complex Collection".to_string(),
        children: Some(vec![
            TestChildItem { value: "complex1".to_string(), count: 100 },
            TestChildItem { value: "complex2".to_string(), count: 200 },
        ]),
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestCollectionComplexOptionInner>"));
    assert!(xml.contains("</TestCollectionComplexOptionInner>"));
    assert!(xml.contains("<id>5</id>"));
    assert!(xml.contains("<name>Test Option Complex Collection</name>"));

    assert!(xml.contains("<children>"));
    assert!(xml.contains("</children>"));
    assert!(xml.contains("<child_item>"));
    assert!(xml.contains("</child_item>"));
    assert!(xml.contains("<value>complex1</value>"));
    assert!(xml.contains("<count>100</count>"));
    assert!(xml.contains("<value>complex2</value>"));
    assert!(xml.contains("<count>200</count>"));

    let parsed: TestCollectionComplexOptionInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

    Ok(())
}

#[test]
fn test_collection_complex_option_inner_empty() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    #[xml(name="child_item")]
    struct TestChildItem {
        pub value: String,
        pub count: i32,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCollectionComplexOptionInner {
        pub id: u32,
        pub name: String,
        pub children: Option<Vec<TestChildItem>>,
    }

    let test_data = TestCollectionComplexOptionInner {
        id: 6,
        name: "Test Option Complex Collection None".to_string(),
        children: None,
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestCollectionComplexOptionInner>"));
    assert!(xml.contains("</TestCollectionComplexOptionInner>"));
    assert!(xml.contains("<id>6</id>"));
    assert!(xml.contains("<name>Test Option Complex Collection None</name>"));

    assert!(!xml.contains("<children>"));
    assert!(!xml.contains("</children>"));
    assert!(!xml.contains("<child_item>"));

    let parsed: TestCollectionComplexOptionInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

    Ok(())
}

#[test]
fn test_collection_inner_empty() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCollectionInner {
        pub id: u32,
        pub name: String,
        #[xml(inner="item")]
        pub items: Vec<String>,
    }

    let test_data = TestCollectionInner {
        id: 7,
        name: "Test Empty Collection".to_string(),
        items: vec![],
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestCollectionInner>"));
    assert!(xml.contains("</TestCollectionInner>"));
    assert!(xml.contains("<id>7</id>"));
    assert!(xml.contains("<name>Test Empty Collection</name>"));

    assert!(xml.contains("<items>"));
    assert!(xml.contains("</items>"));
    assert!(!xml.contains("<item>"));

    let parsed: TestCollectionInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

    Ok(())
}

#[test]
fn test_collection_option_inner_empty_collection() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCollectionOptionInner {
        pub id: u32,
        pub name: String,
        #[xml(inner="item")]
        pub items: Option<Vec<String>>,
    }

    let test_data = TestCollectionOptionInner {
        id: 8,
        name: "Test Empty Option Collection".to_string(),
        items: Some(vec![]),
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestCollectionOptionInner>"));
    assert!(xml.contains("</TestCollectionOptionInner>"));
    assert!(xml.contains("<id>8</id>"));
    assert!(xml.contains("<name>Test Empty Option Collection</name>"));

    assert!(xml.contains("<items>"));
    assert!(xml.contains("</items>"));
    assert!(!xml.contains("<item>"));

    let parsed: TestCollectionOptionInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

    Ok(())
}



#[test]
fn test_collection_inner_nested_structures() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    #[xml(name="nested_item")]
    struct TestNestedItem {
        pub value: String,
        pub metadata: TestMetadata,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestMetadata {
        pub created: String,
        pub updated: String,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCollectionNestedInner {
        pub id: u32,
        pub name: String,
        pub items: Vec<TestNestedItem>,
    }

    let test_data = TestCollectionNestedInner {
        id: 10,
        name: "Test Nested Collection".to_string(),
        items: vec![
            TestNestedItem {
                value: "nested1".to_string(),
                metadata: TestMetadata {
                    created: "2024-01-01".to_string(),
                    updated: "2024-01-02".to_string(),
                },
            },
            TestNestedItem {
                value: "nested2".to_string(),
                metadata: TestMetadata {
                    created: "2024-01-03".to_string(),
                    updated: "2024-01-04".to_string(),
                },
            },
        ],
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestCollectionNestedInner>"));
    assert!(xml.contains("</TestCollectionNestedInner>"));
    assert!(xml.contains("<id>10</id>"));
    assert!(xml.contains("<name>Test Nested Collection</name>"));

    assert!(xml.contains("<items>"));
    assert!(xml.contains("</items>"));
    assert!(xml.contains("<nested_item>"));
    assert!(xml.contains("</nested_item>"));
    assert!(xml.contains("<value>nested1</value>"));
    assert!(xml.contains("<metadata>"));
    assert!(xml.contains("<created>2024-01-01</created>"));
    assert!(xml.contains("<updated>2024-01-02</updated>"));
    assert!(xml.contains("<value>nested2</value>"));
    assert!(xml.contains("<created>2024-01-03</created>"));
    assert!(xml.contains("<updated>2024-01-04</updated>"));

    let parsed: TestCollectionNestedInner = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

    Ok(())
} 