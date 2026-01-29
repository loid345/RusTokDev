use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::model::IndexProductModel;
use crate::error::IndexResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProductSortBy {
    PublishedAt,
    CreatedAt,
    UpdatedAt,
    Title,
    PriceMin,
    PriceMax,
    SalesCount,
    ViewCount,
    Rating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct ProductQuery {
    pub tenant_id: Uuid,
    pub locale: String,
    pub category_id: Option<Uuid>,
    pub tag: Option<String>,
    pub search: Option<String>,
    pub in_stock: Option<bool>,
    pub is_published: Option<bool>,
    pub price_min: Option<i64>,
    pub price_max: Option<i64>,
    pub sort_by: ProductSortBy,
    pub sort_order: SortOrder,
    pub limit: u64,
    pub offset: u64,
}

pub struct ProductQueryBuilder {
    query: ProductQuery,
}

impl ProductQueryBuilder {
    pub fn new(tenant_id: Uuid, locale: impl Into<String>) -> Self {
        Self {
            query: ProductQuery {
                tenant_id,
                locale: locale.into(),
                category_id: None,
                tag: None,
                search: None,
                in_stock: None,
                is_published: Some(true),
                price_min: None,
                price_max: None,
                sort_by: ProductSortBy::PublishedAt,
                sort_order: SortOrder::Desc,
                limit: 20,
                offset: 0,
            },
        }
    }

    pub fn category(mut self, category_id: Uuid) -> Self {
        self.query.category_id = Some(category_id);
        self
    }

    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.query.tag = Some(tag.into());
        self
    }

    pub fn search(mut self, q: impl Into<String>) -> Self {
        self.query.search = Some(q.into());
        self
    }

    pub fn in_stock(mut self, in_stock: bool) -> Self {
        self.query.in_stock = Some(in_stock);
        self
    }

    pub fn published(mut self, published: bool) -> Self {
        self.query.is_published = Some(published);
        self
    }

    pub fn price_range(mut self, min: Option<i64>, max: Option<i64>) -> Self {
        self.query.price_min = min;
        self.query.price_max = max;
        self
    }

    pub fn sort(mut self, sort_by: ProductSortBy, order: SortOrder) -> Self {
        self.query.sort_by = sort_by;
        self.query.sort_order = order;
        self
    }

    pub fn paginate(mut self, limit: u64, offset: u64) -> Self {
        self.query.limit = limit;
        self.query.offset = offset;
        self
    }

    pub fn build(self) -> ProductQuery {
        self.query
    }
}

/// Query service for product index
pub struct ProductQueryService {
    db: DatabaseConnection,
}

impl ProductQueryService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find(&self, _query: ProductQuery) -> IndexResult<Vec<IndexProductModel>> {
        Ok(vec![])
    }

    pub async fn find_by_handle(
        &self,
        _tenant_id: Uuid,
        _locale: &str,
        _handle: &str,
    ) -> IndexResult<Option<IndexProductModel>> {
        Ok(None)
    }

    pub async fn count(&self, _query: ProductQuery) -> IndexResult<u64> {
        Ok(0)
    }
}
