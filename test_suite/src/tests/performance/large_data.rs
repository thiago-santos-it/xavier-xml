use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};
use std::time::Instant;

#[test]
fn test_performance_large_data_parsing() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceLargeData {
        pub items: Vec<TestPerformanceDataItem>,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceDataItem {
        pub id: u64,
        pub content: String,
        pub tags: Vec<String>,
    }

    fn generate_large_dataset(count: usize) -> TestPerformanceLargeData {
        let mut items = Vec::with_capacity(count);
        for i in 0..count {
            items.push(TestPerformanceDataItem {
                id: i as u64,
                content: format!("Content for item {}", i),
                tags: vec![format!("tag{}", i), format!("category{}", i % 10)],
            });
        }
        TestPerformanceLargeData { items }
    }

    let large_data = generate_large_dataset(1000);
    let xml = from_obj(&large_data);
    
    let start = Instant::now();
    let parsed: TestPerformanceLargeData = from_xml(&xml)?;
    let duration = start.elapsed();
    
    assert!(duration < std::time::Duration::from_secs(5)); 
    assert_eq!(parsed.items.len(), 1000);
    
    Ok(())
}

#[test]
fn test_performance_large_data_memory_usage() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceLargeData {
        pub items: Vec<TestPerformanceDataItem>,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceDataItem {
        pub id: u64,
        pub content: String,
        pub tags: Vec<String>,
    }

    fn generate_large_dataset(count: usize) -> TestPerformanceLargeData {
        let mut items = Vec::with_capacity(count);
        for i in 0..count {
            items.push(TestPerformanceDataItem {
                id: i as u64,
                content: format!("Content for item {}", i),
                tags: vec![format!("tag{}", i), format!("category{}", i % 10)],
            });
        }
        TestPerformanceLargeData { items }
    }

    let large_data = generate_large_dataset(5000);
    let xml = from_obj(&large_data);
    
    let start = Instant::now();
    let parsed: TestPerformanceLargeData = from_xml(&xml)?;
    let duration = start.elapsed();
    
    assert!(duration < std::time::Duration::from_secs(10)); 
    assert_eq!(parsed.items.len(), 5000);
    
    Ok(())
}

#[test]
fn test_performance_large_data_serialization_speed() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceLargeData {
        pub items: Vec<TestPerformanceDataItem>,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceDataItem {
        pub id: u64,
        pub content: String,
        pub tags: Vec<String>,
    }

    fn generate_large_dataset(count: usize) -> TestPerformanceLargeData {
        let mut items = Vec::with_capacity(count);
        for i in 0..count {
            items.push(TestPerformanceDataItem {
                id: i as u64,
                content: format!("Content for item {}", i),
                tags: vec![format!("tag{}", i), format!("category{}", i % 10)],
            });
        }
        TestPerformanceLargeData { items }
    }

    let large_data = generate_large_dataset(1000);
    
    let start = Instant::now();
    let _xml = from_obj(&large_data);
    let duration = start.elapsed();
    
    assert!(duration < std::time::Duration::from_secs(2)); 
    
    Ok(())
}

