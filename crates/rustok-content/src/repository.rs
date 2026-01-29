use async_trait::async_trait;
use uuid::Uuid;

use crate::error::ContentResult;
use crate::models::{Node, NodeTranslation, NodeUpdate};

#[async_trait]
pub trait ContentRepository: Send + Sync {
    async fn insert_node(&self, node: Node) -> ContentResult<Node>;
    async fn update_node(&self, node_id: Uuid, update: NodeUpdate) -> ContentResult<Node>;
    async fn delete_node(&self, node_id: Uuid) -> ContentResult<()>;
    async fn find_node(&self, node_id: Uuid) -> ContentResult<Option<Node>>;

    async fn upsert_translation(&self, translation: NodeTranslation)
        -> ContentResult<NodeTranslation>;
    async fn find_translation(
        &self,
        node_id: Uuid,
        locale: &str,
    ) -> ContentResult<Option<NodeTranslation>>;
}
