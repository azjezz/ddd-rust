mod handlers;
mod input;

use actix_web::web::get;
use actix_web::web::post;
use actix_web::web::ServiceConfig;

pub fn configure(config: &mut ServiceConfig) {
    config
        .route("/", get().to(handlers::index))
        .route("/task", post().to(handlers::create))
        .route("/task/finish/{id}", post().to(handlers::finish))
        .route("/task/delete/{id}", post().to(handlers::delete));
}
