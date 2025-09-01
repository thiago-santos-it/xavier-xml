use xavier::{from_xml, PError, XmlDeserializable};

#[test]
fn test_security_injection_xss_script_tags() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug, PartialEq)]
    struct TestSecurityInjection {
        pub content: String,
        pub user_input: String,
    }

    let malicious_xml = r#"
    <TestSecurityInjection>
        <content>Normal content</content>
        <user_input><![CDATA[<script>alert('XSS Attack')</script>]]></user_input>
    </TestSecurityInjection>
    "#;

    let result = from_xml::<TestSecurityInjection>(&malicious_xml);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert!(parsed.user_input.contains("<script>"));
    assert!(parsed.user_input.contains("alert('XSS Attack')"));
    
    Ok(())
}

#[test]
fn test_security_injection_sql_injection_quotes() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug, PartialEq)]
    struct TestSecurityInjection {
        pub query: String,
        pub user_id: String,
    }

    let malicious_xml = r#"
    <TestSecurityInjection>
        <query>SELECT * FROM users</query>
        <user_id><![CDATA[' OR '1'='1]]></user_id>
    </TestSecurityInjection>
    "#;

    let result = from_xml::<TestSecurityInjection>(&malicious_xml);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert!(parsed.user_id.contains("OR '1'='1"));
    
    Ok(())
}

#[test]
fn test_security_injection_xml_injection_nested() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug, PartialEq)]
    struct TestSecurityInjection {
        pub content: String,
        pub data: String,
    }

    let malicious_xml = r#"
    <TestSecurityInjection>
        <content>Safe content</content>
        <data><![CDATA[<injected><nested><malicious>data</malicious></nested></injected>]]></data>
    </TestSecurityInjection>
    "#;

    let result = from_xml::<TestSecurityInjection>(&malicious_xml);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert!(parsed.data.contains("<injected>"));
    assert!(parsed.data.contains("<nested>"));
    assert!(parsed.data.contains("<malicious>"));
    
    Ok(())
}

#[test]
fn test_security_injection_command_injection() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug, PartialEq)]
    struct TestSecurityInjection {
        pub command: String,
        pub parameter: String,
    }

    let malicious_xml = r#"
    <TestSecurityInjection>
        <command>ls</command>
        <parameter><![CDATA[; rm -rf /]]></parameter>
    </TestSecurityInjection>
    "#;

    let result = from_xml::<TestSecurityInjection>(&malicious_xml);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert!(parsed.parameter.contains("; rm -rf /"));
    
    Ok(())
}

#[test]
fn test_security_injection_path_traversal() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug, PartialEq)]
    struct TestSecurityInjection {
        pub filename: String,
        pub path: String,
    }

    let malicious_xml = r#"
    <TestSecurityInjection>
        <filename>document.txt</filename>
        <path><![CDATA[../../../etc/passwd]]></path>
    </TestSecurityInjection>
    "#;

    let result = from_xml::<TestSecurityInjection>(&malicious_xml);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert!(parsed.path.contains("../../../etc/passwd"));
    
    Ok(())
}

#[test]
fn test_security_injection_html_injection() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug, PartialEq)]
    struct TestSecurityInjection {
        pub title: String,
        pub body: String,
    }

    let malicious_xml = r#"
    <TestSecurityInjection>
        <title>Normal Title</title>
        <body><![CDATA[<img src="x" onerror="alert('XSS')">]]></body>
    </TestSecurityInjection>
    "#;

    let result = from_xml::<TestSecurityInjection>(&malicious_xml);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert!(parsed.body.contains("<img"));
    assert!(parsed.body.contains("onerror"));
    assert!(parsed.body.contains("alert('XSS')"));
    
    Ok(())
}
