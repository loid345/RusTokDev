use async_graphql::{InputObject, SimpleObject};
use uuid::Uuid;

#[derive(Clone, Debug, SimpleObject)]
pub struct ForumCategory {
    pub id: Uuid,
    pub locale: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub parent_id: Option<Uuid>,
    pub position: i32,
    pub topic_count: i32,
    pub reply_count: i32,
    pub moderated: bool,
}

#[derive(Clone, Debug, SimpleObject)]
pub struct ForumTopic {
    pub id: Uuid,
    pub locale: String,
    pub category_id: Uuid,
    pub title: String,
    pub status: String,
    pub is_pinned: bool,
    pub is_locked: bool,
    pub reply_count: i32,
    pub created_at: String,
}

#[derive(Clone, Debug, SimpleObject)]
pub struct ForumTopicDetail {
    pub id: Uuid,
    pub locale: String,
    pub category_id: Uuid,
    pub title: String,
    pub body: String,
    pub status: String,
    pub tags: Vec<String>,
    pub is_pinned: bool,
    pub is_locked: bool,
    pub reply_count: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, SimpleObject)]
pub struct ForumReply {
    pub id: Uuid,
    pub locale: String,
    pub topic_id: Uuid,
    pub content: String,
    pub status: String,
    pub parent_reply_id: Option<Uuid>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, SimpleObject)]
pub struct ForumReplyListItem {
    pub id: Uuid,
    pub locale: String,
    pub topic_id: Uuid,
    pub content_preview: String,
    pub status: String,
    pub created_at: String,
}

#[derive(InputObject)]
pub struct CreateForumCategoryInput {
    pub locale: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub parent_id: Option<Uuid>,
    pub position: Option<i32>,
    pub moderated: bool,
}

#[derive(InputObject)]
pub struct UpdateForumCategoryInput {
    pub locale: String,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub position: Option<i32>,
    pub moderated: Option<bool>,
}

#[derive(InputObject)]
pub struct CreateForumTopicInput {
    pub locale: String,
    pub category_id: Uuid,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
}

#[derive(InputObject)]
pub struct UpdateForumTopicInput {
    pub locale: String,
    pub title: Option<String>,
    pub body: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(InputObject)]
pub struct ListForumTopicsInput {
    pub category_id: Option<Uuid>,
    pub status: Option<String>,
    pub locale: Option<String>,
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

#[derive(InputObject)]
pub struct CreateForumReplyInput {
    pub locale: String,
    pub content: String,
    pub parent_reply_id: Option<Uuid>,
}

#[derive(InputObject)]
pub struct UpdateForumReplyInput {
    pub locale: String,
    pub content: Option<String>,
}

impl From<rustok_forum::CategoryResponse> for ForumCategory {
    fn from(r: rustok_forum::CategoryResponse) -> Self {
        Self {
            id: r.id,
            locale: r.locale,
            name: r.name,
            slug: r.slug,
            description: r.description,
            icon: r.icon,
            color: r.color,
            parent_id: r.parent_id,
            position: r.position,
            topic_count: r.topic_count,
            reply_count: r.reply_count,
            moderated: r.moderated,
        }
    }
}

impl From<rustok_forum::CategoryListItem> for ForumCategory {
    fn from(r: rustok_forum::CategoryListItem) -> Self {
        Self {
            id: r.id,
            locale: r.locale,
            name: r.name,
            slug: r.slug,
            description: r.description,
            icon: r.icon,
            color: r.color,
            parent_id: None,
            position: 0,
            topic_count: r.topic_count,
            reply_count: r.reply_count,
            moderated: false,
        }
    }
}

impl From<rustok_forum::TopicListItem> for ForumTopic {
    fn from(r: rustok_forum::TopicListItem) -> Self {
        Self {
            id: r.id,
            locale: r.locale,
            category_id: r.category_id,
            title: r.title,
            status: r.status,
            is_pinned: r.is_pinned,
            is_locked: r.is_locked,
            reply_count: r.reply_count,
            created_at: r.created_at,
        }
    }
}

impl From<rustok_forum::TopicResponse> for ForumTopicDetail {
    fn from(r: rustok_forum::TopicResponse) -> Self {
        Self {
            id: r.id,
            locale: r.locale,
            category_id: r.category_id,
            title: r.title,
            body: r.body,
            status: r.status,
            tags: r.tags,
            is_pinned: r.is_pinned,
            is_locked: r.is_locked,
            reply_count: r.reply_count,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

impl From<rustok_forum::ReplyResponse> for ForumReply {
    fn from(r: rustok_forum::ReplyResponse) -> Self {
        Self {
            id: r.id,
            locale: r.locale,
            topic_id: r.topic_id,
            content: r.content,
            status: r.status,
            parent_reply_id: r.parent_reply_id,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

impl From<rustok_forum::ReplyListItem> for ForumReplyListItem {
    fn from(r: rustok_forum::ReplyListItem) -> Self {
        Self {
            id: r.id,
            locale: r.locale,
            topic_id: r.topic_id,
            content_preview: r.content_preview,
            status: r.status,
            created_at: r.created_at,
        }
    }
}
