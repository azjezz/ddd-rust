#![macro_use]

#[macro_export]
macro_rules! __render {
    ($tera:ident, $t:expr, $($json:tt)+) => {
        $tera.0.render(&$t.to_string(), &::tera::Context::from_value(::serde_json::json!($($json)+)).unwrap())?
    };
}

pub(crate) use __render as render;
