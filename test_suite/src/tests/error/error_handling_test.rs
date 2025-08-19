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
fn error_malformed_xml() {
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
fn error_incompatible_types() {
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
fn error_missing_required_field() {
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
fn error_invalid_namespace() {
    let xml_invalid_namespace = r#"
    <ns:ErrorTestStruct xmlns:ns="invalid-namespace">
        <ns:id>1</ns:id>
        <ns:name>John Doe</ns:name>
        <ns:email>john@example.com</ns:email>
    </ns:ErrorTestStruct>
    "#;
    
    let result = from_xml::<ErrorTestStruct>(xml_invalid_namespace);
    // Pode ser ok ou err, dependendo da implementação
    println!("Result: {:?}", result);
}

#[test]
fn error_duplicate_attributes() {
    let xml_duplicate_attrs = r#"
    <ErrorTestStruct id="1" id="2">
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
        <active>true</active>
        <score>95.5</score>
    </ErrorTestStruct>
    "#;
    
    let result = from_xml::<ErrorTestStruct>(xml_duplicate_attrs);
    // Deve falhar ou ser rejeitado
    assert!(result.is_err());
}

#[test]
fn error_invalid_boolean() {
    let xml_invalid_bool = r#"
    <ErrorTestStruct>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
        <active>maybe</active>
        <score>95.5</score>
    </ErrorTestStruct>
    "#;
    
    let result = from_xml::<ErrorTestStruct>(xml_invalid_bool);
    assert!(result.is_err());
}

#[test]
fn error_invalid_float() {
    let xml_invalid_float = r#"
    <ErrorTestStruct>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
        <active>true</active>
        <score>not_a_number</score>
    </ErrorTestStruct>
    "#;
    
    let result = from_xml::<ErrorTestStruct>(xml_invalid_float);
    assert!(result.is_err());
}

#[test]
fn error_overflow_integer() {
    let xml_overflow = r#"
    <ErrorTestStruct>
        <id>18446744073709551616</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
        <active>true</active>
        <score>95.5</score>
    </ErrorTestStruct>
    "#;
    
    let result = from_xml::<ErrorTestStruct>(xml_overflow);
    assert!(result.is_err());
}

#[test]
fn error_empty_xml() {
    let empty_xml = "";
    let result = from_xml::<ErrorTestStruct>(empty_xml);
    assert!(result.is_err());
}

#[test]
fn error_only_whitespace() {
    let whitespace_xml = "   \n\t   ";
    let result = from_xml::<ErrorTestStruct>(whitespace_xml);
    assert!(result.is_err());
}
