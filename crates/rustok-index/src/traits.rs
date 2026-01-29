use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::error::IndexResult;

/// Context for indexing operations
#[derive(Clone)]
pub struct IndexerContext {
    pub db: DatabaseConnection,
    pub tenant_id: Uuid,
}

impl IndexerContext {
    pub fn new(db: DatabaseConnection, tenant_id: Uuid) -> Self {
        Self { db, tenant_id }
    }
}

/// Trait for indexers that update denormalized tables
#[async_trait]
pub trait Indexer: Send + Sync {
    /// Indexer name for logging
    fn name(&self) -> &'static str;

    /// Index a single entity by ID (all locales)
    async fn index_one(&self, ctx: &IndexerContext, entity_id: Uuid) -> IndexResult<()>;

    /// Remove entity from index
    async fn remove_one(&self, ctx: &IndexerContext, entity_id: Uuid) -> IndexResult<()>;

    /// Reindex all entities for a tenant
    async fn reindex_all(&self, ctx: &IndexerContext) -> IndexResult<u64>;
}

/// Trait for locale-aware indexers
#[async_trait]
pub trait LocaleIndexer: Indexer {
    /// Index entity for specific locale
    async fn index_locale(
        &self,
        ctx: &IndexerContext,
        entity_id: Uuid,
        locale: &str,
    ) -> IndexResult<()>;

    /// Remove entity from index for specific locale
    async fn remove_locale(
        &self,
        ctx: &IndexerContext,
        entity_id: Uuid,
        locale: &str,
    ) -> IndexResult<()>;
}
