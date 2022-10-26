use crate::domain::shared::repository::Repository;
use crate::domain::task::commands::TaskCommand;
use crate::infrastructure::shared::macros;
use crate::infrastructure::shared::web::errors::ResponseError;
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
use actix_web::Result;
use tera::Tera;
use uuid::Uuid;

pub async fn index(
    tera: Service<Tera>,
    repository: Service<PostgresTaskRepository>,
) -> Result<impl Responder, ResponseError> {
    let tasks = repository.0.load_all().await?;
    let content = macros::render!(tera, "index.html", { "tasks": tasks });

    Ok(HttpResponseBuilder::new(StatusCode::OK)
        .content_type(ContentType::html())
        .body(content))
}

pub async fn create(
    cqrs: Service<CQRSFramework>,
    input: Form<CreateTask>,
    metadata: Metadata,
) -> Result<impl Responder, ResponseError> {
    cqrs.0
        .execute_with_metadata(
            &Uuid::new_v4().to_string(),
            TaskCommand::create(input.content.clone()),
            metadata.0,
        )
        .await?;

    Ok(HttpResponseBuilder::new(StatusCode::FOUND)
        .append_header(("Location", "/"))
        .finish())
}

pub async fn finish(
    cqrs: Service<CQRSFramework>,
    id: Path<String>,
    metadata: Metadata,
) -> Result<impl Responder, ResponseError> {
    cqrs.0
        .execute_with_metadata(&id.into_inner().clone(), TaskCommand::Finish, metadata.0)
        .await?;

    Ok(HttpResponseBuilder::new(StatusCode::FOUND)
        .append_header(("Location", "/"))
        .finish())
}

pub async fn delete(
    cqrs: Service<CQRSFramework>,
    id: Path<String>,
    metadata: Metadata,
) -> Result<impl Responder, ResponseError> {
    cqrs.0
        .execute_with_metadata(&id.into_inner().clone(), TaskCommand::Delete, metadata.0)
        .await?;

    Ok(HttpResponseBuilder::new(StatusCode::FOUND)
        .append_header(("Location", "/"))
        .finish())
}
