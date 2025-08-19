use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
struct TestStruct {
    pub content: String,
}

#[test]
fn security_dangerous_characters() {
    // Teste para caracteres especiais perigosos usando CDATA
    let malicious_xml = r#"
    <TestStruct>
        <content><![CDATA[<script>alert('XSS')</script>]]></content>
    </TestStruct>
    "#;
    
    let result = from_xml::<TestStruct>(&malicious_xml);
    // Deve ser processado corretamente com CDATA
    assert!(result.is_ok());
    
    let parsed = result.unwrap();
    // O conteúdo deve ser preservado literalmente dentro do CDATA
    assert!(parsed.content.contains("<script>"));
    assert!(parsed.content.contains("alert('XSS')"));
    assert!(parsed.content.contains("</script>"));
}