use xavier::{ encode, cdata };


#[test]
fn serialize() {
    let should = r#"Some text &amp; others"#;
    assert_eq!(encode!("Some text & others"), should);

    let should = r#"<![CDATA[Some text &amp; others]]>"#;
    assert_eq!(cdata!("Some text & others"), should);
}
