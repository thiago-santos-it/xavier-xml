
#[macro_export]
macro_rules! xcdata {
    ($expr:expr) => { format!("<![CDATA[{}]]>", xavier::serialize::escaping::escape_xml($expr)).to_string() };
}

#[macro_export]
macro_rules! xtext {
    ($expr:expr) => { xavier::serialize::escaping::escape_xml($expr).to_string() };
}

#[macro_export]
macro_rules! comment {
    ($expr:expr) => { format!("<!--{}-->", xavier::serialize::escaping::escape_xml($expr)).to_string() };
}

pub fn escape_xml(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        match c {
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '&' => result.push_str("&amp;"),
            '"' => result.push_str("&quot;"),
            '\'' => result.push_str("&apos;"),
            _ => result.push(c),
        }
    }
    result
}
