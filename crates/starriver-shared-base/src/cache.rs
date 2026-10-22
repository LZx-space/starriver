use std::future::Future;
use std::hash::Hash;
use std::sync::Arc;

use equivalent::Equivalent;

pub trait Cache<K, V>: Invalidator<K>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn try_get_with<F, E>(&self, key: K, init: F) -> impl Future<Output = Result<V, Arc<E>>>
    where
        F: Future<Output = Result<V, E>>,
        E: Send + Sync + 'static;
}

/// [`Cache`] 失效方法的抽象
pub trait Invalidator<K>
where
    K: Hash + Eq + Send + Sync + 'static,
{
    fn invalidate<Q>(&self, key: &Q) -> impl Future<Output = ()>
    where
        Q: Equivalent<K> + Hash + ?Sized;

    fn invalidate_all(&self);
}
