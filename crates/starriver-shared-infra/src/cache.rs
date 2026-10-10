use std::{hash::Hash, marker::PhantomData, time::Duration};

use starriver_shared_base::cache::Cache;

pub struct DefaultCache<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    inner: moka::future::Cache<K, V>,
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}

impl<K, V> DefaultCache<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    pub fn new(capacity: u64, ttl: Duration) -> Self {
        Self {
            inner: moka::future::Cache::builder()
                .max_capacity(capacity)
                .time_to_live(ttl)
                .build(),
            _k: PhantomData,
            _v: PhantomData,
        }
    }
}

impl<K, V> Cache<K, V> for DefaultCache<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn try_get_with<F, E>(
        &self,
        key: K,
        init: F,
    ) -> impl Future<Output = Result<V, std::sync::Arc<E>>>
    where
        F: Future<Output = Result<V, E>>,
        E: Send + Sync + 'static,
        Self: Sized,
    {
        self.inner.try_get_with(key, init)
    }

    fn invalidate_all(&self) {
        self.inner.invalidate_all()
    }
}
