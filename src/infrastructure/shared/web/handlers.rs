use crate::infrastructure::shared::macros;
use crate::infrastructure::shared::web::service::Service;

use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::HttpResponseBuilder;
use actix_web::Responder;
use tera::Tera;

pub async fn default(tera: Service<Tera>) -> impl Responder {
    let content = macros::render!(tera, "errors/404.html", {});

    HttpResponseBuilder::new(StatusCode::NOT_FOUND)
        .content_type(ContentType::html())
        .body(content)
}
