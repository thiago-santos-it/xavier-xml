use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::fmt;
use std::str::FromStr;

#[test]
fn test_thread_safety_basic_parsing() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, Clone, PartialEq)]
    struct TestThreadSafetyBasic {
        pub id: u64,
        pub name: String,
        #[xml(inner="item")]
        pub data: Vec<String>,
    }

    fn generate_test_struct(id: u64) -> TestThreadSafetyBasic {
        TestThreadSafetyBasic {
            id,
            name: format!("TestStruct{}", id),
            data: vec![format!("data{}", id), format!("value{}", id)],
        }
    }

    let test_data = generate_test_struct(1);
    let xml = from_obj(&test_data);
    let xml_arc = Arc::new(xml);
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    let success_counter = Arc::new(AtomicUsize::new(0));
    
    for count in 0..20 {
        let xml_clone = Arc::clone(&xml_arc);
        let results_clone = Arc::clone(&results);
        let counter_clone = Arc::clone(&success_counter);
        
        let handle = thread::spawn(move || {
            let result = from_xml::<TestThreadSafetyBasic>(&xml_clone);
            if result.is_ok() {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
            let mut results_guard = results_clone.lock().unwrap();
            results_guard.push((count, result));
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let results_guard = results.lock().unwrap();
    assert_eq!(results_guard.len(), 20);
    
    let success_count = success_counter.load(Ordering::Relaxed);
    assert_eq!(success_count, 20, "All threads should succeed");
    
    for (thread_id, result) in results_guard.iter() {
        assert!(result.is_ok(), "Thread {} failed to parse XML", thread_id);
        let parsed = result.as_ref().unwrap();
        assert_eq!(parsed.id, 1);
        assert_eq!(parsed.name, "TestStruct1");
        assert_eq!(parsed.data.len(), 2);
    }
    
    Ok(())
}

#[test]
fn test_thread_safety_complex_nested_structures() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, Clone, PartialEq)]
    struct TestThreadSafetyChild {
        pub id: u64,
        pub name: String,
        #[xml(inner="item")]
        pub data: Vec<String>,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, Clone, PartialEq)]
    struct TestThreadSafetyComplex {
        pub id: u32,
        pub name: String,
        #[xml(inner="child")]
        pub children: Vec<TestThreadSafetyChild>,
    }

    fn generate_test_struct(id: u64) -> TestThreadSafetyChild {
        TestThreadSafetyChild {
            id,
            name: format!("TestStruct{}", id),
            data: vec![format!("data{}", id), format!("value{}", id)],
        }
    }

    fn generate_complex_struct(id: u32) -> TestThreadSafetyComplex {
        TestThreadSafetyComplex {
            id,
            name: format!("Complex{}", id),
            children: vec![
                generate_test_struct(1),
                generate_test_struct(2),
            ],
        }
    }

    let test_data = generate_complex_struct(1);
    let xml = from_obj(&test_data);
    let xml_arc = Arc::new(xml);
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    let success_counter = Arc::new(AtomicUsize::new(0));
    
    for count in 0..15 {
        let xml_clone = Arc::clone(&xml_arc);
        let results_clone = Arc::clone(&results);
        let counter_clone = Arc::clone(&success_counter);
        
        let handle = thread::spawn(move || {
            let result = from_xml::<TestThreadSafetyComplex>(&xml_clone);
            if result.is_ok() {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
            let mut results_guard = results_clone.lock().unwrap();
            results_guard.push((count, result));
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let results_guard = results.lock().unwrap();
    assert_eq!(results_guard.len(), 15);
    
    let success_count = success_counter.load(Ordering::Relaxed);
    assert_eq!(success_count, 15, "All threads should succeed");
    
    for (thread_id, result) in results_guard.iter() {
        assert!(result.is_ok(), "Thread {} failed to parse complex XML", thread_id);
        let parsed = result.as_ref().unwrap();
        assert_eq!(parsed.id, 1);
        assert_eq!(parsed.name, "Complex1");
        assert_eq!(parsed.children.len(), 2);
        assert_eq!(parsed.children[0].id, 1);
        assert_eq!(parsed.children[1].id, 2);
    }
    
    Ok(())
}

#[test]
fn test_thread_safety_enum_handling() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, Clone, PartialEq)]
    enum TestThreadSafetyEnum {
        VariantA,
        VariantB,
        VariantC,
    }

    impl fmt::Display for TestThreadSafetyEnum {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                TestThreadSafetyEnum::VariantA => write!(f, "VariantA"),
                TestThreadSafetyEnum::VariantB => write!(f, "VariantB"),
                TestThreadSafetyEnum::VariantC => write!(f, "VariantC"),
            }
        }
    }

    impl FromStr for TestThreadSafetyEnum {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "VariantA" => Ok(TestThreadSafetyEnum::VariantA),
                "VariantB" => Ok(TestThreadSafetyEnum::VariantB),
                "VariantC" => Ok(TestThreadSafetyEnum::VariantC),
                _ => Err(()),
            }
        }
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, Clone, PartialEq)]
    struct TestThreadSafetyEnumContainer {
        pub id: u32,
        pub variant: TestThreadSafetyEnum,
        pub name: String,
    }

    let test_data = TestThreadSafetyEnumContainer {
        id: 1,
        variant: TestThreadSafetyEnum::VariantB,
        name: "Enum Test".to_string(),
    };

    let xml = from_obj(&test_data);
    let xml_arc = Arc::new(xml);
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    let success_counter = Arc::new(AtomicUsize::new(0));

    for count in 0..12 {
        let xml_clone = Arc::clone(&xml_arc);
        let results_clone = Arc::clone(&results);
        let counter_clone = Arc::clone(&success_counter);
        
        let handle = thread::spawn(move || {
            let result = from_xml::<TestThreadSafetyEnumContainer>(&xml_clone);
            if result.is_ok() {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
            let mut results_guard = results_clone.lock().unwrap();
            results_guard.push((count, result));
        });
        
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let results_guard = results.lock().unwrap();
    assert_eq!(results_guard.len(), 12);
    
    let success_count = success_counter.load(Ordering::Relaxed);
    assert_eq!(success_count, 12, "All threads should succeed");
    
    for (thread_id, result) in results_guard.iter() {
        assert!(result.is_ok(), "Thread {} failed to parse enum XML", thread_id);
        let parsed = result.as_ref().unwrap();
        assert_eq!(parsed.id, 1);
        assert_eq!(parsed.name, "Enum Test");
        assert!(matches!(parsed.variant, TestThreadSafetyEnum::VariantB));
    }
    
    Ok(())
}

#[test]
fn test_thread_safety_mixed_operations() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, Clone, PartialEq)]
    struct TestThreadSafetyMixed {
        pub id: u64,
        pub name: String,
        pub count: i32,
        #[xml(inner="item")]
        pub items: Vec<String>,
        pub active: bool,
    }

    fn generate_mixed_test_data(id: u64) -> TestThreadSafetyMixed {
        TestThreadSafetyMixed {
            id,
            name: format!("MixedTest{}", id),
            count: id as i32,
            items: vec![format!("item{}", id), format!("value{}", id)],
            active: true,
        }
    }

    let test_data = generate_mixed_test_data(1);
    let data_arc = Arc::new(test_data);
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    let success_counter = Arc::new(AtomicUsize::new(0));
    
    for count in 0..10 {
        let data_clone = Arc::clone(&data_arc);
        let results_clone = Arc::clone(&results);
        let counter_clone = Arc::clone(&success_counter);
        
        let handle = thread::spawn(move || {

            let xml = from_obj(&*data_clone);
            let result = from_xml::<TestThreadSafetyMixed>(&xml);
            
            if result.is_ok() {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
            
            let mut results_guard = results_clone.lock().unwrap();
            results_guard.push((count, result));
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
        assert!(result.is_ok(), "Thread {} failed", thread_id);
        let parsed = result.as_ref().unwrap();
        assert_eq!(parsed.id, 1);
        assert_eq!(parsed.name, "MixedTest1");
        assert_eq!(parsed.count, 1);
        assert_eq!(parsed.items.len(), 2);
        assert_eq!(parsed.active, true);
    }
    
    Ok(())
}

#[test]
fn test_thread_safety_stress_test() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, Clone, PartialEq)]
    struct TestThreadSafetyStress {
        pub id: u64,
        pub name: String,
        #[xml(inner="item")]
        pub data: Vec<String>,
        #[xml(inner="item")]
        pub metadata: Vec<String>,
    }

    fn generate_stress_test_data(id: u64) -> TestThreadSafetyStress {
        let mut data = Vec::new();
        let mut metadata = Vec::new();
        
        for count in 0..50 {
            data.push(format!("data{}_{}", id, count));
            metadata.push(format!("meta{}_{}", id, count));
        }
        
        TestThreadSafetyStress {
            id,
            name: format!("StressTest{}", id),
            data,
            metadata,
        }
    }

    let test_data = generate_stress_test_data(1);
    let data_arc = Arc::new(test_data);
    
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::new()));
    let success_counter = Arc::new(AtomicUsize::new(0));

    for count in 0..25 {
        let data_clone = Arc::clone(&data_arc);
        let results_clone = Arc::clone(&results);
        let counter_clone = Arc::clone(&success_counter);
        
        let handle = thread::spawn(move || {
            let xml = from_obj(&*data_clone);
            let result = from_xml::<TestThreadSafetyStress>(&xml);
            
            if result.is_ok() {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
            
            let mut results_guard = results_clone.lock().unwrap();
            results_guard.push((count, result));
        });
        
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let results_guard = results.lock().unwrap();
    assert_eq!(results_guard.len(), 25);
    
    let success_count = success_counter.load(Ordering::Relaxed);
    assert_eq!(success_count, 25, "All stress test threads should succeed");
    
    for (thread_id, result) in results_guard.iter() {
        assert!(result.is_ok(), "Thread {} failed in stress test", thread_id);
        let parsed = result.as_ref().unwrap();
        assert_eq!(parsed.id, 1);
        assert_eq!(parsed.name, "StressTest1");
        assert_eq!(parsed.data.len(), 50);
        assert_eq!(parsed.metadata.len(), 50);
    }
    
    Ok(())
}
