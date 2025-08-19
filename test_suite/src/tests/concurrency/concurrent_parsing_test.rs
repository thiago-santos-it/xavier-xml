use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(XmlSerializable, XmlDeserializable, Debug, Clone)]
struct ConcurrentTestStruct {
    pub id: u64,
    pub name: String,
    #[xml(inner="item")]
    pub data: Vec<String>,
}

fn generate_test_data(id: u64) -> ConcurrentTestStruct {
    ConcurrentTestStruct {
        id,
        name: format!("Test{}", id),
        data: vec![format!("data{}", id), format!("value{}", id)],
    }
}

#[test]
fn concurrent_parsing_thread_safety() {
    let test_data = generate_test_data(1);
    let xml = from_obj(&test_data);
    let xml_arc = Arc::new(xml);
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    
    // Criar múltiplas threads para testar parsing concorrente
    for i in 0..10 {
        let xml_clone = Arc::clone(&xml_arc);
        let results_clone = Arc::clone(&results);
        
        let handle = thread::spawn(move || {
            let result = from_xml::<ConcurrentTestStruct>(&xml_clone);
            let mut results_guard = results_clone.lock().unwrap();
            results_guard.push((i, result));
        });
        
        handles.push(handle);
    }
    
    // Aguardar todas as threads terminarem
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verificar se todos os resultados são iguais
    let results_guard = results.lock().unwrap();
    assert_eq!(results_guard.len(), 10);
    
    for (thread_id, result) in results_guard.iter() {
        assert!(result.is_ok(), "Thread {} failed to parse XML", thread_id);
        let parsed = result.as_ref().unwrap();
        assert_eq!(parsed.id, 1);
        assert_eq!(parsed.name, "Test1");
        assert_eq!(parsed.data.len(), 2);
    }
}

#[test]
fn concurrent_serialization_thread_safety() {
    let test_data = generate_test_data(1);
    let data_arc = Arc::new(test_data);
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    
    // Criar múltiplas threads para testar serialização concorrente
    for i in 0..10 {
        let data_clone = Arc::clone(&data_arc);
        let results_clone = Arc::clone(&results);
        
        let handle = thread::spawn(move || {
            let xml = from_obj(&*data_clone);
            let mut results_guard = results_clone.lock().unwrap();
            results_guard.push((i, xml));
        });
        
        handles.push(handle);
    }
    
    // Aguardar todas as threads terminarem
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verificar se todos os resultados são iguais
    let results_guard = results.lock().unwrap();
    assert_eq!(results_guard.len(), 10);
    
    let first_result = &results_guard[0].1;
    for (thread_id, result) in results_guard.iter() {
        assert_eq!(result, first_result, "Thread {} produced different XML", thread_id);
    }
}

#[test]
fn concurrent_mixed_operations() {
    let test_data = generate_test_data(1);
    let xml = from_obj(&test_data);
    let xml_arc = Arc::new(xml);
    let data_arc = Arc::new(test_data);
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    
    // Criar threads que fazem tanto parsing quanto serialização
    for i in 0..5 {
        let xml_clone = Arc::clone(&xml_arc);
        let _data_clone = Arc::clone(&data_arc);
        let results_clone = Arc::clone(&results);
        
        let handle = thread::spawn(move || {
            // Fazer parsing
            let parsed = from_xml::<ConcurrentTestStruct>(&xml_clone).unwrap();
            
            // Fazer serialização
            let serialized = from_obj(&parsed);
            
            // Fazer parsing novamente
            let reparsed = from_xml::<ConcurrentTestStruct>(&serialized).unwrap();
            
            let mut results_guard = results_clone.lock().unwrap();
            results_guard.push((i, parsed, reparsed));
        });
        
        handles.push(handle);
    }
    
    // Aguardar todas as threads terminarem
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verificar se todos os resultados são consistentes
    let results_guard = results.lock().unwrap();
    assert_eq!(results_guard.len(), 5);
    
    for (thread_id, parsed, reparsed) in results_guard.iter() {
        assert_eq!(parsed.id, reparsed.id, "Thread {}: ID mismatch", thread_id);
        assert_eq!(parsed.name, reparsed.name, "Thread {}: Name mismatch", thread_id);
        assert_eq!(parsed.data, reparsed.data, "Thread {}: Data mismatch", thread_id);
    }
}

#[test]
fn concurrent_large_data_processing() {
    // Criar dados maiores para testar performance concorrente
    let mut large_data = ConcurrentTestStruct {
        id: 1,
        name: "LargeTest".to_string(),
        data: Vec::with_capacity(1000),
    };
    
    for i in 0..1000 {
        large_data.data.push(format!("data{}", i));
    }
    
    let xml = from_obj(&large_data);
    let xml_arc = Arc::new(xml);
    
    let mut handles = vec![];
    let start_time = std::time::Instant::now();
    
    // Criar múltiplas threads para processar dados grandes
    for _i in 0..5 {
        let xml_clone = Arc::clone(&xml_arc);
        
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let _parsed = from_xml::<ConcurrentTestStruct>(&xml_clone).unwrap();
            }
        });
        
        handles.push(handle);
    }
    
    // Aguardar todas as threads terminarem
    for handle in handles {
        handle.join().unwrap();
    }
    
    let duration = start_time.elapsed();
    println!("Concurrent large data processing took: {:?}", duration);
    
    // Verificar se não demorou muito
    assert!(duration < Duration::from_secs(10));
} 