use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};
use std::time::Instant;

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct LargeDataSet {
    pub items: Vec<DataItem>,
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct DataItem {
    pub id: u64,
    pub content: String,
    pub tags: Vec<String>,
}

fn generate_large_dataset(count: usize) -> LargeDataSet {
    let mut items = Vec::with_capacity(count);
    for i in 0..count {
        items.push(DataItem {
            id: i as u64,
            content: format!("Content for item {}", i),
            tags: vec![format!("tag{}", i), format!("category{}", i % 10)],
        });
    }
    LargeDataSet { items }
}

#[test]
fn performance_large_xml_parsing() {
    // Gerar XML grande (1000+ itens)
    let large_data = generate_large_dataset(1000);
    let xml = from_obj(&large_data);
    
    let start = Instant::now();
    let parsed: LargeDataSet = from_xml(&xml).unwrap();
    let duration = start.elapsed();
    
    assert!(duration < std::time::Duration::from_secs(5)); // Maximum 5 seconds
    assert_eq!(parsed.items.len(), 1000);
}

#[test]
fn performance_memory_usage() {
    // Memory usage test with large XMLs
    let large_data = generate_large_dataset(5000);
    let xml = from_obj(&large_data);
    
    let start = Instant::now();
    let parsed: LargeDataSet = from_xml(&xml).unwrap();
    let duration = start.elapsed();
    
    assert!(duration < std::time::Duration::from_secs(10)); // Maximum 10 seconds
    assert_eq!(parsed.items.len(), 5000);
}

#[test]
fn performance_serialization_speed() {
    let large_data = generate_large_dataset(1000);
    
    let start = Instant::now();
    let _xml = from_obj(&large_data);
    let duration = start.elapsed();
    
    assert!(duration < std::time::Duration::from_secs(2)); // Maximum 2 seconds
} 