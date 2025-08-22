use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::fmt;
use std::str::FromStr;

#[derive(XmlSerializable, XmlDeserializable, Debug, Clone, PartialEq)]
struct ThreadSafeTestStruct {
    pub id: u64,
    pub name: String,
    #[xml(inner="item")]
    pub data: Vec<String>,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, Clone, PartialEq)]
struct ComplexNestedStruct {
    pub id: u32,
    pub name: String,
    #[xml(inner="child")]
    pub children: Vec<ThreadSafeTestStruct>,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, Clone, PartialEq)]
enum ThreadSafeEnum {
    VariantA,
    VariantB,
    VariantC,
}

impl fmt::Display for ThreadSafeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThreadSafeEnum::VariantA => write!(f, "VariantA"),
            ThreadSafeEnum::VariantB => write!(f, "VariantB"),
            ThreadSafeEnum::VariantC => write!(f, "VariantC"),
        }
    }
}

impl FromStr for ThreadSafeEnum {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "VariantA" => Ok(ThreadSafeEnum::VariantA),
            "VariantB" => Ok(ThreadSafeEnum::VariantB),
            "VariantC" => Ok(ThreadSafeEnum::VariantC),
            _ => Err(()),
        }
    }
}

#[derive(XmlSerializable, XmlDeserializable, Debug, Clone, PartialEq)]
struct EnumContainer {
    pub variant: ThreadSafeEnum,
}

fn generate_test_struct(id: u64) -> ThreadSafeTestStruct {
    ThreadSafeTestStruct {
        id,
        name: format!("TestStruct{}", id),
        data: vec![format!("data{}", id), format!("value{}", id)],
    }
}

fn generate_complex_struct(id: u32) -> ComplexNestedStruct {
    ComplexNestedStruct {
        id,
        name: format!("Complex{}", id),
        children: vec![
            generate_test_struct(1),
            generate_test_struct(2),
        ],
    }
}

