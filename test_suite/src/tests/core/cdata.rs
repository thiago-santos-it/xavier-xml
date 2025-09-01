use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError, cdata};

#[test]
fn test_cdata_and_comments_basic_roundtrip() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCdataAndCommentsBasic {
        pub id: u32,
        pub title: String,
        pub content: String,
        pub code: String,
    }

    let test_data = TestCdataAndCommentsBasic {
        id: 1,
        title: "Test Title".to_string(),
        content: "Normal content".to_string(),
        code: cdata!("<script>alert('test')</script>".to_string()),
    };
    
    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestCdataAndCommentsBasic>"));
    assert!(xml.contains("</TestCdataAndCommentsBasic>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<title>Test Title</title>"));
    assert!(xml.contains("<content>Normal content</content>"));
    assert!(xml.contains("<code>"));
    
    let parsed: TestCdataAndCommentsBasic = from_xml(&xml)?;
    let with_cdata = TestCdataAndCommentsBasic { 
        id: parsed.id, 
        title: parsed.title, 
        content: parsed.content, 
        code: cdata!(parsed.code) 
    };
    assert_eq!(test_data, with_cdata);
    
    Ok(())
}

#[test]
fn test_cdata_and_comments_manual_xml() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCdataAndCommentsManual {
        pub id: u32,
        pub title: String,
        pub content: String,
        pub code: String,
    }

    let manual_xml = r#"
    <TestCdataAndCommentsManual>
        <id>1</id>
        <title>Manual Test</title>
        <content>Manual content</content>
        <code><![CDATA[<script>alert('manual test')</script>]]></code>
    </TestCdataAndCommentsManual>"#;
    
    let parsed: TestCdataAndCommentsManual = from_xml(manual_xml)?;
    
    assert_eq!(parsed.id, 1);
    assert_eq!(parsed.title, "Manual Test");
    assert_eq!(parsed.content, "Manual content");
    assert_eq!(parsed.code, "<script>alert('manual test')</script>");
    
    Ok(())
}

#[test]
fn test_cdata_and_comments_special_characters() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCdataAndCommentsSpecialChars {
        pub id: u32,
        pub title: String,
        pub content: String,
        pub code: String,
    }

    let test_data = TestCdataAndCommentsSpecialChars {
        id: 1,
        title: cdata!("Special & Characters <Test>".to_string()),
        content: cdata!("Content with & < > \" ' characters".to_string()),
        code: cdata!("if (x < 10 && y > 20) { return \"value\"; }".to_string()),
    };
    
    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestCdataAndCommentsSpecialChars>"));
    assert!(xml.contains("</TestCdataAndCommentsSpecialChars>"));
    assert!(xml.contains("<id>1</id>"));
    
    let parsed: TestCdataAndCommentsSpecialChars = from_xml(&xml)?;
    assert_eq!(test_data.title, cdata!(parsed.title));
    assert_eq!(test_data.content, cdata!(parsed.content));
    assert_eq!(test_data.code, cdata!(parsed.code));
    
    Ok(())
}

#[test]
fn test_cdata_and_comments_processing() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCdataAndCommentsProcessing {
        pub id: u32,
        pub name: String,
        pub description: String,
    }

    let test_data = TestCdataAndCommentsProcessing {
        id: 1,
        name: "Test Name".to_string(),
        description: "Test Description".to_string(),
    };
    
    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestCdataAndCommentsProcessing>"));
    assert!(xml.contains("</TestCdataAndCommentsProcessing>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Name</name>"));
    assert!(xml.contains("<description>Test Description</description>"));
    
    let parsed: TestCdataAndCommentsProcessing = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_cdata_and_comments_mixed_content() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCdataAndCommentsMixed {
        pub id: u32,
        pub text: String,
        pub html: String,
        pub code: String,
    }

    let test_data = TestCdataAndCommentsMixed {
        id: 1,
        text: "Normal text content".to_string(),
        html: cdata!("<div>HTML content</div>".to_string()),
        code: cdata!("function test() { return true; }".to_string()),
    };
    
    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestCdataAndCommentsMixed>"));
    assert!(xml.contains("</TestCdataAndCommentsMixed>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<text>Normal text content</text>"));
    assert!(xml.contains("<html>"));
    assert!(xml.contains("<code>"));
    
    let parsed: TestCdataAndCommentsMixed = from_xml(&xml)?;
    assert_eq!(test_data.text, parsed.text);
    assert_eq!(test_data.html, cdata!(parsed.html));
    assert_eq!(test_data.code, cdata!(parsed.code));
    
    Ok(())
}

#[test]
fn test_cdata_and_comments_edge_cases() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestCdataAndCommentsEdgeCases {
        pub id: u32,
        pub empty_string: String,
        pub cdata_empty: String,
        pub special_chars: String,
    }

    let test_data = TestCdataAndCommentsEdgeCases {
        id: 1,
        empty_string: "".to_string(),
        cdata_empty: cdata!("".to_string()),
        special_chars: cdata!("]]>".to_string()),
    };
    
    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<TestCdataAndCommentsEdgeCases>"));
    assert!(xml.contains("</TestCdataAndCommentsEdgeCases>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<empty_string></empty_string>"));
    
    let parsed: TestCdataAndCommentsEdgeCases = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.empty_string, parsed.empty_string);
    assert_eq!(test_data.cdata_empty, cdata!(parsed.cdata_empty));
    assert_eq!(test_data.special_chars, cdata!(parsed.special_chars));
    
    Ok(())
} 