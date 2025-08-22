use xavier::{cdata, decode, encode};

#[test]
fn encode_test() {
    let should = r#"Some text &amp; others"#;
    assert_eq!(encode!("Some text & others"), should);

    let should = r#"<![CDATA[Some text &amp; others]]>"#;
    assert_eq!(cdata!("Some text &amp; others"), should);
}

#[test]
fn decode_test() {
    let encoded = "Test &amp; &gt; &lt;";
    let decoded = "Test & > <";
    assert_eq!(decoded, decode!(encoded));
}