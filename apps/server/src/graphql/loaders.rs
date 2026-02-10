use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::models::_entities::tenants;

#[derive(Clone)]
pub struct TenantNameLoader {
    db: DatabaseConnection,
}

impl TenantNameLoader {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TenantNameLoader {
    type Value = String;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let tenants = tenants::Entity::find()
            .filter(tenants::Column::Id.is_in(keys.iter().copied()))
            .all(&self.db)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(tenants
            .into_iter()
            .map(|tenant| (tenant.id, tenant.name))
            .collect())
    }
}
