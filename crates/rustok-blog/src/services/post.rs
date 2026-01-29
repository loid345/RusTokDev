use std::sync::Arc;

use rustok_content::{
    ContentError, ContentRepository, ContentService, ContentResult, CreateNodeInput, Node,
    NodeTranslation, NodeUpdate, TranslationInput,
};
use rustok_core::EventBus;
use serde_json::Value;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum PostServiceError {
    #[error("Content error: {0}")]
    Content(#[from] ContentError),
}

pub struct PostService<R: ContentRepository> {
    content: ContentService<R>,
}

impl<R: ContentRepository> PostService<R> {
    pub fn new(repository: Arc<R>, event_bus: EventBus) -> Self {
        Self {
            content: ContentService::new(repository, event_bus),
        }
    }

    pub async fn create_post(
        &self,
        tenant_id: Uuid,
        author_id: Option<Uuid>,
        metadata: Value,
        actor_id: Option<Uuid>,
    ) -> Result<Node, PostServiceError> {
        let input = CreateNodeInput {
            tenant_id,
            parent_id: None,
            author_id,
            kind: "post".to_string(),
            metadata,
        };

        Ok(self.content.create_node(input, actor_id).await?)
    }

    pub async fn update_post(
        &self,
        post_id: Uuid,
        update: NodeUpdate,
        actor_id: Option<Uuid>,
    ) -> Result<Node, PostServiceError> {
        Ok(self.content.update_node(post_id, update, actor_id).await?)
    }

    pub async fn upsert_translation(
        &self,
        post_id: Uuid,
        input: TranslationInput,
        actor_id: Option<Uuid>,
    ) -> Result<NodeTranslation, PostServiceError> {
        Ok(self
            .content
            .upsert_translation(post_id, input, actor_id)
            .await?)
    }

    pub async fn publish_post(
        &self,
        post_id: Uuid,
        actor_id: Option<Uuid>,
    ) -> Result<Node, PostServiceError> {
        Ok(self.content.publish_node(post_id, actor_id).await?)
    }

    pub async fn unpublish_post(
        &self,
        post_id: Uuid,
        actor_id: Option<Uuid>,
    ) -> Result<Node, PostServiceError> {
        Ok(self.content.unpublish_node(post_id, actor_id).await?)
    }

    pub async fn delete_post(
        &self,
        post_id: Uuid,
        actor_id: Option<Uuid>,
    ) -> Result<(), PostServiceError> {
        Ok(self.content.delete_node(post_id, actor_id).await?)
    }

    pub fn content(&self) -> &ContentService<R> {
        &self.content
    }
}

pub type PostResult<T> = ContentResult<T>;
