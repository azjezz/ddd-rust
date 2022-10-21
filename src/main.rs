mod application;
mod domain;
mod infrastructure;

use crate::infrastructure::shared::logger;
use crate::infrastructure::shared::settings;
use crate::infrastructure::shared::state;

use actix_settings::ApplySettings;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use tokio::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = settings::load("actix.toml");

    logger::configure(&settings);

    HttpServer::new({
        let settings = settings.clone();

        move || {
            let state = state::State::new(settings.clone());

            App::new()
                .app_data(Data::new(Mutex::new(state)))
                .configure(infrastructure::shared::web::configure)
                .configure(infrastructure::task::web::configure)
        }
    })
    .apply_settings(&settings)
    .run()
    .await
}
