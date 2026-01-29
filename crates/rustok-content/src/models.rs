use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeStatus {
    Draft,
    Published,
    Archived,
}

impl NodeStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            NodeStatus::Draft => "draft",
            NodeStatus::Published => "published",
            NodeStatus::Archived => "archived",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub author_id: Option<Uuid>,
    pub kind: String,
    pub status: NodeStatus,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeTranslation {
    pub node_id: Uuid,
    pub locale: String,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub excerpt: Option<String>,
    pub body: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateNodeInput {
    pub tenant_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub author_id: Option<Uuid>,
    pub kind: String,
    pub metadata: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct NodeUpdate {
    pub parent_id: Option<Uuid>,
    pub author_id: Option<Uuid>,
    pub metadata: Option<Value>,
    pub status: Option<NodeStatus>,
    pub published_at: Option<Option<DateTime<Utc>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TranslationInput {
    pub locale: String,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub excerpt: Option<String>,
    pub body: Option<String>,
}
