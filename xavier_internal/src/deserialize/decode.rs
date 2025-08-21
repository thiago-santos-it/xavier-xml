#[macro_export]
macro_rules! decode {
    ($expr:expr) => { xavier::deserialize::decode::decode_xml($expr).to_string() };
}

pub fn decode_xml(input: &str) -> String {
    input
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

pub fn strip_cdata(s: &str) -> &str {
    let cdata_start = "<![CDATA[";
    let cdata_end = "]]>";

    if s.starts_with(cdata_start) && s.ends_with(cdata_end) {
        &s[cdata_start.len()..s.len() - cdata_end.len()]
    } else {
        s
    }
}