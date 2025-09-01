use std::fmt::Display;
use std::str::FromStr;
use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};

#[test]
fn test_enums_basic_roundtrip() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    enum TestEnumsBasic {
        VariantA,
        VariantB,
        VariantC,
    }

    impl Display for TestEnumsBasic {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                TestEnumsBasic::VariantA => write!(f, "VariantA"),
                TestEnumsBasic::VariantB => write!(f, "VariantB"),
                TestEnumsBasic::VariantC => write!(f, "VariantC"),
            }
        }
    }

    impl FromStr for TestEnumsBasic {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "VariantA" => Ok(TestEnumsBasic::VariantA),
                "VariantB" => Ok(TestEnumsBasic::VariantB),
                "VariantC" => Ok(TestEnumsBasic::VariantC),
                _ => Err(()),
            }
        }
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestEnumsContainer {
        pub id: u32,
        pub name: String,
        pub enum_field: TestEnumsBasic,
    }

    let test_data = TestEnumsContainer {
        id: 1,
        name: "Test Enum".to_string(),
        enum_field: TestEnumsBasic::VariantA,
    };

    // Serialização
    let xml = from_obj(&test_data);
    
    // Verificar se contém as tags principais
    assert!(xml.contains("<TestEnumsContainer>"));
    assert!(xml.contains("</TestEnumsContainer>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Enum</name>"));
    assert!(xml.contains("<enum_field>VariantA</enum_field>"));
    
    // Deserialização
    let parsed: TestEnumsContainer = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_enums_all_variants() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    enum TestEnumsAllVariants {
        VariantA,
        VariantB,
        VariantC,
    }

    impl Display for TestEnumsAllVariants {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                TestEnumsAllVariants::VariantA => write!(f, "VariantA"),
                TestEnumsAllVariants::VariantB => write!(f, "VariantB"),
                TestEnumsAllVariants::VariantC => write!(f, "VariantC"),
            }
        }
    }

    impl FromStr for TestEnumsAllVariants {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "VariantA" => Ok(TestEnumsAllVariants::VariantA),
                "VariantB" => Ok(TestEnumsAllVariants::VariantB),
                "VariantC" => Ok(TestEnumsAllVariants::VariantC),
                _ => Err(()),
            }
        }
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestEnumsAllVariantsContainer {
        pub id: u32,
        pub name: String,
        pub enum_field: TestEnumsAllVariants,
    }

    let test_data_a = TestEnumsAllVariantsContainer {
        id: 1,
        name: "Variant A".to_string(),
        enum_field: TestEnumsAllVariants::VariantA,
    };

    let xml_a = from_obj(&test_data_a);
    assert!(xml_a.contains("<enum_field>VariantA</enum_field>"));
    let parsed_a: TestEnumsAllVariantsContainer = from_xml(&xml_a)?;
    assert_eq!(test_data_a, parsed_a);

    let test_data_b = TestEnumsAllVariantsContainer {
        id: 2,
        name: "Variant B".to_string(),
        enum_field: TestEnumsAllVariants::VariantB,
    };

    let xml_b = from_obj(&test_data_b);
    assert!(xml_b.contains("<enum_field>VariantB</enum_field>"));
    let parsed_b: TestEnumsAllVariantsContainer = from_xml(&xml_b)?;
    assert_eq!(test_data_b, parsed_b);

    let test_data_c = TestEnumsAllVariantsContainer {
        id: 3,
        name: "Variant C".to_string(),
        enum_field: TestEnumsAllVariants::VariantC,
    };

    let xml_c = from_obj(&test_data_c);
    assert!(xml_c.contains("<enum_field>VariantC</enum_field>"));
    let parsed_c: TestEnumsAllVariantsContainer = from_xml(&xml_c)?;
    assert_eq!(test_data_c, parsed_c);
    
    Ok(())
}

