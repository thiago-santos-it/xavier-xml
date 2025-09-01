use xavier::{from_xml, PError, XmlDeserializable};
use std::time::Instant;

#[test]
fn test_security_resource_exhaustion_large_xml() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug, PartialEq)]
    struct TestSecurityResourceExhaustion {
        pub content: String,
        pub data: Vec<String>,
    }

    let mut large_xml = String::new();
    large_xml.push_str("<TestSecurityResourceExhaustion>");
    large_xml.push_str("<content>Large content</content>");
    large_xml.push_str("<data>");

    for i in 0..10000 {
        large_xml.push_str(&format!("<data>Item {}</data>", i));
    }

    large_xml.push_str("</data>");
    large_xml.push_str("</TestSecurityResourceExhaustion>");

    let start = Instant::now();
    let result = from_xml::<TestSecurityResourceExhaustion>(&large_xml);
    let duration = start.elapsed();

    assert!(duration < std::time::Duration::from_secs(10));
    assert!(result.is_ok());

    Ok(())
}


#[test]
fn test_security_resource_exhaustion_deep_nesting() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug, PartialEq)]
    struct TestSecurityResourceExhaustion {
        pub level: u32,
        pub content: String,
        #[xml(tree)]
        pub nested: Option<Box<TestSecurityResourceExhaustion>>,
    }

    let mut deep_xml = String::new();
    let depth = 100;

    deep_xml.push_str(&format!("<TestSecurityResourceExhaustion>"));
    for i in 0..depth {
        deep_xml.push_str(&format!("<level>{}</level>", i));
        deep_xml.push_str(&format!("<content>Level {}</content>", i));
        if i < depth - 1 {
            deep_xml.push_str("<nested>");
        }
    }
    for _ in 0..(depth - 1) {
        deep_xml.push_str("</nested>");
    }
    deep_xml.push_str("</TestSecurityResourceExhaustion>");

    let start = Instant::now();
    let result = from_xml::<TestSecurityResourceExhaustion>(&deep_xml);
    let duration = start.elapsed();
    assert!(duration < std::time::Duration::from_secs(5));
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn test_security_resource_exhaustion_memory_usage() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug, PartialEq)]
    struct TestSecurityResourceExhaustion {
        pub content: String,
        pub large_string: String,
    }

    let large_string = "A".repeat(100000); 
    let malicious_xml = format!(
        r#"<TestSecurityResourceExhaustion>
            <content>Normal content</content>
            <large_string>{}</large_string>
        </TestSecurityResourceExhaustion>"#,
        large_string
    );

    let start = Instant::now();
    let result = from_xml::<TestSecurityResourceExhaustion>(&malicious_xml);
    let duration = start.elapsed();
    
    assert!(duration < std::time::Duration::from_secs(3));
    assert!(result.is_ok());
    
    let parsed = result.unwrap();
    assert_eq!(parsed.large_string.len(), 100000);
    
    Ok(())
}

#[test]
fn test_security_resource_exhaustion_entity_expansion() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug, PartialEq)]
    struct TestSecurityResourceExhaustion {
        pub content: String,
    }

    let mut malicious_xml = String::new();
    malicious_xml.push_str("<TestSecurityResourceExhaustion>");
    malicious_xml.push_str("<content>");
    
    for _ in 0..10000 {
        malicious_xml.push_str("&lt;&gt;&amp;&quot;&apos;");
    }
    
    malicious_xml.push_str("</content>");
    malicious_xml.push_str("</TestSecurityResourceExhaustion>");

    let start = Instant::now();
    let result = from_xml::<TestSecurityResourceExhaustion>(&malicious_xml);
    let duration = start.elapsed();
    
    assert!(duration < std::time::Duration::from_secs(5));
    assert!(result.is_ok());
    
    Ok(())
}

#[test]
fn test_security_resource_exhaustion_repeated_tags() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug, PartialEq)]
    struct TestSecurityResourceExhaustion {
        pub items: Vec<String>,
    }

    let mut malicious_xml = String::new();
    malicious_xml.push_str("<TestSecurityResourceExhaustion>");
    malicious_xml.push_str("<items>");
    
    for i in 0..50000 {
        malicious_xml.push_str(&format!("<items>Item {}</items>", i));
    }
    
    malicious_xml.push_str("</items>");
    malicious_xml.push_str("</TestSecurityResourceExhaustion>");

    let start = Instant::now();
    let result = from_xml::<TestSecurityResourceExhaustion>(&malicious_xml);
    let duration = start.elapsed();
    
    assert!(duration < std::time::Duration::from_secs(10));
    assert!(result.is_ok());
    
    Ok(())
}

#[test]
fn test_security_resource_exhaustion_cdata_sections() -> Result<(), PError> {
    #[derive(XmlDeserializable, Debug, PartialEq)]
    struct TestSecurityResourceExhaustion {
        pub content: String,
    }

    let large_cdata = "X".repeat(50000); 
    let malicious_xml = format!(
        r#"<TestSecurityResourceExhaustion>
            <content><![CDATA[{}]]></content>
        </TestSecurityResourceExhaustion>"#,
        large_cdata
    );

    let start = Instant::now();
    let result = from_xml::<TestSecurityResourceExhaustion>(&malicious_xml);
    let duration = start.elapsed();
    
    assert!(duration < std::time::Duration::from_secs(3));
    assert!(result.is_ok());
    
    let parsed = result.unwrap();
    assert_eq!(parsed.content.len(), 50000);
    
    Ok(())
}
