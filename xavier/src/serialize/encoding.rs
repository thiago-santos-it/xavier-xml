#[macro_export]
macro_rules! xcdata {
    ($expr:expr) => { XMLCData($expr.to_string()) };
}

#[macro_export]
macro_rules! xtext {
    ($expr:expr) => { $expr.to_string() };
}