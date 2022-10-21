use crate::domain::shared::repository::Repository;
use crate::domain::task::commands::TaskCommand;
use crate::infrastructure::shared::macros;
use crate::infrastructure::shared::web::metadata::Metadata;
use crate::infrastructure::shared::web::service::Service;
use crate::infrastructure::task::repository::PostgresTaskRepository;
use crate::infrastructure::task::web::input::CreateTask;
use crate::infrastructure::task::CQRSFramework;

use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::web::Form;
use actix_web::web::Path;
use actix_web::HttpResponseBuilder;
use actix_web::Responder;
use tera::Tera;
use uuid::Uuid;

pub async fn index(
    tera: Service<Tera>,
    repository: Service<PostgresTaskRepository>,
) -> impl Responder {
    match repository.0.load_all().await {
        Ok(tasks) => {
            let content = macros::render!(tera, "index.html", { "tasks": tasks });

            HttpResponseBuilder::new(StatusCode::OK)
                .content_type(ContentType::html())
                .body(content)
        }
        Err(err) => {
            let content =
                macros::render!(tera, "errors/500.html", { "error": format!("{:?}", err) });

            HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
                .content_type(ContentType::html())
                .body(content)
        }
    }
}

pub async fn create(
    tera: Service<Tera>,
    cqrs: Service<CQRSFramework>,
    input: Form<CreateTask>,
    metadata: Metadata,
) -> impl Responder {
    let result = cqrs
        .0
        .execute_with_metadata(
            &Uuid::new_v4().to_string(),
            TaskCommand::create(input.content.clone()),
            metadata.0,
        )
        .await;

    if let Err(err) = result {
        let content = macros::render!(tera, "errors/500.html", { "error": format!("{:?}", err) });

        HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type(ContentType::html())
            .body(content)
    } else {
        HttpResponseBuilder::new(StatusCode::FOUND)
            .append_header(("Location", "/"))
            .finish()
    }
}

pub async fn finish(
    tera: Service<Tera>,
    cqrs: Service<CQRSFramework>,
    id: Path<String>,
    metadata: Metadata,
) -> impl Responder {
    let result = cqrs
        .0
        .execute_with_metadata(&id.into_inner().clone(), TaskCommand::Finish, metadata.0)
        .await;

    if let Err(err) = result {
        let content = macros::render!(tera, "errors/500.html", { "error": format!("{:?}", err) });

        HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type(ContentType::html())
            .body(content)
    } else {
        HttpResponseBuilder::new(StatusCode::FOUND)
            .append_header(("Location", "/"))
            .finish()
    }
}

pub async fn delete(
    tera: Service<Tera>,
    cqrs: Service<CQRSFramework>,
    id: Path<String>,
    metadata: Metadata,
) -> impl Responder {
    let result = cqrs
        .0
        .execute_with_metadata(&id.into_inner().clone(), TaskCommand::Delete, metadata.0)
        .await;

    if let Err(err) = result {
        let content = macros::render!(tera, "errors/500.html", { "error": format!("{:?}", err) });

        HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type(ContentType::html())
            .body(content)
    } else {
        HttpResponseBuilder::new(StatusCode::FOUND)
            .append_header(("Location", "/"))
            .finish()
    }
}
