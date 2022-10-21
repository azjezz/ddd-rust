pub mod web;

mod repository;

use crate::application::task::queries::TaskQuery;
use crate::application::task::services::TaskServices;
use crate::domain::task::aggregate::Task;
use crate::infrastructure::shared::state;
use crate::infrastructure::task::repository::PostgresTaskRepository;

use postgres_es::PostgresCqrs;
use sqlx::Pool;
use sqlx::Postgres;
use std::sync::Arc;

type CQRSFramework = Arc<PostgresCqrs<Task>>;

#[async_trait::async_trait]
impl state::CreatedFromState for PostgresTaskRepository {
    async fn create(state: &mut state::State) -> PostgresTaskRepository {
        let pool = state.get::<Pool<Postgres>>().await;

        PostgresTaskRepository::new("task_view", pool)
    }
}

#[async_trait::async_trait]
impl state::CreatedFromState for TaskServices {
    async fn create(_: &mut state::State) -> TaskServices {
        TaskServices::new()
    }
}

#[async_trait::async_trait]
impl state::CreatedFromState for TaskQuery<PostgresTaskRepository> {
    async fn create(state: &mut state::State) -> TaskQuery<PostgresTaskRepository> {
        // TODO(azjezz): add error handler.
        //
        // query.set_error_handler(Some(Box::new(|error| {
        //   // do something
        // })))

        TaskQuery::new(Arc::new(state.get::<PostgresTaskRepository>().await))
    }
}

#[async_trait::async_trait]
impl state::CreatedFromState for CQRSFramework {
    async fn create(state: &mut state::State) -> CQRSFramework {
        let pool = state.get::<Pool<Postgres>>().await;
        let query = state.get::<Box<TaskQuery<PostgresTaskRepository>>>().await;
        let services = state.get::<TaskServices>().await;

        Arc::new(postgres_es::postgres_aggregate_cqrs(
            pool,
            vec![query],
            services,
        ))
    }
}
