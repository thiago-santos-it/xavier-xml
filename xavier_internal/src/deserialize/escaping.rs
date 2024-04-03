#[macro_export]
macro_rules! decode {
    ($expr:expr) => { xavier::deserialize::escaping::decode($expr).to_string() };
}

pub fn decode(input: &str) -> String {
    html_escape::decode_html_entities(input).to_string()
}
