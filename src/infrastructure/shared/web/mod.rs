pub mod metadata;
pub mod service;

mod handlers;

use actix_web::web::to;
use actix_web::web::ServiceConfig;

pub fn configure(config: &mut ServiceConfig) {
    config.default_service(to(handlers::default));
}
