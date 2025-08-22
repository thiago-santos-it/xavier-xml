use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};
use std::time::Instant;

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct DeepNestedStruct {
    pub id: u32,
    pub level1: Level1,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct Level1 {
    pub name: String,
    pub level2: Level2,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct Level2 {
    pub value: i32,
    pub level3: Level3,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct Level3 {
    pub data: String,
    pub level4: Level4,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct Level4 {
    pub flag: bool,
    pub level5: Level5,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct Level5 {
    pub number: f64,
    pub level6: Level6,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct Level6 {
    pub text: String,
    pub level7: Level7,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct Level7 {
    pub count: u64,
    pub level8: Level8,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct Level8 {
    #[xml(inner="item")]
    pub items: Vec<String>,
    pub level9: Level9,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct Level9 {
    pub metadata: String,
    pub level10: Level10,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct Level10 {
    pub final_value: String,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct LargeCollectionStruct {
    pub id: u32,
    #[xml(inner="item")]
    pub items: Vec<LargeItem>,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct LargeItem {
    pub item_id: u64,
    pub name: String,
    pub description: String,
    #[xml(inner="item")]
    pub tags: Vec<String>,
    pub metadata: Vec<KeyValue>,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct KeyValue {
    pub key: String,
    pub value: String,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct MemoryIntensiveStruct {
    pub id: u32,
    pub large_string: String,
    #[xml(inner="item")]
    pub large_array: Vec<u8>,
    pub nested_data: Vec<MemoryIntensiveStruct>,
}

#[test]
fn test_deep_nesting_performance() {
    let start = Instant::now();
    
    let test_data = create_deep_nested_struct();
    let xml = from_obj(&test_data);
    
    let serialization_time = start.elapsed();
    // Check if it didn't exceed a reasonable time (e.g., 100ms)
    assert!(serialization_time.as_millis() < 100);
    
    let start = Instant::now();
    let parsed: DeepNestedStruct = from_xml(&xml).expect("Falha na deserialização");
    let deserialization_time = start.elapsed();
    
    
    assert!(deserialization_time.as_millis() < 100);
    
    assert_eq!(test_data, parsed);
}

#[test]
fn test_large_collection_performance() {
    let start = Instant::now();
    
    let test_data = create_large_collection(1000); // 1000 itens
    let xml = from_obj(&test_data);
    
    let serialization_time = start.elapsed();
    // Check if it didn't exceed a reasonable time (e.g., 500ms)
    assert!(serialization_time.as_millis() < 500);
    
    let start = Instant::now();
    let parsed: LargeCollectionStruct = from_xml(&xml).expect("Falha na deserialização");
    let deserialization_time = start.elapsed();
    
    
    assert!(deserialization_time.as_millis() < 500);
    
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.items.len(), parsed.items.len());
}

#[test]
fn test_memory_intensive_operations() {
    let start = Instant::now();
    
    let test_data = create_memory_intensive_struct(1);
    let xml = from_obj(&test_data);
    
    let serialization_time = start.elapsed();

    assert!(serialization_time.as_millis() < 200);

    let start = Instant::now();
    let parsed: MemoryIntensiveStruct = from_xml(&xml).expect("Falha na deserialização");
    println!("2 {}", serialization_time.as_millis());
    let deserialization_time = start.elapsed();
    println!("3 {}", serialization_time.as_millis());

    assert!(deserialization_time.as_millis() < 200);
    
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.large_string.len(), parsed.large_string.len());
}

#[test]
fn test_xml_size_limits() {
    // Teste com XML muito grande
    let large_data = create_large_collection(10000); // 10.000 itens
    let xml = from_obj(&large_data);
    
    // Check if XML didn't exceed a reasonable size (e.g., 10MB)
    assert!(xml.len() < 10 * 1024 * 1024);
    
            // Test deserialization of large XML
    let start = Instant::now();
    let parsed: LargeCollectionStruct = from_xml(&xml).expect("Falha na deserialização");
    let _deserialization_time = start.elapsed();
    
    
    
    assert_eq!(large_data.id, parsed.id);
    assert_eq!(large_data.items.len(), parsed.items.len());
}

#[test]
fn test_concurrent_parsing_performance() {
    use std::sync::Arc;
    use std::thread;
    
    let test_data = create_large_collection(100);
    let xml = from_obj(&test_data);
    let xml_arc = Arc::new(xml);
    
    let start = Instant::now();
    
    let handles: Vec<_> = (0..4).map(|_| {
        let xml_clone = Arc::clone(&xml_arc);
        thread::spawn(move || {
            let parsed: LargeCollectionStruct = from_xml(&xml_clone).expect("Falha na deserialização");
            assert_eq!(parsed.items.len(), 100);
        })
    }).collect();
    
    for handle in handles {
        handle.join().expect("Falha na thread");
    }
    
    let total_time = start.elapsed();
    // Check if total time didn't exceed a reasonable limit
    assert!(total_time.as_millis() < 1000);
}

#[test]
fn test_iteration_limits() {
            // Test to verify if parser doesn't enter infinite loop
    let start = Instant::now();
    
    let test_data = create_deep_nested_struct();
    let xml = from_obj(&test_data);
    
            // Add extra tags to test iteration limits
    let xml_with_extra = format!("{}<extra>data</extra>", xml);
    
    let result = from_xml::<DeepNestedStruct>(&xml_with_extra);
    
    let processing_time = start.elapsed();
    // Check if it didn't exceed a reasonable time (e.g., 50ms)
    assert!(processing_time.as_millis() < 50);
    
    // Result should be error or success, but shouldn't hang
    assert!(result.is_ok() || result.is_err());
}

        // Helper functions to create test data
fn create_deep_nested_struct() -> DeepNestedStruct {
    DeepNestedStruct {
        id: 1,
        level1: Level1 {
            name: "Level 1".to_string(),
            level2: Level2 {
                value: 2,
                level3: Level3 {
                    data: "Level 3 Data".to_string(),
                    level4: Level4 {
                        flag: true,
                        level5: Level5 {
                            number: 3.14,
                            level6: Level6 {
                                text: "Level 6 Text".to_string(),
                                level7: Level7 {
                                    count: 7,
                                    level8: Level8 {
                                        items: vec!["item1".to_string(), "item2".to_string()],
                                        level9: Level9 {
                                            metadata: "Level 9 Metadata".to_string(),
                                            level10: Level10 {
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
    }
}

fn create_large_collection(count: usize) -> LargeCollectionStruct {
    let mut items = Vec::with_capacity(count);
    
    for i in 0..count {
        let mut tags = Vec::new();
        for j in 0..5 {
            tags.push(format!("tag{}_{}", i, j));
        }
        
        let mut metadata = Vec::new();
        for j in 0..3 {
            metadata.push(KeyValue {
                key: format!("key{}_{}", i, j),
                value: format!("value{}_{}", i, j),
            });
        }
        
        items.push(LargeItem {
            item_id: i as u64,
            name: format!("Item {}", i),
            description: format!("Description for item {}", i),
            tags,
            metadata,
        });
    }
    
    LargeCollectionStruct {
        id: 1,
        items,
    }
}

fn create_memory_intensive_struct(depth: u32) -> MemoryIntensiveStruct {
    if depth == 0 {
        MemoryIntensiveStruct {
            id: 0,
            large_string: "".to_string(),
            large_array: vec![],
            nested_data: vec![],
        }
    } else {
        let mut nested_data = Vec::new();
        for _i in 0..3 {
            nested_data.push(create_memory_intensive_struct(depth - 1));
        }
        
        MemoryIntensiveStruct {
            id: depth,
            large_string: "A".repeat(1000), // String de 1000 caracteres
            large_array: vec![0u8; 1000],   // Array de 1000 bytes
            nested_data,
        }
    }
} 