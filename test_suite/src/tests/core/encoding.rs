use xavier::{cdata, decode, encode, from_obj, from_xml, XmlSerializable, XmlDeserializable, PError};

#[test]
fn test_encoding_cases_basic_roundtrip() -> Result<(), PError> {

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestEncodingBasic {
        pub id: u32,
        pub name: String,
        pub description: String,
    }

    let test_data = TestEncodingBasic {
        id: 1,
        name: "Test Encoding".to_string(),
        description: encode!("Description with & < > \" ' characters"),
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestEncodingBasic>"));
    assert!(xml.contains("</TestEncodingBasic>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Encoding</name>"));
    assert!(xml.contains("&amp;"));
    assert!(xml.contains("&lt;"));
    assert!(xml.contains("&gt;"));
    assert!(xml.contains("&quot;"));
    assert!(xml.contains("&apos;"));
    
    let parsed: TestEncodingBasic = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    assert_eq!(test_data.description, encode!(&parsed.description));
    
    Ok(())
}

#[test]
fn test_encoding_cases_encode_function() -> Result<(), PError> {
    let should = r#"Some text &amp; others"#;
    assert_eq!(encode!("Some text & others"), should);

    let should = r#"<![CDATA[Some text &amp; others]]>"#;
    assert_eq!(cdata!("Some text &amp; others"), should);
    
    Ok(())
}

#[test]
fn test_encoding_cases_decode_function() -> Result<(), PError> {
    let encoded = "Test &amp; &gt; &lt;";
    let decoded = "Test & > <";
    assert_eq!(decoded, decode!(encoded));
    
    Ok(())
}

#[test]
fn test_encoding_cases_cdata_roundtrip() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestEncodingCdata {
        pub id: u32,
        pub title: String,
        pub content: String,
        pub code: String,
    }

    let test_data = TestEncodingCdata {
        id: 1,
        title: "CDATA Test".to_string(),
        content: "Normal content".to_string(),
        code: cdata!("<script>alert('test')</script>".to_string()),
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestEncodingCdata>"));
    assert!(xml.contains("</TestEncodingCdata>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<title>CDATA Test</title>"));
    assert!(xml.contains("<content>Normal content</content>"));
    assert!(xml.contains("<code>"));
    
    let parsed: TestEncodingCdata = from_xml(&xml)?;
    let with_cdata = TestEncodingCdata { 
        id: parsed.id, 
        title: parsed.title, 
        content: parsed.content, 
        code: cdata!(parsed.code) 
    };
    assert_eq!(test_data, with_cdata);
    
    Ok(())
}

#[test]
fn test_encoding_cases_mixed_encoding() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestEncodingMixed {
        pub id: u32,
        pub normal_text: String,
        pub encoded_text: String,
        pub cdata_text: String,
    }

    let test_data = TestEncodingMixed {
        id: 1,
        normal_text: "Normal text".to_string(),
        encoded_text: encode!("Text with & < > \" ' characters"),
        cdata_text: cdata!("<![CDATA[CDATA content]]>".to_string()),
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestEncodingMixed>"));
    assert!(xml.contains("</TestEncodingMixed>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<normal_text>Normal text</normal_text>"));
    assert!(xml.contains("&amp;"));
    assert!(xml.contains("&lt;"));
    assert!(xml.contains("&gt;"));
    assert!(xml.contains("<cdata_text>"));
    
    let parsed: TestEncodingMixed = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.normal_text, parsed.normal_text);
    assert_eq!(test_data.encoded_text, encode!(&parsed.encoded_text));
    assert_eq!(test_data.cdata_text, cdata!(parsed.cdata_text));
    
    Ok(())
}