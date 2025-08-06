
#[macro_export]
macro_rules! cdata {
    ($expr:expr) => { format!("<![CDATA[{}]]>", $expr).to_string() };
}

#[macro_export]
macro_rules! encode {
    ($expr:expr) => { xavier::serialize::encode::escape_xml($expr).to_string() };
}

#[macro_export]
macro_rules! comment {
    ($expr:expr) => { format!("<!--{}-->", xavier::serialize::escaping::escape_xml($expr)).to_string() };
}

pub fn escape_xml(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}
