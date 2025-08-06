#[macro_export]
macro_rules! decode {
    ($expr:expr) => { xavier::deserialize::decode::decode($expr).to_string() };
}

pub fn decode(input: &str) -> String {
    input
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}
