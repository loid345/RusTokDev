mod engine;
mod listener;
mod models;
mod pg_engine;

use async_trait::async_trait;
use rustok_core::{EventListener, MigrationSource, RusToKModule};
use sea_orm_migration::MigrationTrait;

pub use engine::{SearchEngine, SearchQuery, SearchResult};
pub use listener::IndexListener;
pub use models::{DocumentType, IndexDocument};
pub use pg_engine::PgSearchEngine;

pub struct IndexModule;

#[async_trait]
impl RusToKModule for IndexModule {
    fn slug(&self) -> &'static str {
        "index"
    }

    fn name(&self) -> &'static str {
        "Index"
    }

    fn description(&self) -> &'static str {
        "CQRS read models and search indexing"
    }

    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn event_listeners(&self) -> Vec<Box<dyn EventListener>> {
        Vec::new()
    }
}

impl MigrationSource for IndexModule {
    fn migrations(&self) -> Vec<Box<dyn MigrationTrait>> {
        Vec::new()
    }
}
