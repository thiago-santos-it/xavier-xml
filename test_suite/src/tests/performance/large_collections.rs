use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct LargeCollectionStruct {
    #[xml(tree)]
    pub items: Vec<CollectionItem>,
    pub metadata: String,
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct CollectionItem {
    pub id: u64,
    pub name: String,
    pub value: f64,
}

fn generate_large_collection_xml(count: usize) -> String {
    let mut xml = String::new();
    xml.push_str("<LargeCollectionStruct>");
    xml.push_str("<metadata>Large collection test</metadata>");
    xml.push_str("<items>");
    
    for i in 0..count {
        xml.push_str(&format!(
            "<CollectionItem><id>{}</id><name>Item{}</name><value>{}</value></CollectionItem>",
            i, i, i as f64
        ));
    }
    
    xml.push_str("</items>");
    xml.push_str("</LargeCollectionStruct>");
    xml
}

#[test]
fn stress_large_collections() {
    let xml = generate_large_collection_xml(1000);
    let result = from_xml::<LargeCollectionStruct>(&xml);
    assert!(result.is_ok());
    
    let parsed = result.unwrap();
    assert_eq!(parsed.items.len(), 1000);
}

#[test]
fn stress_large_collections_10000_items() {
    let xml = generate_large_collection_xml(10000);
    let result = from_xml::<LargeCollectionStruct>(&xml);
    assert!(result.is_ok());
    
    let parsed = result.unwrap();
    assert_eq!(parsed.items.len(), 10000);
}

#[test]
fn stress_large_collections_serialization() {
    let mut items = Vec::with_capacity(1000);
    for i in 0..1000 {
        items.push(CollectionItem {
            id: i as u64,
            name: format!("Item{}", i),
            value: i as f64,
        });
    }
    
    let collection = LargeCollectionStruct {
        items,
        metadata: "Test collection".to_string(),
    };
    
    let xml = from_obj(&collection);
    let parsed: LargeCollectionStruct = from_xml(&xml).unwrap();
    assert_eq!(parsed.items.len(), 1000);
} 