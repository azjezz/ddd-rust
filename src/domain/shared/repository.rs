use cqrs_es::persist::PersistenceError;
use cqrs_es::persist::ViewRepository;
use cqrs_es::Aggregate;
use cqrs_es::View;

#[async_trait::async_trait]
pub trait Repository<V, A>: ViewRepository<V, A>
where
    V: View<A>,
    A: Aggregate,
{
    /// Returns all view instances.
    async fn load_all(&self) -> Result<Vec<V>, PersistenceError>;
}
