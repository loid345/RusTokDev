use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexProductImage {
    pub url: String,
    pub alt: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexProductOption {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexProductModel {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub product_id: Uuid,
    pub locale: String,

    pub status: String,
    pub is_published: bool,

    pub title: String,
    pub subtitle: Option<String>,
    pub handle: String,
    pub description: Option<String>,

    pub category_id: Option<Uuid>,
    pub category_name: Option<String>,
    pub category_path: Option<String>,

    pub tags: Vec<String>,
    pub brand: Option<String>,

    pub currency: Option<String>,
    pub price_min: Option<i64>,
    pub price_max: Option<i64>,
    pub compare_at_price_min: Option<i64>,
    pub compare_at_price_max: Option<i64>,
    pub on_sale: bool,

    pub in_stock: bool,
    pub total_inventory: i32,
    pub variant_count: i32,
    pub options: Vec<IndexProductOption>,

    pub thumbnail_url: Option<String>,
    pub images: Vec<IndexProductImage>,

    pub meta_title: Option<String>,
    pub meta_description: Option<String>,

    pub attributes: serde_json::Value,

    pub sales_count: i32,
    pub view_count: i32,
    pub rating: Option<f32>,
    pub review_count: i32,

    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
