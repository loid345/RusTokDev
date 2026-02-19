use async_graphql::{Context, Object, Result};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use rustok_forum::{CategoryService, ReplyService, TopicService};
use rustok_outbox::TransactionalEventBus;

use crate::context::AuthContext;
use rustok_core::SecurityContext;

use super::types::*;

#[derive(Default)]
pub struct ForumQuery;

#[Object]
impl ForumQuery {
    async fn forum_categories(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
        locale: Option<String>,
    ) -> Result<Vec<ForumCategory>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let locale = locale.unwrap_or_else(|| "en".to_string());
        let security = auth_context_to_security(ctx);

        let service = CategoryService::new(db.clone(), event_bus.clone());
        let categories = service
            .list(tenant_id, security, &locale)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(categories.into_iter().map(Into::into).collect())
    }

    async fn forum_category(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
        id: Uuid,
        locale: Option<String>,
    ) -> Result<Option<ForumCategory>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let locale = locale.unwrap_or_else(|| "en".to_string());

        let service = CategoryService::new(db.clone(), event_bus.clone());
        match service.get(tenant_id, id, &locale).await {
            Ok(category) => Ok(Some(category.into())),
            Err(rustok_forum::ForumError::CategoryNotFound(_)) => Ok(None),
            Err(err) => Err(async_graphql::Error::new(err.to_string())),
        }
    }

    async fn forum_topics(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
        filter: Option<ListForumTopicsInput>,
    ) -> Result<Vec<ForumTopic>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let security = auth_context_to_security(ctx);

        let filter = filter.unwrap_or(ListForumTopicsInput {
            category_id: None,
            status: None,
            locale: None,
            page: Some(1),
            per_page: Some(20),
        });

        let service = TopicService::new(db.clone(), event_bus.clone());
        let (topics, _total) = service
            .list(
                tenant_id,
                security,
                rustok_forum::ListTopicsFilter {
                    category_id: filter.category_id,
                    status: filter.status,
                    locale: filter.locale,
                    page: filter.page.unwrap_or(1),
                    per_page: filter.per_page.unwrap_or(20),
                },
            )
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(topics.into_iter().map(Into::into).collect())
    }

    async fn forum_topic(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        locale: Option<String>,
    ) -> Result<Option<ForumTopicDetail>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let locale = locale.unwrap_or_else(|| "en".to_string());

        let service = TopicService::new(db.clone(), event_bus.clone());
        match service.get(id, &locale).await {
            Ok(topic) => Ok(Some(topic.into())),
            Err(rustok_forum::ForumError::TopicNotFound(_)) => Ok(None),
            Err(err) => Err(async_graphql::Error::new(err.to_string())),
        }
    }

    async fn forum_replies(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
        topic_id: Uuid,
        locale: Option<String>,
    ) -> Result<Vec<ForumReplyListItem>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let locale = locale.unwrap_or_else(|| "en".to_string());
        let security = auth_context_to_security(ctx);

        let service = ReplyService::new(db.clone(), event_bus.clone());
        let replies = service
            .list_for_topic(tenant_id, security, topic_id, &locale)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(replies.into_iter().map(Into::into).collect())
    }
}

fn auth_context_to_security(ctx: &Context<'_>) -> SecurityContext {
    ctx.data::<AuthContext>()
        .map(|a| a.security_context())
        .unwrap_or_else(|_| SecurityContext::system())
}
