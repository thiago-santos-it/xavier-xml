use xavier::{encode, from_obj, from_xml, instructions, XmlSerializable, XmlDeserializable, PError};

#[test]
fn test_processing_instructions_basic_roundtrip() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable)]
    #[declaration]
    #[pi(instr test="some")]
    #[xml(name="test_processing")]
    struct TestProcessingBasic {
        pub id: u32,
        pub name: String,
        pub description: String,
    }

    let test_data = TestProcessingBasic {
        id: 1,
        name: encode!("Test Processing"),
        description: encode!("Test Description"),
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<test_processing>"));
    assert!(xml.contains("</test_processing>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Processing</name>"));
    assert!(xml.contains("<description>Test Description</description>"));
    
    instructions!(&xml, | _tag, instruction, params | {
        assert_eq!("test=\"some\"", params);
        assert_eq!("instr", instruction);
    })?;
    
    let parsed: TestProcessingBasic = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    assert_eq!(test_data.description, parsed.description);
    
    Ok(())
}

#[test]
fn test_processing_instructions_multiple_instructions() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable)]
    #[declaration]
    #[pi(instr test="some")]
    #[pi(instr test="some")]
    #[xml(name="test_processing_multiple")]
    struct TestProcessingMultiple {
        pub id: u32,
        pub name: String,
        #[pi(instr test="some")]
        pub some_int: i32,
        pub some_float: f32,
    }

    let test_data = TestProcessingMultiple {
        id: 1,
        name: encode!("Test Multiple Processing"),
        some_int: 42,
        some_float: 3.14,
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<test_processing_multiple>"));
    assert!(xml.contains("</test_processing_multiple>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Multiple Processing</name>"));
    assert!(xml.contains("<some_int>42</some_int>"));
    assert!(xml.contains("<some_float>3.14</some_float>"));
    
    instructions!(&xml, | _tag, instruction, params | {
        assert_eq!("test=\"some\"", params);
        assert_eq!("instr", instruction);
    })?;
    
    let parsed: TestProcessingMultiple = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    assert_eq!(test_data.some_int, parsed.some_int);
    assert_eq!(test_data.some_float, parsed.some_float);
    
    Ok(())
}

#[test]
fn test_processing_instructions_field_level() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable)]
    #[declaration]
    #[xml(name="test_processing_field")]
    struct TestProcessingField {
        pub id: u32,
        pub name: String,
        #[pi(instr test="field")]
        pub some_int: i32,
        pub some_float: f32,
    }

    let test_data = TestProcessingField {
        id: 1,
        name: encode!("Test Field Processing"),
        some_int: 42,
        some_float: 3.14,
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<test_processing_field>"));
    assert!(xml.contains("</test_processing_field>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Field Processing</name>"));
    assert!(xml.contains("<some_int>42</some_int>"));
    assert!(xml.contains("<some_float>3.14</some_float>"));
    
    instructions!(&xml, | _tag, instruction, params | {
        assert_eq!("test=\"field\"", params);
        assert_eq!("instr", instruction);
    })?;
    
    let parsed: TestProcessingField = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    assert_eq!(test_data.some_int, parsed.some_int);
    assert_eq!(test_data.some_float, parsed.some_float);
    
    Ok(())
}

#[test]
fn test_processing_instructions_complex_structures() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable)]
    struct TestProcessingChild {
        pub id: u32,
        pub name: String,
    }

    #[derive(XmlSerializable, XmlDeserializable)]
    #[declaration]
    #[pi(instr test="complex")]
    #[xml(name="test_processing_complex")]
    struct TestProcessingComplex {
        pub id: u32,
        pub name: String,
        pub children: Vec<TestProcessingChild>,
    }

    let test_data = TestProcessingComplex {
        id: 1,
        name: encode!("Test Complex Processing"),
        children: vec![
            TestProcessingChild { id: 1, name: "Child 1".to_string() },
            TestProcessingChild { id: 2, name: "Child 2".to_string() },
        ],
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<test_processing_complex>"));
    assert!(xml.contains("</test_processing_complex>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Complex Processing</name>"));
    assert!(xml.contains("<children>"));
    assert!(xml.contains("<TestProcessingChild>"));
    
    instructions!(&xml, | _tag, instruction, params | {
        assert_eq!("test=\"complex\"", params);
        assert_eq!("instr", instruction);
    })?;
    
    let parsed: TestProcessingComplex = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    assert_eq!(test_data.children.len(), parsed.children.len());
    
    Ok(())
}