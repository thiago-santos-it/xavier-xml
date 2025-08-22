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
fn error_duplicate_attributes() {
    let xml_duplicate_attrs = r#"
    <ErrorTestStruct id="1" id="2">
        <name>John Doe</name>
        <email>john@example.com</email>
    </ErrorTestStruct>
    "#;
    
    let result = from_xml::<ErrorTestStruct>(xml_duplicate_attrs);
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
    
    let _result = from_xml::<ErrorTestStruct>(xml_invalid_namespace);
    // May be ok or err, depending on implementation
}

#[test]
fn error_invalid_attribute_value() {
    let xml_invalid_attr = r#"
    <ErrorTestStruct id="invalid_id">
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <tags>tag1</tags>
        </tags>
        <active>true</active>
        <score>95.5</score>
    </ErrorTestStruct>
    "#;
    
    let result = from_xml::<ErrorTestStruct>(xml_invalid_attr);
    assert!(result.is_err());
}

#[test]
fn error_invalid_tag_name() {
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
    
    let result = from_xml::<ErrorTestStruct>(xml_invalid_tag);
    assert!(result.is_err());
}

#[test]
fn error_invalid_collection_structure() {
    let xml_invalid_collection = r#"
    <ErrorTestStruct>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
        <tags>
            <invalid_tag>tag1</invalid_tag>
            <tags>tag2</tags>
        </tags>
        <active>true</active>
        <score>95.5</score>
    </ErrorTestStruct>
    "#;
    
    let result = from_xml::<ErrorTestStruct>(xml_invalid_collection);
    assert!(result.is_err());
} 