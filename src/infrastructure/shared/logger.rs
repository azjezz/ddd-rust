use crate::infrastructure::shared::settings;

use actix_settings::Mode;

pub fn configure(settings: &settings::Settings) {
    if settings.actix.enable_log {
        match settings.actix.mode {
            Mode::Development => {
                std::env::set_var("RUST_BACKTRACE", "full");
                std::env::set_var("RUST_LOG", "actix_web=debug");
            }
            Mode::Production => {
                std::env::set_var("RUST_LOG", "actix_web=info");
            }
        }

        env_logger::init();
    }
}
