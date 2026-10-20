use std::future::Future;
use std::hash::Hash;
use std::sync::Arc;

pub trait Cache<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn try_get_with<F, E>(&self, key: K, init: F) -> impl Future<Output = Result<V, Arc<E>>>
    where
        F: Future<Output = Result<V, E>>,
        E: Send + Sync + 'static;

    fn invalidate_all(&self);
}
