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
fn error_incompatible_types_string_to_number() {
    let xml_with_wrong_type = r#"
    <ErrorTestStruct>
        <id>not_a_number</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
            <tags>tag2</tags>
        </tags>
        <active>true</active>
        <score>95.5</score>
    </ErrorTestStruct>
    "#;
    
    let result = from_xml::<ErrorTestStruct>(xml_with_wrong_type);
    assert!(result.is_err());
}

#[test]
fn error_incompatible_types_string_to_bool() {
    let xml_with_wrong_type = r#"
    <ErrorTestStruct>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
        <active>not_a_boolean</active>
        <score>95.5</score>
    </ErrorTestStruct>
    "#;
    
    let result = from_xml::<ErrorTestStruct>(xml_with_wrong_type);
    assert!(result.is_err());
}

#[test]
fn error_incompatible_types_string_to_float() {
    let xml_with_wrong_type = r#"
    <ErrorTestStruct>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
        <active>true</active>
        <score>not_a_float</score>
    </ErrorTestStruct>
    "#;
    
    let result = from_xml::<ErrorTestStruct>(xml_with_wrong_type);
    assert!(result.is_err());
}

#[test]
fn error_incompatible_types_invalid_collection() {
    let xml_with_wrong_type = r#"
    <ErrorTestStruct>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>not_a_collection</tags>
        <active>true</active>
        <score>95.5</score>
    </ErrorTestStruct>
    "#;
    
    let result = from_xml::<ErrorTestStruct>(xml_with_wrong_type);
    assert!(result.is_err());
} 