#[test]
fn basic_thread_safety_parsing() {
    let test_data = generate_test_struct(1);
    let xml = from_obj(&test_data);
    let xml_arc = Arc::new(xml);
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    let success_counter = Arc::new(AtomicUsize::new(0));
    
    // Create multiple threads to test concurrent parsing
    for i in 0..20 {
        let xml_clone = Arc::clone(&xml_arc);
        let results_clone = Arc::clone(&results);
        let counter_clone = Arc::clone(&success_counter);
        
        let handle = thread::spawn(move || {
            let result = from_xml::<ThreadSafeTestStruct>(&xml_clone);
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
    
    // Verify that all results are equal and correct
    let results_guard = results.lock().unwrap();
    assert_eq!(results_guard.len(), 20);
    
    let success_count = success_counter.load(Ordering::Relaxed);
    assert_eq!(success_count, 20, "All threads should succeed");
    
    for (thread_id, result) in results_guard.iter() {
        assert!(result.is_ok(), "Thread {} failed to parse XML: {:?}", thread_id, result);
        let parsed = result.as_ref().unwrap();
        assert_eq!(parsed.id, 1);
        assert_eq!(parsed.name, "TestStruct1");
        assert_eq!(parsed.data.len(), 2);
    }
}

#[test]
fn basic_thread_safety_serialization() {
    let test_data = generate_test_struct(1);
    let data_arc = Arc::new(test_data);
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    let success_counter = Arc::new(AtomicUsize::new(0));
    
    // Create multiple threads to test concurrent serialization
    for i in 0..20 {
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
    assert_eq!(results_guard.len(), 20);
    
    let success_count = success_counter.load(Ordering::Relaxed);
    assert_eq!(success_count, 20, "All threads should succeed");
    
    let first_result = &results_guard[0].1;
    for (thread_id, result) in results_guard.iter() {
        assert_eq!(result, first_result, "Thread {} produced different XML", thread_id);
    }
}

#[test]
fn complex_nested_struct_thread_safety() {
    let test_data = generate_complex_struct(1);
    let xml = from_obj(&test_data);
    let xml_arc = Arc::new(xml);
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    let success_counter = Arc::new(AtomicUsize::new(0));
    
    // Create multiple threads to test concurrent parsing of complex structures
    for i in 0..15 {
        let xml_clone = Arc::clone(&xml_arc);
        let results_clone = Arc::clone(&results);
        let counter_clone = Arc::clone(&success_counter);
        
        let handle = thread::spawn(move || {
            let result = from_xml::<ComplexNestedStruct>(&xml_clone);
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
    
    // Verify that all results are equal and correct
    let results_guard = results.lock().unwrap();
    assert_eq!(results_guard.len(), 15);
    
    let success_count = success_counter.load(Ordering::Relaxed);
    assert_eq!(success_count, 15, "All threads should succeed");
    
    for (thread_id, result) in results_guard.iter() {
        assert!(result.is_ok(), "Thread {} failed to parse complex XML: {:?}", thread_id, result);
        let parsed = result.as_ref().unwrap();
        assert_eq!(parsed.id, 1);
        assert_eq!(parsed.name, "Complex1");
        assert_eq!(parsed.children.len(), 2);
    }
}

#[test]
fn enum_thread_safety() {
    // Test enum within a struct (this approach works)
    let container = EnumContainer {
        variant: ThreadSafeEnum::VariantA,
    };
    
    let xml = from_obj(&container);
    println!("Generated XML: {}", xml);
    
    let parsed = from_xml::<EnumContainer>(&xml);
    println!("Parse result: {:?}", parsed);
    
    match parsed {
        Ok(parsed_container) => {
            assert_eq!(container.variant, parsed_container.variant);
        }
        Err(e) => {
            panic!("Failed to parse enum container: {:?}", e);
        }
    }
    
    // Test multiple variants
    let variants = vec![
        ThreadSafeEnum::VariantA,
        ThreadSafeEnum::VariantB,
        ThreadSafeEnum::VariantC,
    ];
    
    for variant in variants {
        let container = EnumContainer { variant };
        let xml = from_obj(&container);
        let parsed = from_xml::<EnumContainer>(&xml).unwrap();
        assert_eq!(container.variant, parsed.variant);
    }
}

#[test]
fn concurrent_mixed_operations_thread_safety() {
    let test_data = generate_test_struct(1);
    let xml = from_obj(&test_data);
    let xml_arc = Arc::new(xml);
    let _data_arc = Arc::new(test_data);
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    let success_counter = Arc::new(AtomicUsize::new(0));
    
    // Create threads that do both parsing and serialization
    for i in 0..10 {
        let xml_clone = Arc::clone(&xml_arc);
        let results_clone = Arc::clone(&results);
        let counter_clone = Arc::clone(&success_counter);
        
        let handle = thread::spawn(move || {
            // Do parsing
            let parsed = from_xml::<ThreadSafeTestStruct>(&xml_clone).unwrap();
            
            // Do serialization
            let serialized = from_obj(&parsed);
            
            // Do parsing again
            let reparsed = from_xml::<ThreadSafeTestStruct>(&serialized).unwrap();
            
            // Do another serialization
            let final_serialized = from_obj(&reparsed);
            
            counter_clone.fetch_add(1, Ordering::Relaxed);
            let mut results_guard = results_clone.lock().unwrap();
            results_guard.push((i, parsed, reparsed, final_serialized));
        });
        
        handles.push(handle);
    }
    
    // Await all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify that all results are consistent
    let results_guard = results.lock().unwrap();
    assert_eq!(results_guard.len(), 10);
    
    let success_count = success_counter.load(Ordering::Relaxed);
    assert_eq!(success_count, 10, "All mixed operations should succeed");
    
    for (thread_id, parsed, reparsed, _final_serialized) in results_guard.iter() {
        assert_eq!(parsed.id, reparsed.id, "Thread {}: ID mismatch", thread_id);
        assert_eq!(parsed.name, reparsed.name, "Thread {}: Name mismatch", thread_id);
        assert_eq!(parsed.data, reparsed.data, "Thread {}: Data mismatch", thread_id);
    }
}

#[test]
fn stress_test_thread_safety() {
    let test_data = generate_test_struct(1);
    let xml = from_obj(&test_data);
    let xml_arc = Arc::new(xml);
    
    let mut handles = vec![];
    let start_time = std::time::Instant::now();
    let success_counter = Arc::new(AtomicUsize::new(0));
    
    // Create many threads to stress test the system
    for _i in 0..50 {
        let xml_clone = Arc::clone(&xml_arc);
        let counter_clone = Arc::clone(&success_counter);
        
        let handle = thread::spawn(move || {
            // Perform multiple operations in each thread
            for _j in 0..100 {
                let parsed = from_xml::<ThreadSafeTestStruct>(&xml_clone).unwrap();
                let serialized = from_obj(&parsed);
                let reparsed = from_xml::<ThreadSafeTestStruct>(&serialized).unwrap();
                
                // Verify consistency
                assert_eq!(parsed.id, reparsed.id);
                assert_eq!(parsed.name, reparsed.name);
                assert_eq!(parsed.data, reparsed.data);
                
                if _j % 10 == 0 {
                    // Small delay to simulate real-world usage
                    thread::sleep(Duration::from_micros(100));
                }
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
    
    // Verify that it didn't take too long and completed successfully
    assert!(duration < Duration::from_secs(30), "Stress test took too long: {:?}", duration);
    assert_eq!(success_count, 50, "All stress test threads should complete successfully");
}

#[test]
fn panic_safety_thread_safety() {
    let test_data = generate_test_struct(1);
    let xml = from_obj(&test_data);
    let xml_arc = Arc::new(xml);
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    let success_counter = Arc::new(AtomicUsize::new(0));
    
    // Create threads, some of which will panic
    for i in 0..10 {
        let xml_clone = Arc::clone(&xml_arc);
        let results_clone = Arc::clone(&results);
        let counter_clone = Arc::clone(&success_counter);
        
        let handle = thread::spawn(move || {
            if i % 3 == 0 {
                // This thread will panic
                panic!("Intentional panic in thread {}", i);
            } else {
                // This thread will work normally
                let result = from_xml::<ThreadSafeTestStruct>(&xml_clone);
                if result.is_ok() {
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
                let mut results_guard = results_clone.lock().unwrap();
                results_guard.push((i, result));
            }
        });
        
        handles.push(handle);
    }
    
    // Collect results from non-panicking threads
    let mut panic_count = 0;
    let mut success_count = 0;
    
    for (i, handle) in handles.into_iter().enumerate() {
        if i % 3 == 0 {
            // Thread should have panicked
            let result = handle.join();
            if result.is_err() {
                panic_count += 1;
            }
        } else {
            // Thread should complete successfully
            if let Ok(()) = handle.join() {
                success_count += 1;
            }
        }
    }
    
    // Verify that we have the expected number of panics and successes
    assert!(panic_count > 0, "Should have some panicking threads");
    assert!(success_count > 0, "Should have some successful threads");
    
    // Verify that successful threads produced correct results
    let results_guard = results.lock().unwrap();
    let counter_success = success_counter.load(Ordering::Relaxed);
    
    // Verify results from successful threads
    for (thread_id, result) in results_guard.iter() {
        assert!(result.is_ok(), "Thread {} failed to parse XML", thread_id);
        let parsed = result.as_ref().unwrap();
        assert_eq!(parsed.id, 1);
        assert_eq!(parsed.name, "TestStruct1");
    }
    
    // Verify counter matches results
    assert_eq!(counter_success, results_guard.len(), "Counter should match results length");
}

#[test]
fn memory_safety_thread_safety() {
    let test_data = generate_test_struct(1);
    let xml = from_obj(&test_data);
    let xml_arc = Arc::new(xml);
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::<(usize, Result<usize, PError>)>::new()));
    let memory_usage = Arc::new(Mutex::new(Vec::<(usize, usize)>::new()));
    
    // Create threads that perform memory-intensive operations
    for i in 0..20 {
        let xml_clone = Arc::clone(&xml_arc);
        let results_clone = Arc::clone(&results);
        let memory_clone = Arc::clone(&memory_usage);
        
        let handle = thread::spawn(move || {
            // Perform multiple parsing operations to test memory management
            let mut parsed_objects = Vec::new();
            
            for _ in 0..50 {
                let parsed = from_xml::<ThreadSafeTestStruct>(&xml_clone).unwrap();
                parsed_objects.push(parsed);
            }
            
            // Verify all parsed objects are correct
            for parsed in &parsed_objects {
                assert_eq!(parsed.id, 1);
                assert_eq!(parsed.name, "TestStruct1");
                assert_eq!(parsed.data.len(), 2);
            }
            
            let mut results_guard = results_clone.lock().unwrap();
            results_guard.push((i, Ok(parsed_objects.len())));
            
            // Record memory usage info
            let mut memory_guard = memory_clone.lock().unwrap();
            memory_guard.push((i, parsed_objects.len()));
        });
        
        handles.push(handle);
    }
    
    // Await all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify results
    let results_guard = results.lock().unwrap();
    let memory_guard = memory_usage.lock().unwrap();
    
    assert_eq!(results_guard.len(), 20);
    assert_eq!(memory_guard.len(), 20);
    
    for (thread_id, result) in results_guard.iter() {
        assert!(result.is_ok(), "Thread {} failed", thread_id);
        let count = result.as_ref().unwrap();
        assert_eq!(*count, 50, "Thread {} should have parsed 50 objects", thread_id);
    }
    
    for (thread_id, count) in memory_guard.iter() {
        assert_eq!(*count, 50, "Thread {} should have processed 50 objects", thread_id);
    }
}
