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
fn error_missing_required_field_name() {
    let xml_missing_field = r#"
    <ErrorTestStruct>
        <id>1</id>
        <!-- Missing name field -->
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
        <active>true</active>
        <score>95.5</score>
    </ErrorTestStruct>
    "#;
    
    let result = from_xml::<ErrorTestStruct>(xml_missing_field);
    assert!(result.is_err());
}

#[test]
fn error_missing_required_field_id() {
    let xml_missing_field = r#"
    <ErrorTestStruct>
        <!-- Missing id field -->
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
        <active>true</active>
        <score>95.5</score>
    </ErrorTestStruct>
    "#;
    
    let result = from_xml::<ErrorTestStruct>(xml_missing_field);
    assert!(result.is_err());
}

#[test]
fn error_missing_required_field_active() {
    let xml_missing_field = r#"
    <ErrorTestStruct>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
        <!-- Missing active field -->
        <score>95.5</score>
    </ErrorTestStruct>
    "#;
    
    let result = from_xml::<ErrorTestStruct>(xml_missing_field);
    assert!(result.is_err());
}

#[test]
fn error_missing_required_field_score() {
    let xml_missing_field = r#"
    <ErrorTestStruct>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
        <active>true</active>
        <!-- Missing score field -->
    </ErrorTestStruct>
    "#;
    
    let result = from_xml::<ErrorTestStruct>(xml_missing_field);
    assert!(result.is_err());
}

#[test]
fn error_missing_multiple_required_fields() {
    let xml_missing_fields = r#"
    <ErrorTestStruct>
        <id>1</id>
        <!-- Missing name, active, and score fields -->
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
    </ErrorTestStruct>
    "#;
    
    let result = from_xml::<ErrorTestStruct>(xml_missing_fields);
    assert!(result.is_err());
} 