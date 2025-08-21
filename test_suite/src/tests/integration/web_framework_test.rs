use xavier::{from_xml, from_obj, XmlDeserializable, XmlSerializable, PError};

#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="user")]
struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="response")]
struct ApiResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<User>,
}

// API endpoint simulation
fn test_xml_endpoint(user_xml: &str) -> Result<String, String> {
    match from_xml::<User>(user_xml) {
        Ok(user) => {
            let response = ApiResponse {
                success: true,
                message: "User processed successfully".to_string(),
                data: Some(user),
            };
            Ok(from_obj(&response))
        }
        Err(_) => {
            let response = ApiResponse {
                success: false,
                message: "Invalid user data".to_string(),
                data: None,
            };
            Ok(from_obj(&response))
        }
    }
}

#[test]
fn integration_web_framework_success() {
    let user_xml = r#"
    <user>
        <id>1</id>
        <name>John Doe</name>
        <email>john@example.com</email>
    </user>
    "#;
    
    let result = test_xml_endpoint(user_xml);
    assert!(result.is_ok());
    
    let response_xml = result.unwrap();
    assert!(response_xml.contains("success"));
    assert!(response_xml.contains("User processed successfully"));
}

#[test]
fn integration_web_framework_error() {
    let invalid_xml = r#"
    <user>
        <id>invalid</id>
        <name>John Doe</name>
        <email>john@example.com</email>
    </user>
    "#;
    
    let result = test_xml_endpoint(invalid_xml);
    assert!(result.is_ok());
    
    let response_xml = result.unwrap();
    assert!(response_xml.contains("success"));
    assert!(response_xml.contains("Invalid user data"));
}

#[test]
fn integration_web_framework_roundtrip() {
    let user = User {
        id: 1,
        name: "Jane Doe".to_string(),
        email: "jane@example.com".to_string(),
    };
    
    let xml = from_obj(&user);
    let parsed: User = from_xml(&xml).unwrap();
    
    assert_eq!(parsed.id, user.id);
    assert_eq!(parsed.name, user.name);
    assert_eq!(parsed.email, user.email);
}

#[test]
fn integration_web_framework_content_type() {
    let user = User {
        id: 1,
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
    };
    
    let xml = from_obj(&user);
    
    // Simular resposta HTTP com content-type correto
    let http_response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/xml\r\nContent-Length: {}\r\n\r\n{}",
        xml.len(),
        xml
    );
    
    assert!(http_response.contains("application/xml"));
    assert!(http_response.contains(&xml));
} 