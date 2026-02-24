use async_graphql::{Context, FieldError, Object, Result};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::context::AuthContext;
use crate::graphql::errors::GraphQLError;
use rustok_blog::{PostService, UpdatePostInput as DomainUpdatePostInput};
use rustok_outbox::TransactionalEventBus;

use super::types::*;

#[derive(Default)]
pub struct BlogMutation;

#[Object]
impl BlogMutation {
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
        input: CreatePostInput,
    ) -> Result<Uuid> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <FieldError as GraphQLError>::unauthenticated())?;
        let security = auth.security_context();

        let service = PostService::new(db.clone(), event_bus.clone());
        let post_id = service
            .create_post(tenant_id, security, input.into())
            .await?;

        Ok(post_id)
    }

    async fn update_post(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: UpdatePostInput,
    ) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <FieldError as GraphQLError>::unauthenticated())?;
        let security = auth.security_context();

        let service = PostService::new(db.clone(), event_bus.clone());

        let domain_input = DomainUpdatePostInput {
            locale: input.locale,
            title: input.title,
            body: input.body,
            excerpt: input.excerpt,
            slug: input.slug,
            tags: input.tags,
            category_id: input.category_id,
            featured_image_url: input.featured_image_url,
            seo_title: input.seo_title,
            seo_description: input.seo_description,
            metadata: None,
            version: None,
        };

        service.update_post(id, security, domain_input).await?;

        Ok(true)
    }

    async fn delete_post(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <FieldError as GraphQLError>::unauthenticated())?;
        let security = auth.security_context();

        let service = PostService::new(db.clone(), event_bus.clone());
        service.delete_post(id, security).await?;

        Ok(true)
    }

    async fn publish_post(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <FieldError as GraphQLError>::unauthenticated())?;
        let security = auth.security_context();

        let service = PostService::new(db.clone(), event_bus.clone());
        service.publish_post(id, security).await?;

        Ok(true)
    }

    async fn unpublish_post(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <FieldError as GraphQLError>::unauthenticated())?;
        let security = auth.security_context();

        let service = PostService::new(db.clone(), event_bus.clone());
        service.unpublish_post(id, security).await?;

        Ok(true)
    }

    async fn archive_post(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        reason: Option<String>,
    ) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <FieldError as GraphQLError>::unauthenticated())?;
        let security = auth.security_context();

        let service = PostService::new(db.clone(), event_bus.clone());
        service.archive_post(id, security, reason).await?;

        Ok(true)
    }
}
