use xavier::{from_obj, XmlSerializable};

#[derive(XmlSerializable)]
struct TestSerializationStruct {
    pub id: u32,
    pub name: String,
    pub data: Vec<String>,
}

#[test]
fn serialization_error_invalid_data() {
    // Test serialization with potentially problematic data
    let test_data = TestSerializationStruct {
        id: 123,
        name: "Test Name".to_string(),
        data: vec!["value1".to_string(), "value2".to_string()],
    };
    
    // This should not panic or cause errors
    let result = std::panic::catch_unwind(|| {
        from_obj(&test_data)
    });
    
    assert!(result.is_ok());
}

#[test]
fn serialization_error_empty_strings() {
    let test_data = TestSerializationStruct {
        id: 456,
        name: "".to_string(), // Empty string
        data: vec![], // Empty vector
    };
    
    let result = std::panic::catch_unwind(|| {
        from_obj(&test_data)
    });
    
    assert!(result.is_ok());
}

#[test]
fn serialization_error_special_characters() {
    let test_data = TestSerializationStruct {
        id: 789,
        name: "Special & < > \" ' chars".to_string(),
        data: vec!["value & < > \" '".to_string()],
    };
    
    let result = std::panic::catch_unwind(|| {
        from_obj(&test_data)
    });
    
    assert!(result.is_ok());
}

#[test]
fn serialization_error_unicode_characters() {
    let test_data = TestSerializationStruct {
        id: 101,
        name: "Unicode: 🚀 🌟 💫".to_string(),
        data: vec!["Emoji: 🎉 🎊 🎈".to_string()],
    };
    
    let result = std::panic::catch_unwind(|| {
        from_obj(&test_data)
    });
    
    assert!(result.is_ok());
}

#[test]
fn serialization_error_very_long_strings() {
    let long_string = "A".repeat(10000);
    let long_data = vec!["B".repeat(5000), "C".repeat(5000)];
    
    let test_data = TestSerializationStruct {
        id: 202,
        name: long_string,
        data: long_data,
    };
    
    let result = std::panic::catch_unwind(|| {
        from_obj(&test_data)
    });
    
    assert!(result.is_ok());
}

#[derive(XmlSerializable)]
struct TestNestedSerializationStruct {
    pub id: u32,
    pub nested: TestSerializationStruct,
}

#[test]
fn serialization_error_nested_structures() {
    let nested_data = TestSerializationStruct {
        id: 303,
        name: "Nested".to_string(),
        data: vec!["nested_value".to_string()],
    };
    
    let test_data = TestNestedSerializationStruct {
        id: 404,
        nested: nested_data,
    };
    
    let result = std::panic::catch_unwind(|| {
        from_obj(&test_data)
    });
    
    assert!(result.is_ok());
}

#[derive(XmlSerializable)]
struct TestCircularReference {
    pub id: u32,
    pub name: String,
}

#[test]
fn serialization_error_circular_references() {
    let test_data = TestCircularReference {
        id: 505,
        name: "Circular".to_string(),
    };
    
    let result = std::panic::catch_unwind(|| {
        from_obj(&test_data)
    });
    
    assert!(result.is_ok());
} 