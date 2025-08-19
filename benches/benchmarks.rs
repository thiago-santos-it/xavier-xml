use criterion::{black_box, criterion_group, criterion_main, Criterion};
use xavier::{from_obj, from_xml, XmlSerializable, XmlDeserializable, PError};

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct BenchmarkStruct {
    pub id: u64,
    pub name: String,
    pub values: Vec<String>,
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct ComplexStruct {
    pub id: u64,
    pub name: String,
    pub email: Option<String>,
    pub tags: Vec<String>,
    pub active: bool,
    pub score: f64,
    pub metadata: Option<BenchmarkStruct>,
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct LargeDataSet {
    pub items: Vec<DataItem>,
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct DataItem {
    pub id: u64,
    pub content: String,
    pub timestamp: u64,
    pub tags: Vec<String>,
}

fn benchmark_serialization(c: &mut Criterion) {
    let data = BenchmarkStruct {
        id: 1,
        name: "Test".to_string(),
        values: vec!["value1".to_string(), "value2".to_string()],
    };
    
    c.bench_function("serialize_simple_struct", |b| {
        b.iter(|| from_obj(black_box(&data)))
    });
}

fn benchmark_deserialization(c: &mut Criterion) {
    let xml = r#"<BenchmarkStruct><id>1</id><name>Test</name><values><values>value1</values><values>value2</values></values></BenchmarkStruct>"#;
    
    c.bench_function("deserialize_simple_struct", |b| {
        b.iter(|| from_xml::<BenchmarkStruct>(black_box(xml)))
    });
}

fn benchmark_complex_serialization(c: &mut Criterion) {
    let data = ComplexStruct {
        id: 1,
        name: "John Doe".to_string(),
        email: Some("john@example.com".to_string()),
        tags: vec!["rust".to_string(), "xml".to_string(), "serialization".to_string()],
        active: true,
        score: 95.5,
        metadata: Some(BenchmarkStruct {
            id: 2,
            name: "Metadata".to_string(),
            values: vec!["meta1".to_string(), "meta2".to_string()],
        }),
    };
    
    c.bench_function("serialize_complex_struct", |b| {
        b.iter(|| from_obj(black_box(&data)))
    });
}

fn benchmark_complex_deserialization(c: &mut Criterion) {
    let xml = r#"<ComplexStruct><id>1</id><name>John Doe</name><email>john@example.com</email><tags><tags>rust</tags><tags>xml</tags><tags>serialization</tags></tags><active>true</active><score>95.5</score><metadata><id>2</id><name>Metadata</name><values><values>meta1</values><values>meta2</values></values></metadata></ComplexStruct>"#;
    
    c.bench_function("deserialize_complex_struct", |b| {
        b.iter(|| from_xml::<ComplexStruct>(black_box(xml)))
    });
}

fn benchmark_large_data(c: &mut Criterion) {
    let mut values = Vec::new();
    for i in 0..1000 {
        values.push(format!("value{}", i));
    }
    
    let data = BenchmarkStruct {
        id: 1,
        name: "LargeTest".to_string(),
        values,
    };
    
    c.bench_function("serialize_large_struct", |b| {
        b.iter(|| from_obj(black_box(&data)))
    });
}

fn benchmark_large_dataset(c: &mut Criterion) {
    let mut items = Vec::new();
    for i in 0..1000 {
        items.push(DataItem {
            id: i,
            content: format!("Content for item {}", i),
            timestamp: 1640995200 + i,
            tags: vec![format!("tag{}", i % 10), format!("category{}", i % 5)],
        });
    }
    
    let data = LargeDataSet { items };
    
    c.bench_function("serialize_large_dataset", |b| {
        b.iter(|| from_obj(black_box(&data)))
    });
}

fn benchmark_special_characters(c: &mut Criterion) {
    let data = BenchmarkStruct {
        id: 1,
        name: "Test with & < > \" ' characters".to_string(),
        values: vec!["value with & < > \" '".to_string(), "another & < > \" '".to_string()],
    };
    
    c.bench_function("serialize_special_characters", |b| {
        b.iter(|| from_obj(black_box(&data)))
    });
}

fn benchmark_deserialize_special_characters(c: &mut Criterion) {
    let xml = r#"<BenchmarkStruct><id>1</id><name>Test with &amp; &lt; &gt; &quot; &apos; characters</name><values><values>value with &amp; &lt; &gt; &quot; &apos;</values><values>another &amp; &lt; &gt; &quot; &apos;</values></values></BenchmarkStruct>"#;
    
    c.bench_function("deserialize_special_characters", |b| {
        b.iter(|| from_xml::<BenchmarkStruct>(black_box(xml)))
    });
}

fn benchmark_memory_usage(c: &mut Criterion) {
    let mut items = Vec::new();
    for i in 0..10000 {
        items.push(DataItem {
            id: i,
            content: format!("Very long content for item {} with lots of text to simulate memory usage", i),
            timestamp: 1640995200 + i,
            tags: vec![format!("tag{}", i % 100), format!("category{}", i % 50)],
        });
    }
    
    let data = LargeDataSet { items };
    
    c.bench_function("serialize_memory_intensive", |b| {
        b.iter(|| from_obj(black_box(&data)))
    });
}

criterion_group!(
    benches,
    benchmark_serialization,
    benchmark_deserialization,
    benchmark_complex_serialization,
    benchmark_complex_deserialization,
    benchmark_large_data,
    benchmark_large_dataset,
    benchmark_special_characters,
    benchmark_deserialize_special_characters,
    benchmark_memory_usage
);
criterion_main!(benches); 