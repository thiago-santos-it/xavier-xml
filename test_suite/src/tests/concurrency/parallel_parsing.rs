use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn test_parsing_thread_safety() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, Clone, PartialEq)]
    struct TestConcurrentParsingStruct {
        pub id: u64,
        pub name: String,
        #[xml(inner="item")]
        pub data: Vec<String>,
    }

    fn generate_test_data(id: u64) -> TestConcurrentParsingStruct {
        TestConcurrentParsingStruct {
            id,
            name: format!("Test{}", id),
            data: vec![format!("data{}", id), format!("value{}", id)],
        }
    }

    let test_data = generate_test_data(1);
    let xml = from_obj(&test_data);
    let xml_arc = Arc::new(xml);
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    let success_counter = Arc::new(AtomicUsize::new(0));
    
    
    for i in 0..10 {
        let xml_clone = Arc::clone(&xml_arc);
        let results_clone = Arc::clone(&results);
        let counter_clone = Arc::clone(&success_counter);
        
        let handle = thread::spawn(move || {
            let result = from_xml::<TestConcurrentParsingStruct>(&xml_clone);
            if result.is_ok() {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
            let mut results_guard = results_clone.lock().unwrap();
            results_guard.push((i, result));
        });
        
        handles.push(handle);
    }
    
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    
    let results_guard = results.lock().unwrap();
    assert_eq!(results_guard.len(), 10);
    
    let success_count = success_counter.load(Ordering::Relaxed);
    assert_eq!(success_count, 10, "All threads should succeed");
    
    for (thread_id, result) in results_guard.iter() {
        assert!(result.is_ok(), "Thread {} failed to parse XML", thread_id);
        let parsed = result.as_ref().unwrap();
        assert_eq!(parsed.id, 1);
        assert_eq!(parsed.name, "Test1");
        assert_eq!(parsed.data.len(), 2);
    }
    
    Ok(())
}

#[test]
fn test_serialization_thread_safety() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, Clone, PartialEq)]
    struct TestConcurrentSerializationStruct {
        pub id: u64,
        pub name: String,
        #[xml(inner="item")]
        pub data: Vec<String>,
    }

    fn generate_test_data(id: u64) -> TestConcurrentSerializationStruct {
        TestConcurrentSerializationStruct {
            id,
            name: format!("Test{}", id),
            data: vec![format!("data{}", id), format!("value{}", id)],
        }
    }

    let test_data = generate_test_data(1);
    let data_arc = Arc::new(test_data);
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    let success_counter = Arc::new(AtomicUsize::new(0));
    
    
    for i in 0..10 {
        let data_clone = Arc::clone(&data_arc);
        let results_clone = Arc::clone(&results);
        let counter_clone = Arc::clone(&success_counter);
        
        let handle = thread::spawn(move || {
            let xml = from_obj(&*data_clone);
            counter_clone.fetch_add(1, Ordering::Relaxed);
            let mut results_guard = results_clone.lock().unwrap();
            results_guard.push((i, xml));
        });
        
        handles.push(handle);
    }
    
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let results_guard = results.lock().unwrap();
    assert_eq!(results_guard.len(), 10);
    
    let success_count = success_counter.load(Ordering::Relaxed);
    assert_eq!(success_count, 10, "All threads should succeed");

    let first_xml = &results_guard[0].1;
    for (thread_id, xml) in results_guard.iter() {
        assert_eq!(xml, first_xml, "Thread {} produced different XML", thread_id);
    }
    
    Ok(())
}

#[test]
fn test_mixed_operations() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, Clone, PartialEq)]
    struct TestConcurrentMixedStruct {
        pub id: u64,
        pub name: String,
        pub count: i32,
        #[xml(inner="item")]
        pub items: Vec<String>,
    }

    fn generate_test_data(id: u64) -> TestConcurrentMixedStruct {
        TestConcurrentMixedStruct {
            id,
            name: format!("MixedTest{}", id),
            count: id as i32,
            items: vec![format!("item{}", id), format!("value{}", id)],
        }
    }

    let test_data = generate_test_data(1);
    let data_arc = Arc::new(test_data);
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    let success_counter = Arc::new(AtomicUsize::new(0));
    
    
    for i in 0..5 {
        let data_clone = Arc::clone(&data_arc);
        let results_clone = Arc::clone(&results);
        let counter_clone = Arc::clone(&success_counter);
        
        let handle = thread::spawn(move || {
            
            let xml = from_obj(&*data_clone);
            
            
            let result = from_xml::<TestConcurrentMixedStruct>(&xml);
            
            if result.is_ok() {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
            
            let mut results_guard = results_clone.lock().unwrap();
            results_guard.push((i, result));
        });
        
        handles.push(handle);
    }
    
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    
    let results_guard = results.lock().unwrap();
    assert_eq!(results_guard.len(), 5);
    
    let success_count = success_counter.load(Ordering::Relaxed);
    assert_eq!(success_count, 5, "All threads should succeed");
    
    for (thread_id, result) in results_guard.iter() {
        assert!(result.is_ok(), "Thread {} failed", thread_id);
        let parsed = result.as_ref().unwrap();
        assert_eq!(parsed.id, 1);
        assert_eq!(parsed.name, "MixedTest1");
        assert_eq!(parsed.count, 1);
        assert_eq!(parsed.items.len(), 2);
    }
    
    Ok(())
}

#[test]
fn test_large_data_processing() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, Clone, PartialEq)]
    struct TestConcurrentLargeStruct {
        pub id: u64,
        pub name: String,
        #[xml(inner="item")]
        pub data: Vec<String>,
        #[xml(inner="item")]
        pub metadata: Vec<String>,
    }

    fn generate_large_test_data(id: u64) -> TestConcurrentLargeStruct {
        let mut data = Vec::new();
        let mut metadata = Vec::new();
        
        for i in 0..100 {
            data.push(format!("data{}_{}", id, i));
            metadata.push(format!("meta{}_{}", id, i));
        }
        
        TestConcurrentLargeStruct {
            id,
            name: format!("LargeTest{}", id),
            data,
            metadata,
        }
    }

    let test_data = generate_large_test_data(1);
    let data_arc = Arc::new(test_data);
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    let success_counter = Arc::new(AtomicUsize::new(0));
    
    for i in 0..8 {
        let data_clone = Arc::clone(&data_arc);
        let results_clone = Arc::clone(&results);
        let counter_clone = Arc::clone(&success_counter);
        
        let handle = thread::spawn(move || {
            let xml = from_obj(&*data_clone);
            let result = from_xml::<TestConcurrentLargeStruct>(&xml);
            
            if result.is_ok() {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
            
            let mut results_guard = results_clone.lock().unwrap();
            results_guard.push((i, result));
        });
        
        handles.push(handle);
    }
    
    
    for handle in handles {
        handle.join().unwrap();
    }

    let results_guard = results.lock().unwrap();
    assert_eq!(results_guard.len(), 8);
    
    let success_count = success_counter.load(Ordering::Relaxed);
    assert_eq!(success_count, 8, "All threads should succeed");
    
    for (thread_id, result) in results_guard.iter() {
        assert!(result.is_ok(), "Thread {} failed", thread_id);
        let parsed = result.as_ref().unwrap();
        assert_eq!(parsed.id, 1);
        assert_eq!(parsed.name, "LargeTest1");
        assert_eq!(parsed.data.len(), 100);
        assert_eq!(parsed.metadata.len(), 100);
    }
    
    Ok(())
} 