use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
#[allow(dead_code)]
struct ErrorTestStruct {
    pub id: u64,
    pub name: String,
    pub email: Option<String>,
    pub tags: Vec<String>,
    pub active: bool,
    pub score: f64,
}

#[test]
fn error_malformed_xml_missing_closing_tag() {
    let malformed_xml = r#"
    <ErrorTestStruct>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <!-- Missing closing tag -->
    "#;
    
    let result = from_xml::<ErrorTestStruct>(malformed_xml);
    assert!(result.is_err());
}

#[test]
fn error_malformed_xml_unclosed_tag() {
    let malformed_xml = r#"
    <ErrorTestStruct>
        <id>1</id>
        <name>John Doe
        <email>john@example.com</email>
    </ErrorTestStruct>"#;
    
    let result = from_xml::<ErrorTestStruct>(malformed_xml);
    assert!(result.is_err());
}

#[test]
fn error_malformed_xml_invalid_nesting() {
    let malformed_xml = r#"
    <ErrorTestStruct>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tag>tag1</tag>
            <tag>tag2
        </tags>
        <active>true</active>
    </ErrorTestStruct>"#;
    
    let result = from_xml::<ErrorTestStruct>(malformed_xml);
    assert!(result.is_err());
}

#[test]
fn error_malformed_xml_empty_xml() {
    let empty_xml = "";
    let result = from_xml::<ErrorTestStruct>(empty_xml);
    // Empty XML should return error, not infinite loop
    assert!(result.is_err());
}

#[test]
fn error_malformed_xml_invalid_xml_syntax() {
    let invalid_xml = r#"
    <ErrorTestStruct>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tag>tag1</tag>
            <tag>tag2</tag>
        </tags>
        <active>true</active>
        <score>95.5</score>
    </ErrorTestStruct
    "#;
    
    let result = from_xml::<ErrorTestStruct>(invalid_xml);
    assert!(result.is_err());
} 