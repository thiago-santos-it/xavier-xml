use xavier::{from_xml, PError, XmlDeserializable};

#[test]
fn test_security_malicious_dangerous_characters() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug, PartialEq)]
    struct TestSecurityMalicious {
        pub content: String,
    }

    let malicious_xml = r#"
    <TestSecurityMalicious>
        <content><![CDATA[<script>alert('XSS')</script>]]></content>
    </TestSecurityMalicious>
    "#;

    let result = from_xml::<TestSecurityMalicious>(&malicious_xml);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert!(parsed.content.contains("<script>"));
    assert!(parsed.content.contains("alert('XSS')"));
    assert!(parsed.content.contains("</script>"));
    
    Ok(())
}

#[test]
fn test_security_malicious_sql_injection() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug, PartialEq)]
    struct TestSecurityMalicious {
        pub content: String,
        pub query: String,
    }

    let malicious_xml = r#"
    <TestSecurityMalicious>
        <content>User input</content>
        <query><![CDATA[SELECT * FROM users WHERE id = '1' OR '1'='1']]></query>
    </TestSecurityMalicious>
    "#;

    let result = from_xml::<TestSecurityMalicious>(&malicious_xml);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert!(parsed.query.contains("OR '1'='1"));
    
    Ok(())
}

#[test]
fn test_security_malicious_xml_injection() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug, PartialEq)]
    struct TestSecurityMalicious {
        pub content: String,
        pub data: String,
    }

    let malicious_xml = r#"
    <TestSecurityMalicious>
        <content>Normal content</content>
        <data><![CDATA[<malicious><tag>injected</tag></malicious>]]></data>
    </TestSecurityMalicious>
    "#;

    let result = from_xml::<TestSecurityMalicious>(&malicious_xml);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert!(parsed.data.contains("<malicious>"));
    assert!(parsed.data.contains("injected"));
    
    Ok(())
}

#[test]
fn test_security_malicious_unicode_control() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug, PartialEq)]
    struct TestSecurityMalicious {
        pub content: String,
    }

    let malicious_xml = r#"
    <TestSecurityMalicious>
        <content><![CDATA[Text with control characters: ]]></content>
    </TestSecurityMalicious>
    "#;

    let result = from_xml::<TestSecurityMalicious>(&malicious_xml);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert!(parsed.content.contains("control characters"));
    
    Ok(())
}

#[test]
fn test_security_malicious_entity_expansion() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug, PartialEq)]
    struct TestSecurityMalicious {
        pub content: String,
    }

    let malicious_xml = r#"
    <TestSecurityMalicious>
        <content>&lt;script&gt;alert('XSS')&lt;/script&gt;</content>
    </TestSecurityMalicious>
    "#;

    let result = from_xml::<TestSecurityMalicious>(&malicious_xml);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert!(parsed.content.contains("<script>"));
    assert!(parsed.content.contains("alert('XSS')"));
    assert!(parsed.content.contains("</script>"));
    
    Ok(())
}