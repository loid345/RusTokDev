use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductStatus {
    Draft,
    Active,
    Archived,
}

impl From<ProductStatus> for String {
    fn from(status: ProductStatus) -> Self {
        match status {
            ProductStatus::Draft => "draft".to_string(),
            ProductStatus::Active => "active".to_string(),
            ProductStatus::Archived => "archived".to_string(),
        }
    }
}

impl From<String> for ProductStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "active" => ProductStatus::Active,
            "archived" => ProductStatus::Archived,
            _ => ProductStatus::Draft,
        }
    }
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "products")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub status: String,
    pub vendor: Option<String>,
    pub product_type: Option<String>,
    pub metadata: Json,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub published_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::product_translation::Entity")]
    Translations,
    #[sea_orm(has_many = "super::product_variant::Entity")]
    Variants,
    #[sea_orm(has_many = "super::product_option::Entity")]
    Options,
    #[sea_orm(has_many = "super::product_image::Entity")]
    Images,
}

impl Related<super::product_translation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Translations.def()
    }
}

impl Related<super::product_variant::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Variants.def()
    }
}

impl Related<super::product_option::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Options.def()
    }
}

impl Related<super::product_image::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Images.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
