use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError, cdata};

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestCDATA {
    pub title: String,
    pub content: String,
    pub code: String,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestComments {
    pub id: u32,
    pub name: String,
    pub description: String,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestProcessingInstructions {
    pub id: u32,
    pub data: String,
}

#[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
struct TestMixedContent {
    pub id: u32,
    pub text: String,
    pub html: String,
}

#[test]
fn test_cdata_processing() -> Result<(), PError> {
    let test_data = TestCDATA {
        title: "Test Title".to_string(),
        content: "Normal content".to_string(),
        code: cdata!("<script>alert('test')</script>".to_string()),
    };
    
    let xml = from_obj(&test_data);
    
    let parsed: TestCDATA = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_cdata_manual_xml() -> Result<(), PError> {
    let manual_xml = r#"
    <TestCDATA>
        <title>Manual Test</title>
        <content>Manual content</content>
        <code><![CDATA[<script>alert('manual test')</script>]]></code>
    </TestCDATA>"#;
    
    let parsed: TestCDATA = from_xml(manual_xml)?;
    
    assert_eq!(parsed.title, "Manual Test");
    assert_eq!(parsed.content, "Manual content");
    assert_eq!(parsed.code, "<script>alert('manual test')</script>");
    
    Ok(())
}

#[test]
fn test_cdata_with_special_characters() -> Result<(), PError> {
    let test_data = TestCDATA {
        title: cdata!("Special & Characters <Test>".to_string()),
        content: cdata!("Content with & < > \" ' characters".to_string()),
        code: cdata!("if (x < 10 && y > 20) { return \"value\"; }".to_string()),
    };
    
    let xml = from_obj(&test_data);
    
    let parsed: TestCDATA = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_comments_processing() -> Result<(), PError> {
    let test_data = TestComments {
        id: 123,
        name: "Test Name".to_string(),
        description: "Test Description".to_string(),
    };
    
    let xml = from_obj(&test_data);
    
    let parsed: TestComments = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_comments_manual_xml() -> Result<(), PError> {
    let manual_xml = r#"
    <!-- This is a comment before the element -->
    <TestComments>
        <id>456</id>
        <!-- Comment inside element -->
        <name>Manual Test</name>
        <description>Manual description</description>
        <!-- Another comment -->
    </TestComments>
    <!-- Comment after the element -->"#;
    
    let parsed: TestComments = from_xml(manual_xml)?;
    
    assert_eq!(parsed.id, 456);
    assert_eq!(parsed.name, "Manual Test");
    assert_eq!(parsed.description, "Manual description");
    
    Ok(())
}

#[test]
fn test_processing_instructions() -> Result<(), PError> {
    let test_data = TestProcessingInstructions {
        id: 123,
        data: "Test data".to_string(),
    };
    
    let xml = from_obj(&test_data);
    
    let parsed: TestProcessingInstructions = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_processing_instructions_manual_xml() -> Result<(), PError> {
    let manual_xml = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <?xml-stylesheet type="text/xsl" href="style.xsl"?>
    <TestProcessingInstructions>
        <id>789</id>
        <data>Manual data</data>
    </TestProcessingInstructions>
    <?custom-pi some data?>"#;
    
    let parsed: TestProcessingInstructions = from_xml(manual_xml)?;
    
    assert_eq!(parsed.id, 789);
    assert_eq!(parsed.data, "Manual data");
    
    Ok(())
}

#[test]
fn test_mixed_content_with_cdata() -> Result<(), PError> {
    let test_data = TestMixedContent {
        id: 123,
        text: "Normal text content".to_string(),
        html: "<p>This is <strong>HTML</strong> content</p>".to_string(),
    };
    
    let xml = from_obj(&test_data);
    
    let parsed: TestMixedContent = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_cdata_edge_cases() -> Result<(), PError> {
    // CDATA vazio
    let empty_cdata_xml = r#"
    <TestCDATA>
        <title>Empty CDATA Test</title>
        <content>Content</content>
        <code><![CDATA[]]></code>
    </TestCDATA>"#;
    
    let parsed_empty: TestCDATA = from_xml(empty_cdata_xml)?;
    assert_eq!(parsed_empty.code, "");
    
    // CDATA with only whitespace
    let whitespace_cdata_xml = r#"
    <TestCDATA>
        <title>Whitespace CDATA Test</title>
        <content>Content</content>
        <code><![CDATA[   ]]></code>
    </TestCDATA>"#;
    
    let parsed_whitespace: TestCDATA = from_xml(whitespace_cdata_xml)?;
    assert_eq!(parsed_whitespace.code, "   ");
    
    Ok(())
}

#[test]
fn test_comments_edge_cases() -> Result<(), PError> {
            // Empty comments
    let empty_comments_xml = r#"
    <TestComments>
        <id>123</id>
        <name>Test</name>
        <description>Description</description>
    </TestComments>"#;
    
    let parsed: TestComments = from_xml(empty_comments_xml)?;
    assert_eq!(parsed.id, 123);
    
            // Comments with special characters
    let special_comments_xml = r#"
    <!-- Comment with & < > " ' characters -->
    <TestComments>
        <id>456</id>
        <name>Test</name>
        <description>Description</description>
    </TestComments>"#;
    
    let parsed_special: TestComments = from_xml(special_comments_xml)?;
    assert_eq!(parsed_special.id, 456);
    
    Ok(())
}

#[test]
fn test_malformed_cdata() {
    // CDATA malformado - sem fechamento
    let malformed_cdata_xml = r#"
    <TestCDATA>
        <title>Test</title>
        <content>Content</content>
        <code><![CDATA[<script>alert('test')</script>
    </TestCDATA>"#;
    
    let result = from_xml::<TestCDATA>(malformed_cdata_xml);
    assert!(result.is_err());
}

#[test]
fn test_nested_cdata_and_comments() -> Result<(), PError> {
    let test_data = TestMixedContent {
        id: 123,
        text: "Text with <!-- comment --> inside".to_string(),
        html: "<div><!-- HTML comment --><p>Content</p></div>".to_string(),
    };
    
    let xml = from_obj(&test_data);
    
    let parsed: TestMixedContent = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
} 