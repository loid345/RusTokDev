use async_graphql::{Context, Object, Result};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use rustok_forum::{CategoryService, ReplyService, TopicService};
use rustok_outbox::TransactionalEventBus;

use crate::context::AuthContext;
use rustok_core::SecurityContext;

use super::types::*;

#[derive(Default)]
pub struct ForumMutation;

#[Object]
impl ForumMutation {
    async fn create_forum_category(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
        input: CreateForumCategoryInput,
    ) -> Result<ForumCategory> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let security = auth_context_to_security(ctx);

        let service = CategoryService::new(db.clone(), event_bus.clone());
        let category = service
            .create(
                tenant_id,
                security,
                rustok_forum::CreateCategoryInput {
                    locale: input.locale,
                    name: input.name,
                    slug: input.slug,
                    description: input.description,
                    icon: input.icon,
                    color: input.color,
                    parent_id: input.parent_id,
                    position: input.position,
                    moderated: input.moderated,
                },
            )
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(category.into())
    }

    async fn update_forum_category(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
        id: Uuid,
        input: UpdateForumCategoryInput,
    ) -> Result<ForumCategory> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let security = auth_context_to_security(ctx);

        let service = CategoryService::new(db.clone(), event_bus.clone());
        let category = service
            .update(
                tenant_id,
                id,
                security,
                rustok_forum::UpdateCategoryInput {
                    locale: input.locale,
                    name: input.name,
                    slug: input.slug,
                    description: input.description,
                    icon: input.icon,
                    color: input.color,
                    position: input.position,
                    moderated: input.moderated,
                },
            )
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(category.into())
    }

    async fn delete_forum_category(
        &self,
        ctx: &Context<'_>,
        _tenant_id: Uuid,
        id: Uuid,
    ) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let security = auth_context_to_security(ctx);

        let service = CategoryService::new(db.clone(), event_bus.clone());
        service
            .delete(id, security)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(true)
    }

    async fn create_forum_topic(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
        input: CreateForumTopicInput,
    ) -> Result<ForumTopicDetail> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let security = auth_context_to_security(ctx);

        let service = TopicService::new(db.clone(), event_bus.clone());
        let topic = service
            .create(
                tenant_id,
                security,
                rustok_forum::CreateTopicInput {
                    locale: input.locale,
                    category_id: input.category_id,
                    title: input.title,
                    body: input.body,
                    tags: input.tags,
                },
            )
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(topic.into())
    }

    async fn update_forum_topic(
        &self,
        ctx: &Context<'_>,
        _tenant_id: Uuid,
        id: Uuid,
        input: UpdateForumTopicInput,
    ) -> Result<ForumTopicDetail> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let security = auth_context_to_security(ctx);

        let service = TopicService::new(db.clone(), event_bus.clone());
        let topic = service
            .update(
                id,
                security,
                rustok_forum::UpdateTopicInput {
                    locale: input.locale,
                    title: input.title,
                    body: input.body,
                    tags: input.tags,
                },
            )
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(topic.into())
    }

    async fn delete_forum_topic(
        &self,
        ctx: &Context<'_>,
        _tenant_id: Uuid,
        id: Uuid,
    ) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let security = auth_context_to_security(ctx);

        let service = TopicService::new(db.clone(), event_bus.clone());
        service
            .delete(id, security)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(true)
    }

    async fn create_forum_reply(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
        topic_id: Uuid,
        input: CreateForumReplyInput,
    ) -> Result<ForumReply> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let security = auth_context_to_security(ctx);

        let service = ReplyService::new(db.clone(), event_bus.clone());
        let reply = service
            .create(
                tenant_id,
                security,
                topic_id,
                rustok_forum::CreateReplyInput {
                    locale: input.locale,
                    content: input.content,
                    parent_reply_id: input.parent_reply_id,
                },
            )
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(reply.into())
    }

    async fn update_forum_reply(
        &self,
        ctx: &Context<'_>,
        _tenant_id: Uuid,
        id: Uuid,
        input: UpdateForumReplyInput,
    ) -> Result<ForumReply> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let security = auth_context_to_security(ctx);

        let service = ReplyService::new(db.clone(), event_bus.clone());
        let reply = service
            .update(
                id,
                security,
                rustok_forum::UpdateReplyInput {
                    locale: input.locale,
                    content: input.content,
                },
            )
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(reply.into())
    }

    async fn delete_forum_reply(
        &self,
        ctx: &Context<'_>,
        _tenant_id: Uuid,
        id: Uuid,
    ) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let security = auth_context_to_security(ctx);

        let service = ReplyService::new(db.clone(), event_bus.clone());
        service
            .delete(id, security)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(true)
    }
}

fn auth_context_to_security(ctx: &Context<'_>) -> SecurityContext {
    ctx.data::<AuthContext>()
        .map(|a| a.security_context())
        .unwrap_or_else(|_| SecurityContext::system())
}