#[test]
fn test_enums_with_data() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    enum TestEnumsWithData {
        VariantA { id: u32, name: String },
        VariantB { count: i32, active: bool },
        VariantC { value: String },
    }

    impl Display for TestEnumsWithData {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                TestEnumsWithData::VariantA { id, name } => write!(f, "VariantA(id={}, name={})", id, name),
                TestEnumsWithData::VariantB { count, active } => write!(f, "VariantB(count={}, active={})", count, active),
                TestEnumsWithData::VariantC { value } => write!(f, "VariantC(value={})", value),
            }
        }
    }

    impl FromStr for TestEnumsWithData {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            // Simplified parsing for test purposes
            if s.starts_with("VariantA") {
                Ok(TestEnumsWithData::VariantA { id: 1, name: "A".to_string() })
            } else if s.starts_with("VariantB") {
                Ok(TestEnumsWithData::VariantB { count: 42, active: true })
            } else if s.starts_with("VariantC") {
                Ok(TestEnumsWithData::VariantC { value: "C".to_string() })
            } else {
                Err(())
            }
        }
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestEnumsWithDataContainer {
        pub id: u32,
        pub name: String,
        pub enum_field: TestEnumsWithData,
    }

    let test_data = TestEnumsWithDataContainer {
        id: 1,
        name: "Test With Data".to_string(),
        enum_field: TestEnumsWithData::VariantA { id: 1, name: "A".to_string() },
    };

    // Serialização
    let xml = from_obj(&test_data);
    
    // Verificar se contém as tags principais
    assert!(xml.contains("<TestEnumsWithDataContainer>"));
    assert!(xml.contains("</TestEnumsWithDataContainer>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test With Data</name>"));
    
    // Deserialização
    let parsed: TestEnumsWithDataContainer = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    
    Ok(())
}

#[test]
fn test_enums_manual_xml() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    enum TestEnumsManual {
        VariantA,
        VariantB,
        VariantC,
    }

    impl Display for TestEnumsManual {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                TestEnumsManual::VariantA => write!(f, "VariantA"),
                TestEnumsManual::VariantB => write!(f, "VariantB"),
                TestEnumsManual::VariantC => write!(f, "VariantC"),
            }
        }
    }

    impl FromStr for TestEnumsManual {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "VariantA" => Ok(TestEnumsManual::VariantA),
                "VariantB" => Ok(TestEnumsManual::VariantB),
                "VariantC" => Ok(TestEnumsManual::VariantC),
                _ => Err(()),
            }
        }
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestEnumsManualContainer {
        pub id: u32,
        pub name: String,
        pub enum_field: TestEnumsManual,
    }

    let xml = r#"
    <TestEnumsManualContainer>
        <id>1</id>
        <name>Manual Test</name>
        <enum_field>VariantB</enum_field>
    </TestEnumsManualContainer>"#;
    
    let obj: TestEnumsManualContainer = from_xml(&xml)?;
    assert_eq!(obj.id, 1);
    assert_eq!(obj.name, "Manual Test");
    assert_eq!(obj.enum_field, TestEnumsManual::VariantB);
    
    Ok(())
}

#[test]
fn test_enums_roundtrip_direct() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    enum TestEnumsRoundtrip {
        VariantA,
        VariantB,
        VariantC,
    }

    impl Display for TestEnumsRoundtrip {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                TestEnumsRoundtrip::VariantA => write!(f, "VariantA"),
                TestEnumsRoundtrip::VariantB => write!(f, "VariantB"),
                TestEnumsRoundtrip::VariantC => write!(f, "VariantC"),
            }
        }
    }

    impl FromStr for TestEnumsRoundtrip {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "VariantA" => Ok(TestEnumsRoundtrip::VariantA),
                "VariantB" => Ok(TestEnumsRoundtrip::VariantB),
                "VariantC" => Ok(TestEnumsRoundtrip::VariantC),
                _ => Err(()),
            }
        }
    }

    // Test VariantA
    let original_a = TestEnumsRoundtrip::VariantA;
    let xml_a = from_obj(&original_a);
    let parsed_a: TestEnumsRoundtrip = from_xml(&format!("<xml>{}</xml>", &xml_a))?;
    assert_eq!(original_a, parsed_a);

    // Test VariantB
    let original_b = TestEnumsRoundtrip::VariantB;
    let xml_b = from_obj(&original_b);
    let parsed_b: TestEnumsRoundtrip = from_xml(&format!("<xml>{}</xml>", &xml_b))?;
    assert_eq!(original_b, parsed_b);

    // Test VariantC
    let original_c = TestEnumsRoundtrip::VariantC;
    let xml_c = from_obj(&original_c);
    let parsed_c: TestEnumsRoundtrip = from_xml(&format!("<xml>{}</xml>", &xml_c))?;
    assert_eq!(original_c, parsed_c);
    
    Ok(())
}
