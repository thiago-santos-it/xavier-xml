use xavier::decode;

#[test]
fn decode() {
    let encoded = "Test &amp; &gt; &lt;";
    let decoded = "Test & > <";
    assert_eq!(decoded, decode!(encoded));
}