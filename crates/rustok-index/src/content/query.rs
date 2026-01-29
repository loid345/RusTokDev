use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::model::IndexContentModel;
use crate::error::IndexResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentSortBy {
    PublishedAt,
    CreatedAt,
    UpdatedAt,
    Title,
    Position,
    ViewCount,
    ReplyCount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct ContentQuery {
    pub tenant_id: Uuid,
    pub locale: String,
    pub kind: Option<String>,
    pub status: Option<String>,
    pub category_id: Option<Uuid>,
    pub author_id: Option<Uuid>,
    pub tag_slug: Option<String>,
    pub search: Option<String>,
    pub parent_id: Option<Uuid>,
    pub sort_by: ContentSortBy,
    pub sort_order: SortOrder,
    pub limit: u64,
    pub offset: u64,
}

pub struct ContentQueryBuilder {
    query: ContentQuery,
}

impl ContentQueryBuilder {
    pub fn new(tenant_id: Uuid, locale: impl Into<String>) -> Self {
        Self {
            query: ContentQuery {
                tenant_id,
                locale: locale.into(),
                kind: None,
                status: Some("published".to_string()),
                category_id: None,
                author_id: None,
                tag_slug: None,
                search: None,
                parent_id: None,
                sort_by: ContentSortBy::PublishedAt,
                sort_order: SortOrder::Desc,
                limit: 20,
                offset: 0,
            },
        }
    }

    pub fn kind(mut self, kind: impl Into<String>) -> Self {
        self.query.kind = Some(kind.into());
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.query.status = Some(status.into());
        self
    }

    pub fn category(mut self, category_id: Uuid) -> Self {
        self.query.category_id = Some(category_id);
        self
    }

    pub fn author(mut self, author_id: Uuid) -> Self {
        self.query.author_id = Some(author_id);
        self
    }

    pub fn tag(mut self, tag_slug: impl Into<String>) -> Self {
        self.query.tag_slug = Some(tag_slug.into());
        self
    }

    pub fn search(mut self, q: impl Into<String>) -> Self {
        self.query.search = Some(q.into());
        self
    }

    pub fn parent(mut self, parent_id: Uuid) -> Self {
        self.query.parent_id = Some(parent_id);
        self
    }

    pub fn sort(mut self, sort_by: ContentSortBy, order: SortOrder) -> Self {
        self.query.sort_by = sort_by;
        self.query.sort_order = order;
        self
    }

    pub fn paginate(mut self, limit: u64, offset: u64) -> Self {
        self.query.limit = limit;
        self.query.offset = offset;
        self
    }

    pub fn build(self) -> ContentQuery {
        self.query
    }
}

/// Query service for content index
pub struct ContentQueryService {
    db: DatabaseConnection,
}

impl ContentQueryService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find(&self, _query: ContentQuery) -> IndexResult<Vec<IndexContentModel>> {
        Ok(vec![])
    }

    pub async fn find_by_slug(
        &self,
        _tenant_id: Uuid,
        _locale: &str,
        _kind: &str,
        _slug: &str,
    ) -> IndexResult<Option<IndexContentModel>> {
        Ok(None)
    }

    pub async fn count(&self, _query: ContentQuery) -> IndexResult<u64> {
        Ok(0)
    }

    pub async fn search(
        &self,
        _tenant_id: Uuid,
        _locale: &str,
        _q: &str,
        _limit: u64,
    ) -> IndexResult<Vec<IndexContentModel>> {
        Ok(vec![])
    }
}
