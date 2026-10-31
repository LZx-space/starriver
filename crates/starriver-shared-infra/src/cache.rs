use std::{hash::Hash, marker::PhantomData, time::Duration};

use moka::Equivalent;
use starriver_shared_base::cache::{Cache, Invalidator};

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
}

impl<K, V> Invalidator<K> for DefaultCache<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    async fn invalidate<Q>(&self, key: &Q)
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.inner.invalidate(key).await;
    }

    fn invalidate_all(&self) {
        self.inner.invalidate_all()
    }
}

impl<K, V> Clone for DefaultCache<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        DefaultCache {
            inner: self.inner.clone(),
            _k: PhantomData,
            _v: PhantomData,
        }
    }
}
