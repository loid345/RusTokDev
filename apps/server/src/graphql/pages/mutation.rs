use async_graphql::{Context, Object, Result};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use rustok_outbox::TransactionalEventBus;
use rustok_pages::{
    CreatePageInput, PageBodyInput, PageService, PageTranslationInput, UpdatePageInput,
};

use crate::context::AuthContext;
use rustok_core::SecurityContext;

use super::types::*;

#[derive(Default)]
pub struct PagesMutation;

#[Object]
impl PagesMutation {
    async fn create_page(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
        input: CreateGqlPageInput,
    ) -> Result<GqlPage> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let security = auth_context_to_security(ctx);

        let service = PageService::new(db.clone(), event_bus.clone());
        let page = service
            .create(
                tenant_id,
                security,
                CreatePageInput {
                    translations: input
                        .translations
                        .into_iter()
                        .map(|t| PageTranslationInput {
                            locale: t.locale,
                            title: t.title,
                            slug: t.slug,
                            meta_title: t.meta_title,
                            meta_description: t.meta_description,
                        })
                        .collect(),
                    template: input.template,
                    body: input.body.map(|b| PageBodyInput {
                        locale: b.locale,
                        content: b.content,
                        format: b.format,
                    }),
                    blocks: None,
                    publish: input.publish.unwrap_or(false),
                },
            )
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(page.into())
    }

    async fn update_page(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
        id: Uuid,
        input: UpdateGqlPageInput,
    ) -> Result<GqlPage> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let security = auth_context_to_security(ctx);

        let service = PageService::new(db.clone(), event_bus.clone());
        let page = service
            .update(
                tenant_id,
                security,
                id,
                UpdatePageInput {
                    translations: input.translations.map(|translations| {
                        translations
                            .into_iter()
                            .map(|t| PageTranslationInput {
                                locale: t.locale,
                                title: t.title,
                                slug: t.slug,
                                meta_title: t.meta_title,
                                meta_description: t.meta_description,
                            })
                            .collect()
                    }),
                    template: input.template,
                    body: input.body.map(|b| PageBodyInput {
                        locale: b.locale,
                        content: b.content,
                        format: b.format,
                    }),
                    status: None,
                },
            )
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(page.into())
    }

    async fn publish_page(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
        id: Uuid,
    ) -> Result<GqlPage> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let security = auth_context_to_security(ctx);

        let service = PageService::new(db.clone(), event_bus.clone());
        let page = service
            .publish(tenant_id, security, id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(page.into())
    }

    async fn unpublish_page(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
        id: Uuid,
    ) -> Result<GqlPage> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let security = auth_context_to_security(ctx);

        let service = PageService::new(db.clone(), event_bus.clone());
        let page = service
            .unpublish(tenant_id, security, id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(page.into())
    }

    async fn delete_page(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
        id: Uuid,
    ) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let security = auth_context_to_security(ctx);

        let service = PageService::new(db.clone(), event_bus.clone());
        service
            .delete(tenant_id, security, id)
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
