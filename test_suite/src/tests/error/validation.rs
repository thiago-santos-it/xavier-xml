use xavier::{from_obj, from_xml, PError, XmlDeserializable, XmlSerializable};

#[test]
fn test_error_handling_validation_duplicate_attributes() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingValidation {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let xml_duplicate_attrs = r#"
    <TestErrorHandlingValidation id="1" id="2">
        <name>John Doe</name>
        <email>john@example.com</email>
    </TestErrorHandlingValidation>
    "#;
    
    let result = from_xml::<TestErrorHandlingValidation>(xml_duplicate_attrs);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_error_handling_validation_invalid_namespace() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingValidation {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let xml_invalid_namespace = r#"
    <ns:TestErrorHandlingValidation xmlns:ns="invalid-namespace">
        <ns:id>1</ns:id>
        <ns:name>John Doe</ns:name>
        <ns:email>john@example.com</ns:email>
    </ns:TestErrorHandlingValidation>
    "#;
    
    let _result = from_xml::<TestErrorHandlingValidation>(xml_invalid_namespace);
    
    Ok(())
}

#[test]
fn test_error_handling_validation_invalid_attribute_value() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingValidation {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let xml_invalid_attr = r#"
    <TestErrorHandlingValidation id="invalid_id">
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
        <active>true</active>
        <score>95.5</score>
    </TestErrorHandlingValidation>
    "#;
    
    let result = from_xml::<TestErrorHandlingValidation>(xml_invalid_attr);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_error_handling_validation_invalid_tag_name() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingValidation {
        pub id: u64,
        pub name: String,
        pub email: Option<String>,
        pub tags: Vec<String>,
        pub active: bool,
        pub score: f64,
    }

    let xml_invalid_tag = r#"
    <InvalidTagName>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
        <active>true</active>
        <score>95.5</score>
    </InvalidTagName>
    "#;
    
    let result = from_xml::<TestErrorHandlingValidation>(xml_invalid_tag);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_error_handling_validation_invalid_enum_value() -> Result<(), PError> {
    use std::fmt::Display;
    use std::str::FromStr;

    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    enum TestErrorHandlingValidationEnum {
        VariantA,
        VariantB,
        VariantC,
    }

    impl Display for TestErrorHandlingValidationEnum {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                TestErrorHandlingValidationEnum::VariantA => write!(f, "VariantA"),
                TestErrorHandlingValidationEnum::VariantB => write!(f, "VariantB"),
                TestErrorHandlingValidationEnum::VariantC => write!(f, "VariantC"),
            }
        }
    }

    impl FromStr for TestErrorHandlingValidationEnum {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "VariantA" => Ok(TestErrorHandlingValidationEnum::VariantA),
                "VariantB" => Ok(TestErrorHandlingValidationEnum::VariantB),
                "VariantC" => Ok(TestErrorHandlingValidationEnum::VariantC),
                _ => Err(()),
            }
        }
    }

    #[derive(XmlDeserializable, Debug)]
    #[allow(dead_code)]
    struct TestErrorHandlingValidation {
        pub id: u64,
        pub name: String,
        pub enum_field: TestErrorHandlingValidationEnum,
    }

    let xml_invalid_enum = r#"
    <TestErrorHandlingValidation>
        <id>1</id>
        <name>John Doe</name>
        <enum_field>InvalidVariant</enum_field>
    </TestErrorHandlingValidation>
    "#;
    
    let result = from_xml::<TestErrorHandlingValidation>(xml_invalid_enum);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_error_handling_validation_malformed_inner_tags() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestErrorHandlingSpecificInner {
        pub id: u32,
        pub name: String,
        #[xml(inner="tagz")]
        pub tags: Option<Vec<String>>,
    }

    let xml_malformed_inner = r#"
    <TestErrorHandlingSpecificInner>
        <id>123</id>
        <name>Test</name>
        <tags>
            <wrong_tag>tag1</wrong_tag>
            <tagz>tag2</tagz>
        </tags>
    </TestErrorHandlingSpecificInner>"#;

    let result = from_xml::<TestErrorHandlingSpecificInner>(xml_malformed_inner);
    assert_eq!(result?.tags.unwrap(), vec!["tag2"]);

    Ok(())
}


