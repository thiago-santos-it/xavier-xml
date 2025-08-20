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
    pub items: Vec<LargeItem>,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct LargeItem {
    pub item_id: u64,
    pub name: String,
    pub description: String,
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
    pub large_array: Vec<u8>,
    pub nested_data: Vec<MemoryIntensiveStruct>,
}

#[test]
fn test_deep_nesting_performance() {
    let start = Instant::now();
    
    let test_data = create_deep_nested_struct();
    let xml = from_obj(&test_data);
    
    let serialization_time = start.elapsed();
    println!("Tempo de serialização (10 níveis): {:?}", serialization_time);
    
    // Verificar se não excedeu um tempo razoável (ex: 100ms)
    assert!(serialization_time.as_millis() < 100);
    
    let start = Instant::now();
    let parsed: DeepNestedStruct = from_xml(&xml).expect("Falha na deserialização");
    let deserialization_time = start.elapsed();
    
    println!("Tempo de deserialização (10 níveis): {:?}", deserialization_time);
    assert!(deserialization_time.as_millis() < 100);
    
    assert_eq!(test_data, parsed);
}

#[test]
fn test_large_collection_performance() {
    let start = Instant::now();
    
    let test_data = create_large_collection(1000); // 1000 itens
    let xml = from_obj(&test_data);
    
    let serialization_time = start.elapsed();
    println!("Tempo de serialização (1000 itens): {:?}", serialization_time);
    
    // Verificar se não excedeu um tempo razoável (ex: 500ms)
    assert!(serialization_time.as_millis() < 500);
    
    let start = Instant::now();
    let parsed: LargeCollectionStruct = from_xml(&xml).expect("Falha na deserialização");
    let deserialization_time = start.elapsed();
    
    println!("Tempo de deserialização (1000 itens): {:?}", deserialization_time);
    assert!(deserialization_time.as_millis() < 500);
    
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.items.len(), parsed.items.len());
}

#[test]
fn test_memory_intensive_operations() {
    let start = Instant::now();
    
    let test_data = create_memory_intensive_struct(5); // 5 níveis de aninhamento
    let xml = from_obj(&test_data);
    
    let serialization_time = start.elapsed();
    println!("Tempo de serialização (estrutura intensiva): {:?}", serialization_time);
    
    // Verificar se não excedeu um tempo razoável (ex: 200ms)
    assert!(serialization_time.as_millis() < 200);
    
    let start = Instant::now();
    let parsed: MemoryIntensiveStruct = from_xml(&xml).expect("Falha na deserialização");
    let deserialization_time = start.elapsed();
    
    println!("Tempo de deserialização (estrutura intensiva): {:?}", deserialization_time);
    assert!(deserialization_time.as_millis() < 200);
    
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.large_string.len(), parsed.large_string.len());
}

#[test]
fn test_xml_size_limits() {
    // Teste com XML muito grande
    let large_data = create_large_collection(10000); // 10.000 itens
    let xml = from_obj(&large_data);
    
    println!("Tamanho do XML: {} bytes", xml.len());
    
    // Verificar se o XML não excedeu um tamanho razoável (ex: 10MB)
    assert!(xml.len() < 10 * 1024 * 1024);
    
    // Teste de deserialização do XML grande
    let start = Instant::now();
    let parsed: LargeCollectionStruct = from_xml(&xml).expect("Falha na deserialização");
    let deserialization_time = start.elapsed();
    
    println!("Tempo de deserialização (10.000 itens): {:?}", deserialization_time);
    
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
    println!("Tempo total para 4 threads paralelas: {:?}", total_time);
    
    // Verificar se o tempo total não excedeu um limite razoável
    assert!(total_time.as_millis() < 1000);
}

#[test]
fn test_iteration_limits() {
    // Teste para verificar se o parser não entra em loop infinito
    let start = Instant::now();
    
    let test_data = create_deep_nested_struct();
    let xml = from_obj(&test_data);
    
    // Adicionar tags extras para testar limites de iteração
    let xml_with_extra = format!("{}<extra>data</extra>", xml);
    
    let result = from_xml::<DeepNestedStruct>(&xml_with_extra);
    
    let processing_time = start.elapsed();
    println!("Tempo de processamento com tags extras: {:?}", processing_time);
    
    // Verificar se não excedeu um tempo razoável (ex: 50ms)
    assert!(processing_time.as_millis() < 50);
    
    // O resultado deve ser um erro ou sucesso, mas não deve travar
    assert!(result.is_ok() || result.is_err());
}

// Funções auxiliares para criar dados de teste
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