#[test]
fn test_performance_large_data_complex_structures() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceLargeDataComplex {
        pub items: Vec<TestPerformanceDataItemComplex>,
        pub metadata: TestPerformanceMetadata,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceDataItemComplex {
        pub id: u64,
        pub content: String,
        pub tags: Vec<String>,
        pub attributes: TestPerformanceAttributes,
        pub nested: Option<TestPerformanceNested>,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceAttributes {
        pub priority: u32,
        pub category: String,
        pub active: bool,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceNested {
        pub value: f64,
        pub description: String,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceMetadata {
        pub count: u32,
        pub version: String,
        pub timestamp: u64,
    }

    fn generate_complex_dataset(count: usize) -> TestPerformanceLargeDataComplex {
        let mut items = Vec::with_capacity(count);
        for i in 0..count {
            items.push(TestPerformanceDataItemComplex {
                id: i as u64,
                content: format!("Complex content for item {}", i),
                tags: vec![format!("tag{}", i), format!("category{}", i % 10)],
                attributes: TestPerformanceAttributes {
                    priority: (i % 5) as u32,
                    category: format!("Category{}", i % 20),
                    active: i % 2 == 0,
                },
                nested: if i % 3 == 0 {
                    Some(TestPerformanceNested {
                        value: i as f64 * 1.5,
                        description: format!("Nested description for item {}", i),
                    })
                } else {
                    None
                },
            });
        }
        TestPerformanceLargeDataComplex {
            items,
            metadata: TestPerformanceMetadata {
                count: count as u32,
                version: "1.0.0".to_string(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
        }
    }

    let complex_data = generate_complex_dataset(500);
    let xml = from_obj(&complex_data);
    
    let start = Instant::now();
    let parsed: TestPerformanceLargeDataComplex = from_xml(&xml)?;
    let duration = start.elapsed();
    
    assert!(duration < std::time::Duration::from_secs(3)); 
    assert_eq!(parsed.items.len(), 500);
    assert_eq!(parsed.metadata.count, 500);
    
    Ok(())
}


#[test]
fn test_performance_large_collections_1000_items() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceLargeCollection {
        pub items: Vec<TestPerformanceCollectionItem>,
        pub metadata: String,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceCollectionItem {
        pub id: u64,
        pub name: String,
        pub value: f64,
    }

    fn generate_large_collection_xml(count: usize) -> String {
        let mut xml = String::new();
        xml.push_str("<TestPerformanceLargeCollection>");
        xml.push_str("<metadata>Large collection test</metadata>");
        xml.push_str("<items>");

        for i in 0..count {
            xml.push_str(&format!(
                "<TestPerformanceCollectionItem><id>{}</id><name>Item{}</name><value>{}</value></TestPerformanceCollectionItem>",
                i, i, i as f64
            ));
        }

        xml.push_str("</items>");
        xml.push_str("</TestPerformanceLargeCollection>");
        xml
    }

    let xml = generate_large_collection_xml(1000);
    let result = from_xml::<TestPerformanceLargeCollection>(&xml);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert_eq!(parsed.items.len(), 1000);
    assert_eq!(parsed.metadata, "Large collection test");

    Ok(())
}

#[test]
fn test_performance_large_collections_10000_items() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceLargeCollection {
        pub items: Vec<TestPerformanceCollectionItem>,
        pub metadata: String,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceCollectionItem {
        pub id: u64,
        pub name: String,
        pub value: f64,
    }

    fn generate_large_collection_xml(count: usize) -> String {
        let mut xml = String::new();
        xml.push_str("<TestPerformanceLargeCollection>");
        xml.push_str("<metadata>Large collection test</metadata>");
        xml.push_str("<items>");

        for i in 0..count {
            xml.push_str(&format!(
                "<TestPerformanceCollectionItem><id>{}</id><name>Item{}</name><value>{}</value></TestPerformanceCollectionItem>",
                i, i, i as f64
            ));
        }

        xml.push_str("</items>");
        xml.push_str("</TestPerformanceLargeCollection>");
        xml
    }

    let xml = generate_large_collection_xml(10000);
    let result = from_xml::<TestPerformanceLargeCollection>(&xml);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert_eq!(parsed.items.len(), 10000);
    assert_eq!(parsed.metadata, "Large collection test");

    Ok(())
}

#[test]
fn test_performance_large_collections_serialization() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceLargeCollection {
        #[xml(tree)]
        pub items: Vec<TestPerformanceCollectionItem>,
        pub metadata: String,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceCollectionItem {
        pub id: u64,
        pub name: String,
        pub value: f64,
    }

    let mut items = Vec::with_capacity(1000);
    for i in 0..1000 {
        items.push(TestPerformanceCollectionItem {
            id: i as u64,
            name: format!("Item{}", i),
            value: i as f64,
        });
    }

    let collection = TestPerformanceLargeCollection {
        items,
        metadata: "Test collection".to_string(),
    };

    let xml = from_obj(&collection);
    let parsed: TestPerformanceLargeCollection = from_xml(&xml)?;
    assert_eq!(parsed.items.len(), 1000);
    assert_eq!(parsed.metadata, "Test collection");

    Ok(())
}

#[test]
fn test_performance_large_collections_mixed_types() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceLargeCollectionMixed {
        #[xml(tree)]
        pub items: Vec<TestPerformanceCollectionItemMixed>,
        pub metadata: String,
        pub count: u32,
        pub active: bool,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceCollectionItemMixed {
        pub id: u64,
        pub name: String,
        pub value: f64,
        pub tags: Vec<String>,
        pub metadata: Option<String>,
    }

    let mut items = Vec::with_capacity(500);
    for i in 0..500 {
        items.push(TestPerformanceCollectionItemMixed {
            id: i as u64,
            name: format!("Item{}", i),
            value: i as f64,
            tags: vec![format!("tag{}", i), format!("category{}", i % 10)],
            metadata: if i % 2 == 0 { Some(format!("Metadata for item {}", i)) } else { None },
        });
    }

    let collection = TestPerformanceLargeCollectionMixed {
        items,
        metadata: "Mixed types collection".to_string(),
        count: 500,
        active: true,
    };

    let xml = from_obj(&collection);
    let parsed: TestPerformanceLargeCollectionMixed = from_xml(&xml)?;
    assert_eq!(parsed.items.len(), 500);
    assert_eq!(parsed.metadata, "Mixed types collection");
    assert_eq!(parsed.count, 500);
    assert_eq!(parsed.active, true);

    Ok(())
} 