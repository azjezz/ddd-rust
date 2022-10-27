use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::HttpResponseBuilder;
use actix_web::ResponseError as ActixResponseError;
use cqrs_es::persist::PersistenceError;
use cqrs_es::AggregateError;
use serde_json::json;
use std::error::Error;
use std::fmt::Display;
use tera::Error as TeraError;

#[derive(Debug)]
pub enum ResponseError {
    Persistence(PersistenceError),
    Templating(TeraError),
    Aggregate(Box<dyn Error + Send + Sync + 'static>),
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseError::Persistence(error) => write!(f, "{}", error),
            ResponseError::Templating(error) => write!(f, "{}", error),
            ResponseError::Aggregate(error) => write!(f, "{}", error),
        }
    }
}

impl Error for ResponseError {}

impl From<PersistenceError> for ResponseError {
    fn from(e: PersistenceError) -> Self {
        ResponseError::Persistence(e)
    }
}

impl From<TeraError> for ResponseError {
    fn from(e: TeraError) -> Self {
        ResponseError::Templating(e)
    }
}

impl<T: Error + Send + Sync + 'static> From<AggregateError<T>> for ResponseError {
    fn from(e: AggregateError<T>) -> Self {
        ResponseError::Aggregate(Box::new(e))
    }
}

impl ActixResponseError for ResponseError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let json = match self {
            ResponseError::Persistence(error) => json!({
              "message": "internal server error",
              "error": {
                "type": "Persistence",
                "message": error.to_string(),
              }
            }),
            ResponseError::Templating(error) => json!({
              "message": "internal server error",
              "error": {
                "type": "Templating",
                "message": error.to_string(),
              }
            }),
            ResponseError::Aggregate(error) => json!({
              "message": "internal server error",
              "error": {
                "type": "Aggregate",
                "message": error.to_string(),
              }
            }),
        };

        HttpResponseBuilder::new(self.status_code()).json(json)
    }
}
