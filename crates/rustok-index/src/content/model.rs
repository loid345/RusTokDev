use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Tag representation in index
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexTag {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}

/// Denormalized content index model
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexContentModel {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub node_id: Uuid,
    pub locale: String,

    pub kind: String,
    pub status: String,

    pub title: Option<String>,
    pub slug: Option<String>,
    pub excerpt: Option<String>,
    pub body: Option<String>,
    pub body_format: Option<String>,

    pub author_id: Option<Uuid>,
    pub author_name: Option<String>,
    pub author_avatar: Option<String>,

    pub category_id: Option<Uuid>,
    pub category_name: Option<String>,
    pub category_slug: Option<String>,

    pub tags: Vec<IndexTag>,

    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub og_image: Option<String>,

    pub featured_image_url: Option<String>,
    pub featured_image_alt: Option<String>,

    pub parent_id: Option<Uuid>,
    pub depth: i32,
    pub position: i32,

    pub reply_count: i32,
    pub view_count: i32,

    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
