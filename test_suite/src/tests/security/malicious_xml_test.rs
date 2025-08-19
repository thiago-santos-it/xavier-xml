use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
struct TestStruct {
    pub content: String,
}

#[test]
fn security_large_entities() {
    // Teste para entidades XML muito grandes
    let malicious_xml = format!(
        r#"<TestStruct><content>&{};</content></TestStruct>"#,
        "a".repeat(1000000) // Entidade muito grande
    );
    
    let result = from_xml::<TestStruct>(&malicious_xml);
    // Deve falhar ou ser rejeitado
    assert!(result.is_err());
}

#[test]
fn security_infinite_loop_references() {
    // Teste para loops infinitos em referências
    let malicious_xml = r#"
    <!DOCTYPE test [
        <!ENTITY a "&b;">
        <!ENTITY b "&a;">
    ]>
    <TestStruct><content>&a;</content></TestStruct>
    "#;
    
    let result = from_xml::<TestStruct>(&malicious_xml);
    // Deve falhar ou ser rejeitado
    assert!(result.is_err());
}

#[test]
fn security_dangerous_characters() {
    // Teste para caracteres especiais perigosos
    let malicious_xml = r#"
    <TestStruct>
        <content><script>alert('XSS')</script></content>
    </TestStruct>
    "#;
    
    let result = from_xml::<TestStruct>(&malicious_xml);
    // Deve ser processado corretamente (não deve executar o script)
    assert!(result.is_ok());
    
    let parsed = result.unwrap();
    assert!(parsed.content.contains("<script>"));
}

#[test]
fn security_malicious_encoding() {
    // Teste para XML com encoding malicioso
    let malicious_xml = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <TestStruct>
        <content>&#x00;&#x01;&#x02;</content>
    </TestStruct>
    "#;
    
    let result = from_xml::<TestStruct>(&malicious_xml);
    // Deve falhar ou ser rejeitado
    assert!(result.is_err());
}

#[test]
fn security_oversized_xml() {
    // Teste para XML muito grande
    let oversized_content = "A".repeat(10_000_000); // 10MB de conteúdo
    let malicious_xml = format!(
        r#"<TestStruct><content>{}</content></TestStruct>"#,
        oversized_content
    );
    
    let result = from_xml::<TestStruct>(&malicious_xml);
    // Deve falhar ou ser rejeitado
    assert!(result.is_err());
}

#[test]
fn security_nested_entities() {
    // Teste para entidades aninhadas
    let malicious_xml = r#"
    <!DOCTYPE test [
        <!ENTITY a "&b;&c;">
        <!ENTITY b "&c;&d;">
        <!ENTITY c "&d;&e;">
        <!ENTITY d "&e;&f;">
        <!ENTITY e "&f;&g;">
        <!ENTITY f "&g;&h;">
        <!ENTITY g "&h;&i;">
        <!ENTITY h "&i;&j;">
        <!ENTITY i "&j;&k;">
        <!ENTITY j "&k;&l;">
        <!ENTITY k "&l;&m;">
        <!ENTITY l "&m;&n;">
        <!ENTITY m "&n;&o;">
        <!ENTITY n "&o;&p;">
        <!ENTITY o "&p;&q;">
        <!ENTITY p "&q;&r;">
        <!ENTITY q "&r;&s;">
        <!ENTITY r "&s;&t;">
        <!ENTITY s "&t;&u;">
        <!ENTITY t "&u;&v;">
        <!ENTITY u "&v;&w;">
        <!ENTITY v "&w;&x;">
        <!ENTITY w "&x;&y;">
        <!ENTITY x "&y;&z;">
        <!ENTITY y "&z;&a;">
        <!ENTITY z "test">
    ]>
    <TestStruct><content>&a;</content></TestStruct>
    "#;
    
    let result = from_xml::<TestStruct>(&malicious_xml);
    // Deve falhar ou ser rejeitado
    assert!(result.is_err());
} 