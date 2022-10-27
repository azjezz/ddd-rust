pub(in crate::infrastructure) mod macros;

pub mod logger;
pub mod settings;
pub mod state;
pub mod web;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tera::Tera;

#[async_trait::async_trait]
impl<T: state::CreatedFromState + Clone + Sync + 'static> state::CreatedFromState for Box<T> {
    async fn create(state: &mut state::State) -> Box<T> {
        let inner = state.get::<T>().await;

        Box::new(inner)
    }
}

#[async_trait::async_trait]
impl state::CreatedFromState for Tera {
    async fn create(state: &mut state::State) -> Tera {
        Tera::new(&state.settings.application.templates)
            .map_err(|e| {
                println!("Error(s) parsing tera templates: {}", e);

                ::std::process::exit(1);
            })
            .unwrap()
    }
}

#[async_trait::async_trait]
impl state::CreatedFromState for Pool<Postgres> {
    async fn create(state: &mut state::State) -> Pool<Postgres> {
        PgPoolOptions::new()
            .max_connections(10)
            .connect(&state.settings.application.database_url)
            .await
            .map_err(|e| {
                println!(
                    "Failed to create postgreSQL pool ( {} ): {}",
                    state.settings.application.database_url, e
                );

                ::std::process::exit(1);
            })
            .unwrap()
    }
}
