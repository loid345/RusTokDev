use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use rustok_core::events::{EventEnvelope, EventTransport, ReliabilityLevel};
use rustok_core::Result;

use crate::entity;
use crate::entity::SysEventStatus;

#[derive(Clone, Debug)]
pub struct OutboxTransport {
    db: DatabaseConnection,
}

impl OutboxTransport {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl EventTransport for OutboxTransport {
    async fn publish(&self, envelope: EventEnvelope) -> Result<()> {
        let payload = serde_json::to_value(&envelope)?;
        let model = entity::ActiveModel {
            id: Set(envelope.id),
            payload: Set(sea_orm::Json(payload)),
            status: Set(SysEventStatus::Pending),
            created_at: Set(Utc::now()),
            dispatched_at: Set(None),
        };
        model.insert(&self.db).await?;
        Ok(())
    }

    async fn acknowledge(&self, event_id: uuid::Uuid) -> Result<()> {
        let mut model: entity::ActiveModel = entity::Entity::find_by_id(event_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| rustok_core::Error::NotFound(format!("sys_event {event_id}")))?
            .into();
        model.status = Set(SysEventStatus::Dispatched);
        model.dispatched_at = Set(Some(Utc::now()));
        model.update(&self.db).await?;
        Ok(())
    }

    fn reliability_level(&self) -> ReliabilityLevel {
        ReliabilityLevel::Outbox
    }
}
