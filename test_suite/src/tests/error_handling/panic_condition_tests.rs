use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
struct XMLObject {
    pub _some_string: String,
    pub _some_int: i32,
    pub _some_float: f32
}

#[test]
fn panic_condition_missing_required_field() {
    let xml = r#"
               <XMLObject>
                    <_some_int>10</_some_int>
                    <_some_float>11.1</_some_float>
               </XMLObject>"#;
    let result: Result<XMLObject, PError> = from_xml(&xml);
    if let Err(_error) = result {
        assert!(true)
    } else {
        assert!(false)
    }
}

#[test]
fn panic_condition_infinite_recursion() {
    let xml = r#"
    <XMLObject>
        <_some_string>Test</_some_string>
        <_some_int>10</_some_int>
        <_some_float>11.1</_some_float>
        <_some_string>Test2</_some_string>
    </XMLObject>"#;
    
    let result: Result<XMLObject, PError> = from_xml(&xml);
    // Should handle gracefully without infinite recursion
    assert!(result.is_err() || result.is_ok());
}

#[test]
fn panic_condition_malformed_nested_structure() {
    let xml = r#"
    <XMLObject>
        <_some_string>Test</_some_string>
        <_some_int>10</_some_int>
        <_some_float>11.1</_some_float>
        <nested>
            <_some_string>Nested</_some_string>
            <_some_int>20</_some_int>
            <_some_float>22.2</_some_float>
        </nested>
    </XMLObject>"#;
    
    let result: Result<XMLObject, PError> = from_xml(&xml);
    // Should handle gracefully without panic
    assert!(result.is_err() || result.is_ok());
}

#[test]
fn panic_condition_very_deep_nesting() {
    let mut xml = String::new();
    xml.push_str("<XMLObject>");
    
    // Create very deep nesting
    for i in 0..1000 {
        xml.push_str(&format!("<level{}>", i));
    }
    
    xml.push_str("<_some_string>Test</_some_string>");
    xml.push_str("<_some_int>10</_some_int>");
    xml.push_str("<_some_float>11.1</_some_float>");
    
    // Close all levels
    for i in (0..1000).rev() {
        xml.push_str(&format!("</level{}>", i));
    }
    
    xml.push_str("</XMLObject>");
    
    let result: Result<XMLObject, PError> = from_xml(&xml);
    // Should handle gracefully without stack overflow
    assert!(result.is_err() || result.is_ok());
} 