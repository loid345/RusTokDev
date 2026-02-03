use std::sync::Arc;

use async_trait::async_trait;

use crate::events::EventTransport;

#[async_trait]
pub trait CacheBackend: Send + Sync {
    async fn health(&self) -> crate::Result<()>;
}

#[async_trait]
pub trait SearchBackend: Send + Sync {
    async fn health(&self) -> crate::Result<()>;
}

pub struct AppContext {
    pub events: Arc<dyn EventTransport>,
    pub cache: Arc<dyn CacheBackend>,
    pub search: Arc<dyn SearchBackend>,
}