#[test]
fn test_error_handling_serialization_invalid_data() -> Result<(), PError> {
    #[derive(XmlSerializable)]
    struct TestErrorHandlingSerialization {
        pub id: u32,
        pub name: String,
        pub data: Vec<String>,
    }

    // Test serialization with potentially problematic data
    let test_data = TestErrorHandlingSerialization {
        id: 123,
        name: "Test Name".to_string(),
        data: vec!["value1".to_string(), "value2".to_string()],
    };

    // This should not panic or cause errors
    let result = std::panic::catch_unwind(|| {
        from_obj(&test_data)
    });

    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_error_handling_serialization_empty_strings() -> Result<(), PError> {
    #[derive(XmlSerializable)]
    struct TestErrorHandlingSerialization {
        pub id: u32,
        pub name: String,
        pub data: Vec<String>,
    }

    let test_data = TestErrorHandlingSerialization {
        id: 456,
        name: "".to_string(), // Empty string
        data: vec![], // Empty vector
    };

    let result = std::panic::catch_unwind(|| {
        from_obj(&test_data)
    });

    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_error_handling_serialization_special_characters() -> Result<(), PError> {
    #[derive(XmlSerializable)]
    struct TestErrorHandlingSerialization {
        pub id: u32,
        pub name: String,
        pub data: Vec<String>,
    }

    let test_data = TestErrorHandlingSerialization {
        id: 789,
        name: "Special & < > \" ' chars".to_string(),
        data: vec!["value & < > \" '".to_string()],
    };

    let result = std::panic::catch_unwind(|| {
        from_obj(&test_data)
    });

    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_error_handling_serialization_unicode_characters() -> Result<(), PError> {
    #[derive(XmlSerializable)]
    struct TestErrorHandlingSerialization {
        pub id: u32,
        pub name: String,
        pub data: Vec<String>,
    }

    let test_data = TestErrorHandlingSerialization {
        id: 101,
        name: "Unicode: 🚀 🌟 💫".to_string(),
        data: vec!["Emoji: 🎉 🎊 🎈".to_string()],
    };

    let result = std::panic::catch_unwind(|| {
        from_obj(&test_data)
    });

    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_error_handling_serialization_very_long_strings() -> Result<(), PError> {
    #[derive(XmlSerializable)]
    struct TestErrorHandlingSerialization {
        pub id: u32,
        pub name: String,
        pub data: Vec<String>,
    }

    let long_string = "A".repeat(10000);
    let long_data = vec!["B".repeat(5000), "C".repeat(5000)];

    let test_data = TestErrorHandlingSerialization {
        id: 202,
        name: long_string,
        data: long_data,
    };

    let result = std::panic::catch_unwind(|| {
        from_obj(&test_data)
    });

    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_error_handling_serialization_nested_structures() -> Result<(), PError> {
    #[derive(XmlSerializable)]
    struct TestErrorHandlingSerializationChild {
        pub id: u32,
        pub name: String,
        pub data: Vec<String>,
    }

    #[derive(XmlSerializable)]
    struct TestErrorHandlingSerializationNested {
        pub id: u32,
        pub nested: TestErrorHandlingSerializationChild,
    }

    let nested_data = TestErrorHandlingSerializationChild {
        id: 303,
        name: "Nested".to_string(),
        data: vec!["nested_value".to_string()],
    };

    let test_data = TestErrorHandlingSerializationNested {
        id: 404,
        nested: nested_data,
    };

    let result = std::panic::catch_unwind(|| {
        from_obj(&test_data)
    });

    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_error_handling_serialization_circular_references() -> Result<(), PError> {
    #[derive(XmlSerializable)]
    struct TestErrorHandlingSerializationCircular {
        pub id: u32,
        pub name: String,
        pub reference: Option<Box<TestErrorHandlingSerializationCircular>>,
    }

    let test_data = TestErrorHandlingSerializationCircular {
        id: 505,
        name: "Circular".to_string(),
        reference: None, 
    };

    let result = std::panic::catch_unwind(|| {
        from_obj(&test_data)
    });

    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_error_handling_serialization_large_numbers() -> Result<(), PError> {
    #[derive(XmlSerializable)]
    struct TestErrorHandlingSerializationLarge {
        pub id: u64,
        pub large_number: u128,
        pub negative_number: i64,
        pub float_number: f64,
    }

    let test_data = TestErrorHandlingSerializationLarge {
        id: 18446744073709551615, 
        large_number: 340282366920938463463374607431768211455, 
        negative_number: -9223372036854775808, 
        float_number: std::f64::MAX,
    };

    let result = std::panic::catch_unwind(|| {
        from_obj(&test_data)
    });

    assert!(result.is_ok());

    Ok(())
}