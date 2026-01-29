use std::sync::Arc;

use chrono::Utc;
use rustok_core::{DomainEvent, EventBus};
use uuid::Uuid;

use crate::error::{ContentError, ContentResult};
use crate::models::{
    CreateNodeInput, Node, NodeStatus, NodeTranslation, NodeUpdate, TranslationInput,
};
use crate::repository::ContentRepository;

pub struct ContentService<R: ContentRepository> {
    repository: Arc<R>,
    event_bus: EventBus,
}

impl<R: ContentRepository> ContentService<R> {
    pub fn new(repository: Arc<R>, event_bus: EventBus) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn create_node(
        &self,
        input: CreateNodeInput,
        actor_id: Option<Uuid>,
    ) -> ContentResult<Node> {
        let now = Utc::now();
        let node = Node {
            id: rustok_core::generate_id(),
            tenant_id: input.tenant_id,
            parent_id: input.parent_id,
            author_id: input.author_id,
            kind: input.kind,
            status: NodeStatus::Draft,
            metadata: input.metadata,
            created_at: now,
            updated_at: now,
            published_at: None,
        };

        let node = self.repository.insert_node(node).await?;
        self.event_bus.publish(
            node.tenant_id,
            actor_id,
            DomainEvent::NodeCreated {
                node_id: node.id,
                kind: node.kind.clone(),
                author_id: node.author_id,
            },
        )?;

        Ok(node)
    }

    pub async fn update_node(
        &self,
        node_id: Uuid,
        update: NodeUpdate,
        actor_id: Option<Uuid>,
    ) -> ContentResult<Node> {
        let node = self.repository.update_node(node_id, update).await?;
        self.event_bus.publish(
            node.tenant_id,
            actor_id,
            DomainEvent::NodeUpdated {
                node_id: node.id,
                kind: node.kind.clone(),
            },
        )?;

        Ok(node)
    }

    pub async fn upsert_translation(
        &self,
        node_id: Uuid,
        input: TranslationInput,
        actor_id: Option<Uuid>,
    ) -> ContentResult<NodeTranslation> {
        let translation = NodeTranslation {
            node_id,
            locale: input.locale,
            title: input.title,
            slug: input.slug,
            excerpt: input.excerpt,
            body: input.body,
            updated_at: Utc::now(),
        };

        let translation = self.repository.upsert_translation(translation).await?;
        let node = self
            .repository
            .find_node(node_id)
            .await?
            .ok_or(ContentError::NodeNotFound(node_id))?;

        self.event_bus.publish(
            node.tenant_id,
            actor_id,
            DomainEvent::NodeTranslationUpdated {
                node_id: node.id,
                locale: translation.locale.clone(),
            },
        )?;

        Ok(translation)
    }

    pub async fn publish_node(
        &self,
        node_id: Uuid,
        actor_id: Option<Uuid>,
    ) -> ContentResult<Node> {
        let node = self
            .repository
            .find_node(node_id)
            .await?
            .ok_or(ContentError::NodeNotFound(node_id))?;

        let update = NodeUpdate {
            parent_id: None,
            author_id: None,
            metadata: None,
            status: Some(NodeStatus::Published),
            published_at: Some(Some(Utc::now())),
        };
        let updated = self.repository.update_node(node_id, update).await?;

        self.event_bus.publish(
            node.tenant_id,
            actor_id,
            DomainEvent::NodePublished {
                node_id: node.id,
                kind: node.kind.clone(),
            },
        )?;

        Ok(updated)
    }

    pub async fn unpublish_node(
        &self,
        node_id: Uuid,
        actor_id: Option<Uuid>,
    ) -> ContentResult<Node> {
        let node = self
            .repository
            .find_node(node_id)
            .await?
            .ok_or(ContentError::NodeNotFound(node_id))?;

        let update = NodeUpdate {
            parent_id: None,
            author_id: None,
            metadata: None,
            status: Some(NodeStatus::Draft),
            published_at: Some(None),
        };
        let updated = self.repository.update_node(node_id, update).await?;

        self.event_bus.publish(
            node.tenant_id,
            actor_id,
            DomainEvent::NodeUnpublished {
                node_id: node.id,
                kind: node.kind.clone(),
            },
        )?;

        Ok(updated)
    }

    pub async fn delete_node(
        &self,
        node_id: Uuid,
        actor_id: Option<Uuid>,
    ) -> ContentResult<()> {
        let node = self
            .repository
            .find_node(node_id)
            .await?
            .ok_or(ContentError::NodeNotFound(node_id))?;

        self.repository.delete_node(node_id).await?;
        self.event_bus.publish(
            node.tenant_id,
            actor_id,
            DomainEvent::NodeDeleted {
                node_id: node.id,
                kind: node.kind,
            },
        )?;

        Ok(())
    }
}
