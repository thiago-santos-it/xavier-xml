use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};

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
    let success_counter = Arc::new(AtomicUsize::new(0));
    
    // Create multiple threads to test concurrent parsing
    for i in 0..10 {
        let xml_clone = Arc::clone(&xml_arc);
        let results_clone = Arc::clone(&results);
        let counter_clone = Arc::clone(&success_counter);
        
        let handle = thread::spawn(move || {
            let result = from_xml::<ConcurrentTestStruct>(&xml_clone);
            if result.is_ok() {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
            let mut results_guard = results_clone.lock().unwrap();
            results_guard.push((i, result));
        });
        
        handles.push(handle);
    }
    
    // Await all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify that all results are equal
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
}

#[test]
fn concurrent_serialization_thread_safety() {
    let test_data = generate_test_data(1);
    let data_arc = Arc::new(test_data);
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    let success_counter = Arc::new(AtomicUsize::new(0));
    
    // Create multiple threads to test concurrent serialization
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
    
    // Await all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify that all results are equal
    let results_guard = results.lock().unwrap();
    assert_eq!(results_guard.len(), 10);
    
    let success_count = success_counter.load(Ordering::Relaxed);
    assert_eq!(success_count, 10, "All threads should succeed");
    
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
    let _data_arc = Arc::new(test_data);
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    let success_counter = Arc::new(AtomicUsize::new(0));
    
    // Create threads that do both parsing and serialization
    for i in 0..5 {
        let xml_clone = Arc::clone(&xml_arc);
        let results_clone = Arc::clone(&results);
        let counter_clone = Arc::clone(&success_counter);
        
        let handle = thread::spawn(move || {
            // Do parsing
            let parsed = from_xml::<ConcurrentTestStruct>(&xml_clone).unwrap();
            
            // Do serialization
            let serialized = from_obj(&parsed);
            
            // Do parsing again
            let reparsed = from_xml::<ConcurrentTestStruct>(&serialized).unwrap();
            
            counter_clone.fetch_add(1, Ordering::Relaxed);
            let mut results_guard = results_clone.lock().unwrap();
            results_guard.push((i, parsed, reparsed));
        });
        
        handles.push(handle);
    }
    
    // Await all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify that all results are consistent
    let results_guard = results.lock().unwrap();
    assert_eq!(results_guard.len(), 5);
    
    let success_count = success_counter.load(Ordering::Relaxed);
    assert_eq!(success_count, 5, "All mixed operations should succeed");
    
    for (thread_id, parsed, reparsed) in results_guard.iter() {
        assert_eq!(parsed.id, reparsed.id, "Thread {}: ID mismatch", thread_id);
        assert_eq!(parsed.name, reparsed.name, "Thread {}: Name mismatch", thread_id);
        assert_eq!(parsed.data, reparsed.data, "Thread {}: Data mismatch", thread_id);
    }
}

#[test]
fn concurrent_large_data_processing() {
    // Create larger data to test concurrent performance
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
    let success_counter = Arc::new(AtomicUsize::new(0));
    
    // Create multiple threads to process large data
    for _i in 0..5 {
        let xml_clone = Arc::clone(&xml_arc);
        let counter_clone = Arc::clone(&success_counter);
        
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let _parsed = from_xml::<ConcurrentTestStruct>(&xml_clone).unwrap();
            }
            counter_clone.fetch_add(1, Ordering::Relaxed);
        });
        
        handles.push(handle);
    }
    
    // Await all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let duration = start_time.elapsed();
    let success_count = success_counter.load(Ordering::Relaxed);
    
    // Verify that it didn't take too long
    assert!(duration < Duration::from_secs(10));
    assert_eq!(success_count, 5, "All large data processing threads should complete successfully");
}

#[test]
fn concurrent_error_handling() {
    let test_data = generate_test_data(1);
    let xml = from_obj(&test_data);
    let xml_arc = Arc::new(xml);
    
    // Create some invalid XML to test error handling
    let invalid_xml = Arc::new("<invalid>".to_string());
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    let success_counter = Arc::new(AtomicUsize::new(0));
    let error_counter = Arc::new(AtomicUsize::new(0));
    
    // Create threads that process both valid and invalid XML
    for i in 0..10 {
        let xml_clone = if i % 3 == 0 {
            Arc::clone(&invalid_xml)
        } else {
            Arc::clone(&xml_arc)
        };
        let results_clone = Arc::clone(&results);
        let success_clone = Arc::clone(&success_counter);
        let error_clone = Arc::clone(&error_counter);
        
        let handle = thread::spawn(move || {
            let result = from_xml::<ConcurrentTestStruct>(&xml_clone);
            
            match &result {
                Ok(_) => { success_clone.fetch_add(1, Ordering::Relaxed); }
                Err(_) => { error_clone.fetch_add(1, Ordering::Relaxed); }
            }
            
            let mut results_guard = results_clone.lock().unwrap();
            results_guard.push((i, result));
        });
        
        handles.push(handle);
    }
    
    // Await all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify results
    let results_guard = results.lock().unwrap();
    assert_eq!(results_guard.len(), 10);
    
    let success_count = success_counter.load(Ordering::Relaxed);
    let error_count = error_counter.load(Ordering::Relaxed);
    
    // Verify that we have the expected number of successes and errors
    assert!(success_count > 0, "Should have some successful threads");
    assert!(error_count > 0, "Should have some error threads");
    assert_eq!(success_count + error_count, 10, "Total should be 10");
    
    // Verify individual results
    for (thread_id, result) in results_guard.iter() {
        if thread_id % 3 == 0 {
            // These threads should have errors (threads 0, 3, 6, 9)
            assert!(result.is_err(), "Thread {} should have failed with invalid XML", thread_id);
        } else {
            // These threads should succeed (threads 1, 2, 4, 5, 7, 8)
            assert!(result.is_ok(), "Thread {} should have succeeded", thread_id);
            let parsed = result.as_ref().unwrap();
            assert_eq!(parsed.id, 1);
            assert_eq!(parsed.name, "Test1");
        }
    }
} 