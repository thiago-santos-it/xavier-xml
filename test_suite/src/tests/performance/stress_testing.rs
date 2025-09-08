use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};
use std::time::Instant;

#[test]
fn test_performance_stress_deep_nesting() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceStressDeepNested {
        pub id: u32,
        pub level1: TestPerformanceStressLevel1,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceStressLevel1 {
        pub name: String,
        pub level2: TestPerformanceStressLevel2,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceStressLevel2 {
        pub value: i32,
        pub level3: TestPerformanceStressLevel3,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceStressLevel3 {
        pub data: String,
        pub level4: TestPerformanceStressLevel4,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceStressLevel4 {
        pub flag: bool,
        pub level5: TestPerformanceStressLevel5,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceStressLevel5 {
        pub number: f64,
        pub level6: TestPerformanceStressLevel6,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceStressLevel6 {
        pub text: String,
        pub level7: TestPerformanceStressLevel7,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceStressLevel7 {
        pub count: u64,
        pub level8: TestPerformanceStressLevel8,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceStressLevel8 {
        #[xml(inner="item")]
        pub items: Vec<String>,
        pub level9: TestPerformanceStressLevel9,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceStressLevel9 {
        pub metadata: String,
        pub level10: TestPerformanceStressLevel10,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceStressLevel10 {
        pub final_value: String,
    }

    let data = TestPerformanceStressDeepNested {
        id: 1,
        level1: TestPerformanceStressLevel1 {
            name: "Level 1".to_string(),
            level2: TestPerformanceStressLevel2 {
                value: 42,
                level3: TestPerformanceStressLevel3 {
                    data: "Level 3 Data".to_string(),
                    level4: TestPerformanceStressLevel4 {
                        flag: true,
                        level5: TestPerformanceStressLevel5 {
                            number: 3.14159,
                            level6: TestPerformanceStressLevel6 {
                                text: "Level 6 Text".to_string(),
                                level7: TestPerformanceStressLevel7 {
                                    count: 1000,
                                    level8: TestPerformanceStressLevel8 {
                                        items: vec!["item1".to_string(), "item2".to_string()],
                                        level9: TestPerformanceStressLevel9 {
                                            metadata: "Level 9 Metadata".to_string(),
                                            level10: TestPerformanceStressLevel10 {
                                                final_value: "Final Value".to_string(),
                                            },
                                        },
                                    },
                                },
                            },
                        },
                    },
                },
            },
        },
    };
    
    let start = Instant::now();
    let xml = from_obj(&data);
    let serialization_duration = start.elapsed();
    
    let start = Instant::now();
    let parsed: TestPerformanceStressDeepNested = from_xml(&xml)?;
    let deserialization_duration = start.elapsed();
    
    assert!(serialization_duration < std::time::Duration::from_secs(1));
    assert!(deserialization_duration < std::time::Duration::from_secs(1));
    assert_eq!(data, parsed);
    
    Ok(())
}

#[test]
fn test_performance_stress_large_collections() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceStressLargeCollection {
        pub id: u32,
        #[xml(inner="item")]
        pub items: Vec<TestPerformanceStressLargeItem>,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceStressLargeItem {
        pub item_id: u64,
        pub name: String,
        pub description: String,
        #[xml(inner="item")]
        pub tags: Vec<String>,
        pub metadata: Vec<TestPerformanceStressKeyValue>,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceStressKeyValue {
        pub key: String,
        pub value: String,
    }

    let mut items = Vec::with_capacity(1000);
    for i in 0..1000 {
        items.push(TestPerformanceStressLargeItem {
            item_id: i as u64,
            name: format!("Item {}", i),
            description: format!("Description for item {}", i),
            tags: vec![format!("tag{}", i), format!("category{}", i % 10)],
            metadata: vec![
                TestPerformanceStressKeyValue {
                    key: format!("key{}", i),
                    value: format!("value{}", i),
                },
                TestPerformanceStressKeyValue {
                    key: format!("meta{}", i),
                    value: format!("meta_value{}", i),
                },
            ],
        });
    }
    
    let data = TestPerformanceStressLargeCollection {
        id: 1,
        items,
    };
    
    let start = Instant::now();
    let xml = from_obj(&data);
    let serialization_duration = start.elapsed();
    
    let start = Instant::now();
    let parsed: TestPerformanceStressLargeCollection = from_xml(&xml)?;
    let deserialization_duration = start.elapsed();
    
    assert!(serialization_duration < std::time::Duration::from_secs(5));
    assert!(deserialization_duration < std::time::Duration::from_secs(5));
    assert_eq!(data.id, parsed.id);
    assert_eq!(data.items.len(), parsed.items.len());
    
    Ok(())
}

#[test]
fn test_performance_stress_memory_intensive() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceStressMemoryIntensive {
        pub id: u32,
        pub large_string: String,
        #[xml(inner="item")]
        pub large_array: Vec<u8>,
        pub nested_data: Vec<TestPerformanceStressMemoryIntensive>,
    }

    let large_string = "A".repeat(10000);
    let large_array: Vec<u8> = (0..255).collect();
    
    let data = TestPerformanceStressMemoryIntensive {
        id: 1,
        large_string,
        large_array,
        nested_data: vec![], 
    };
    
    let start = Instant::now();
    let xml = from_obj(&data);
    let serialization_duration = start.elapsed();
    
    let start = Instant::now();
    let parsed: TestPerformanceStressMemoryIntensive = from_xml(&xml)?;
    let deserialization_duration = start.elapsed();
    
    assert!(serialization_duration < std::time::Duration::from_secs(3));
    assert!(deserialization_duration < std::time::Duration::from_secs(3));
    assert_eq!(data.id, parsed.id);
    assert_eq!(data.large_string, parsed.large_string);
    assert_eq!(data.large_array.len(), parsed.large_array.len());
    
    Ok(())
}

#[test]
fn test_performance_stress_mixed_types() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceStressMixedTypes {
        pub id: u32,
        pub name: String,
        pub count: i32,
        pub price: f64,
        pub active: bool,
        pub tags: Vec<String>,
        pub metadata: Option<String>,
        #[xml(tree)]
        pub nested: Option<TestPerformanceStressNested>,
        #[xml(tree)]
        pub attributes: TestPerformanceStressAttributes,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceStressNested {
        pub value: String,
        pub count: u32,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceStressAttributes {
        pub priority: u32,
        pub category: String,
        pub active: bool,
    }

    let data = TestPerformanceStressMixedTypes {
        id: 1,
        name: "Mixed Types Stress Test".to_string(),
        count: 42,
        price: 3.14159,
        active: true,
        tags: vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()],
        metadata: Some("Some metadata".to_string()),
        nested: Some(TestPerformanceStressNested {
            value: "Nested value".to_string(),
            count: 100,
        }),
        attributes: TestPerformanceStressAttributes {
            priority: 1,
            category: "Test".to_string(),
            active: true,
        },
    };
    
    let start = Instant::now();
    let xml = from_obj(&data);
    let serialization_duration = start.elapsed();
    
    let start = Instant::now();
    let parsed: TestPerformanceStressMixedTypes = from_xml(&xml)?;
    let deserialization_duration = start.elapsed();
    
    assert!(serialization_duration < std::time::Duration::from_secs(1));
    assert!(deserialization_duration < std::time::Duration::from_secs(1));
    assert_eq!(data, parsed);
    
    Ok(())
}

#[test]
fn test_performance_stress_concurrent_operations() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceStressConcurrent {
        pub id: u32,
        pub name: String,
        pub items: Vec<TestPerformanceStressConcurrentItem>,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceStressConcurrentItem {
        pub id: u32,
        pub name: String,
        pub value: f64,
    }

    let data = TestPerformanceStressConcurrent {
        id: 1,
        name: "Concurrent Test".to_string(),
        items: vec![
            TestPerformanceStressConcurrentItem {
                id: 1,
                name: "Item 1".to_string(),
                value: 1.0,
            },
            TestPerformanceStressConcurrentItem {
                id: 2,
                name: "Item 2".to_string(),
                value: 2.0,
            },
        ],
    };
    
    let start = Instant::now();
    for _ in 0..100 {
        let xml = from_obj(&data);
        let _parsed: TestPerformanceStressConcurrent = from_xml(&xml)?;
    }
    let total_duration = start.elapsed();
    
    assert!(total_duration < std::time::Duration::from_secs(10));
    
    Ok(())
} 