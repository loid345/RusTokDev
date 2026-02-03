pub mod dto;
pub mod entities;
pub mod error;
pub mod services;

pub use dto::{CreatePageInput, PageResponse};
pub use services::PageService;

use async_trait::async_trait;
use rustok_core::module::{HealthStatus, MigrationSource, RusToKModule};
use sea_orm_migration::MigrationTrait;

pub struct PagesModule;

impl MigrationSource for PagesModule {
    fn migrations(&self) -> Vec<Box<dyn MigrationTrait>> {
        Vec::new()
    }
}

#[async_trait]
impl RusToKModule for PagesModule {
    fn slug(&self) -> &'static str {
        "pages"
    }

    fn name(&self) -> &'static str {
        "Pages"
    }

    fn description(&self) -> &'static str {
        "Pages and menus domain logic."
    }

    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    async fn health(&self) -> HealthStatus {
        HealthStatus::Healthy
    }
}
