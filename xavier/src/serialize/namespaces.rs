pub type Namespaces = String;

#[macro_export]
macro_rules! namespaces {
    ( $( $prefix:ident = $uri:expr ),* $(,)? ) => {
        {
            let mut namespaces = String::new();
            $(
                namespaces.push_str(&format!("xmlns:{}=\"{}\" ", stringify!($prefix), $uri));
            )*
            namespaces
        }
    };
}
