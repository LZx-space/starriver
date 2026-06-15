use std::future::Future;
use std::hash::RandomState;
use std::sync::Arc;

pub trait Cache<K, V, S = RandomState> {
    fn try_get_with<F, E>(&self, key: K, init: F) -> impl Future<Output = Result<V, Arc<E>>>
    where
        F: Future<Output = Result<V, E>>,
        E: Send + Sync + 'static,
        Self: Sized;

    fn invalidate_all(&self);
}
