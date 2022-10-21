#![macro_use]

#[macro_export]
macro_rules! __render {
    ($tera:ident, $t:expr, $($json:tt)+) => {
        match $tera.0.render(&$t.to_string(), &::tera::Context::from_value(::serde_json::json!($($json)+)).unwrap()) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("error while rendering response template ( {} ): {e:?}", $t.to_string());

                ::std::process::exit(1);
            }
        }
    };
}

pub(crate) use __render as render;
