use crate::domain::shared::repository::Repository;

use async_trait::async_trait;
use cqrs_es::persist::{PersistenceError, ViewContext};
use cqrs_es::{Aggregate, EventEnvelope, Query, View};
use std::marker::PhantomData;
use std::sync::Arc;

/// A convenience type for query error handlers.
///
/// In a CQRS system queries are downstream services and can not return errors in a problem is encountered.
/// This convenience type allows the user to define a function for handling persistence errors.
///
/// An error handler should be a method that takes a single `PersistenceError` parameter and has no
/// result.
pub type QueryErrorHandler = dyn Fn(PersistenceError) + Send + Sync + 'static;

/// A standard query and repository.
///
/// This is used both to act as a `Query` for processing events and to return materialized views.
pub struct StandardQuery<R, V, A>
where
    R: Repository<V, A>,
    V: View<A>,
    A: Aggregate,
{
    repository: Arc<R>,
    error_handler: Option<Box<QueryErrorHandler>>,
    phantom: PhantomData<(V, A)>,
}

impl<R, V, A> StandardQuery<R, V, A>
where
    R: Repository<V, A>,
    V: View<A>,
    A: Aggregate,
{
    /// Creates a new `StandardQuery` using the provided `Repository`.
    pub fn new(repository: Arc<R>) -> Self {
        StandardQuery {
            repository,
            error_handler: None,
            phantom: Default::default(),
        }
    }

    #[allow(dead_code)]
    pub fn set_error_handler(&mut self, error_handler: Option<Box<QueryErrorHandler>>) {
        self.error_handler = error_handler;
    }
}

#[async_trait]
impl<R, V, A> Query<A> for StandardQuery<R, V, A>
where
    R: Repository<V, A>,
    V: View<A>,
    A: Aggregate,
{
    async fn dispatch(&self, view_id: &str, events: &[EventEnvelope<A>]) {
        let result = self.repository.load_with_context(view_id).await;
        if let Err(e) = result {
            if let Some(handler) = &self.error_handler {
                (handler)(e)
            }

            return;
        }

        let (mut view, context) = match result.unwrap() {
            None => {
                let view_context = ViewContext::new(view_id.to_string(), 0);
                (Default::default(), view_context)
            }
            Some((view, context)) => (view, context),
        };

        for event in events {
            view.update(event);
        }

        if let Err(e) = self.repository.update_view(view, context).await {
            if let Some(handler) = &self.error_handler {
                (handler)(e)
            }
        }
    }
}

impl<R, V, A> Clone for StandardQuery<R, V, A>
where
    R: Repository<V, A>,
    V: View<A>,
    A: Aggregate,
{
    fn clone(&self) -> Self {
        Self {
            repository: self.repository.clone(),
            error_handler: None,
            phantom: self.phantom,
        }
    }
}
