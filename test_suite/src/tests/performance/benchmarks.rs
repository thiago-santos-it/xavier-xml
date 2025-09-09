use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};

#[test]
fn test_performance_benchmarks_simple_struct() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceBenchmark {
        pub id: u64,
        pub name: String,
        #[xml(inner="value")]
        pub values: Vec<String>,
    }

    let data = TestPerformanceBenchmark {
        id: 1,
        name: "Test".to_string(),
        values: vec!["value1".to_string(), "value2".to_string()],
    };
    
    let xml = from_obj(&data);
    
    let parsed: TestPerformanceBenchmark = from_xml(&xml)?;
    assert_eq!(data, parsed);
    
    Ok(())
}

#[test]
fn test_performance_benchmarks_complex_struct() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceBenchmarkComplex {
        pub id: u64,
        pub name: String,
        #[xml(tree)]
        pub metadata: TestPerformanceBenchmarkMetadata,
        pub items: Vec<TestPerformanceBenchmarkItem>,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceBenchmarkMetadata {
        pub version: String,
        pub timestamp: u64,
        pub active: bool,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceBenchmarkItem {
        pub id: u32,
        pub name: String,
        pub value: f64,
        #[xml(inner="value")]
        pub tags: Vec<String>,
    }

    let data = TestPerformanceBenchmarkComplex {
        id: 1,
        name: "Complex Test".to_string(),
        metadata: TestPerformanceBenchmarkMetadata {
            version: "1.0.0".to_string(),
            timestamp: 1234567890,
            active: true,
        },
        items: vec![
            TestPerformanceBenchmarkItem {
                id: 1,
                name: "Item 1".to_string(),
                value: 3.14,
                tags: vec!["tag1".to_string(), "tag2".to_string()],
            },
            TestPerformanceBenchmarkItem {
                id: 2,
                name: "Item 2".to_string(),
                value: 2.718,
                tags: vec!["tag3".to_string()],
            },
        ],
    };
    
    let xml = from_obj(&data);
    
    let parsed: TestPerformanceBenchmarkComplex = from_xml(&xml)?;
    assert_eq!(data, parsed);
    
    Ok(())
}

#[test]
fn test_performance_benchmarks_large_collection() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceBenchmarkLarge {
        pub id: u64,
        pub name: String,
        pub items: Vec<TestPerformanceBenchmarkItem>,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceBenchmarkItem {
        pub id: u32,
        pub name: String,
        pub value: f64,
    }

    let mut items = Vec::with_capacity(1000);
    for i in 0..1000 {
        items.push(TestPerformanceBenchmarkItem {
            id: i as u32,
            name: format!("Item {}", i),
            value: i as f64 * 1.5,
        });
    }
    
    let data = TestPerformanceBenchmarkLarge {
        id: 1,
        name: "Large Collection".to_string(),
        items,
    };
    
    let xml = from_obj(&data);
    
    let parsed: TestPerformanceBenchmarkLarge = from_xml(&xml)?;
    assert_eq!(data.id, parsed.id);
    assert_eq!(data.name, parsed.name);
    assert_eq!(data.items.len(), parsed.items.len());
    
    Ok(())
}

#[test]
fn test_performance_benchmarks_mixed_types() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceBenchmarkMixed {
        pub id: u64,
        pub name: String,
        pub count: i32,
        pub price: f64,
        pub active: bool,
        #[xml(inner="tag")]
        pub tags: Vec<String>,
        pub metadata: Option<String>,
        #[xml(tree)]
        pub nested: Option<TestPerformanceBenchmarkNested>,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestPerformanceBenchmarkNested {
        pub value: String,
        pub count: u32,
    }

    let data = TestPerformanceBenchmarkMixed {
        id: 1,
        name: "Mixed Types".to_string(),
        count: 42,
        price: 3.14159,
        active: true,
        tags: vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()],
        metadata: Some("Some metadata".to_string()),
        nested: Some(TestPerformanceBenchmarkNested {
            value: "Nested value".to_string(),
            count: 100,
        }),
    };
    
    let xml = from_obj(&data);
    
    let parsed: TestPerformanceBenchmarkMixed = from_xml(&xml)?;
    assert_eq!(data, parsed);
    
    Ok(())
} 