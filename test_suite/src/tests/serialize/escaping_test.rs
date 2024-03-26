use xavier::{ xtext, xcdata };


#[test]
fn serialize() {
    let should = r#"Some text &amp; others"#;
    assert_eq!(xtext!("Some text & others"), should);

    let should = r#"<![CDATA[Some text &amp; others]]>"#;
    assert_eq!(xcdata!("Some text & others"), should);

}
