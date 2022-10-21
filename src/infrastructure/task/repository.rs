use crate::domain::shared::repository::Repository;
use crate::domain::task::aggregate::Task;
use crate::domain::task::repository::TaskRepository;
use crate::domain::task::view::TaskView;

use cqrs_es::persist::{PersistenceError, ViewContext, ViewRepository};
use postgres_es::PostgresViewRepository;
use serde_json;
use sqlx::postgres::PgRow;
use sqlx::Error;
use sqlx::Row;
use sqlx::{Pool, Postgres};

pub struct PostgresTaskRepository {
    view_name: String,
    pool: Pool<Postgres>,
    inner: PostgresViewRepository<TaskView, Task>,
}

impl PostgresTaskRepository {
    pub fn new(view_name: &str, pool: Pool<Postgres>) -> Self {
        let inner_pool = pool.clone();

        PostgresTaskRepository {
            view_name: view_name.to_string(),
            pool,
            inner: PostgresViewRepository::new(view_name, inner_pool),
        }
    }
}

impl Clone for PostgresTaskRepository {
    fn clone(&self) -> Self {
        let view_name = self.view_name.clone();
        let pool = self.pool.clone();

        PostgresTaskRepository::new(&view_name, pool)
    }
}

#[async_trait::async_trait]
impl ViewRepository<TaskView, Task> for PostgresTaskRepository {
    /// Returns the current view instance.
    async fn load(&self, view_id: &str) -> Result<Option<TaskView>, PersistenceError> {
        self.inner.load(view_id).await
    }

    /// Returns the current view instance and context, used by the `GenericQuery` to update
    /// views with committed events.
    async fn load_with_context(
        &self,
        view_id: &str,
    ) -> Result<Option<(TaskView, ViewContext)>, PersistenceError> {
        self.inner.load_with_context(view_id).await
    }

    /// Updates the view instance and context, used by the `GenericQuery` to update
    /// views with committed events.
    async fn update_view(
        &self,
        view: TaskView,
        context: ViewContext,
    ) -> Result<(), PersistenceError> {
        self.inner.update_view(view, context).await
    }
}

#[async_trait::async_trait]
impl Repository<TaskView, Task> for PostgresTaskRepository {
    /// Returns all view instances.
    async fn load_all(&self) -> Result<Vec<TaskView>, PersistenceError> {
        let rows: Vec<PgRow> = sqlx::query(&format!(
            "
                SELECT version, payload
                FROM {}
                WHERE (payload->>'is_deleted')::boolean IS FALSE
                ORDER BY (payload->>'created_at') DESC
            ",
            &self.view_name
        ))
        .fetch_all(&self.pool)
        .await
        .map_err(|err: Error| match &err {
            Error::Database(database_error) => {
                if let Some(code) = database_error.code() {
                    if code.as_ref() == "23505" {
                        return PersistenceError::OptimisticLockError;
                    }
                }
                PersistenceError::UnknownError(Box::new(err))
            }
            Error::Io(_) | Error::Tls(_) => PersistenceError::ConnectionError(Box::new(err)),
            _ => PersistenceError::UnknownError(Box::new(err)),
        })?;

        let mut result: Vec<TaskView> = Vec::new();
        for row in rows {
            let view: TaskView = serde_json::from_value(row.get("payload"))?;
            result.push(view);
        }

        Ok(result)
    }
}

impl TaskRepository for PostgresTaskRepository {}
