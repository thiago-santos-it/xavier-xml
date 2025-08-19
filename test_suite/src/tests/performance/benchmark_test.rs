use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct BenchmarkStruct {
    pub id: u64,
    pub name: String,
    pub values: Vec<String>,
}

#[cfg(test)]
mod benchmarks {
    use super::*;
    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    #[allow(dead_code)]
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
    #[allow(dead_code)]
    fn benchmark_deserialization(c: &mut Criterion) {
        let xml = r#"<BenchmarkStruct><id>1</id><name>Test</name><values><values>value1</values><values>value2</values></values></BenchmarkStruct>"#;
        
        c.bench_function("deserialize_simple_struct", |b| {
            b.iter(|| from_xml::<BenchmarkStruct>(black_box(xml)))
        });
    }

    criterion_group!(benches, benchmark_serialization, benchmark_deserialization);
    criterion_main!(benches);
} 