use async_graphql::{Context, Object, Result};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use rustok_outbox::TransactionalEventBus;
use rustok_pages::PageService;

use crate::context::AuthContext;
use rustok_core::SecurityContext;

use super::types::*;

#[derive(Default)]
pub struct PagesQuery;

#[Object]
impl PagesQuery {
    async fn page(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
        id: Uuid,
    ) -> Result<Option<GqlPage>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let security = auth_context_to_security(ctx);

        let service = PageService::new(db.clone(), event_bus.clone());
        match service.get(tenant_id, security, id).await {
            Ok(page) => Ok(Some(page.into())),
            Err(rustok_pages::PagesError::PageNotFound(_)) => Ok(None),
            Err(err) => Err(async_graphql::Error::new(err.to_string())),
        }
    }

    async fn page_by_slug(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
        locale: String,
        slug: String,
    ) -> Result<Option<GqlPage>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let security = auth_context_to_security(ctx);

        let service = PageService::new(db.clone(), event_bus.clone());
        let page = service
            .get_by_slug(tenant_id, security, &locale, &slug)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(page.map(Into::into))
    }

    async fn pages(
        &self,
        ctx: &Context<'_>,
        tenant_id: Uuid,
        filter: Option<ListGqlPagesFilter>,
    ) -> Result<GqlPageList> {
        let db = ctx.data::<DatabaseConnection>()?;
        let event_bus = ctx.data::<TransactionalEventBus>()?;
        let security = auth_context_to_security(ctx);

        let filter = filter.unwrap_or(ListGqlPagesFilter {
            locale: None,
            template: None,
            page: Some(1),
            per_page: Some(20),
        });

        let service = PageService::new(db.clone(), event_bus.clone());
        let (items, total) = service
            .list(
                tenant_id,
                security,
                rustok_pages::ListPagesFilter {
                    status: None,
                    template: filter.template,
                    locale: filter.locale,
                    page: filter.page.unwrap_or(1),
                    per_page: filter.per_page.unwrap_or(20),
                },
            )
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(GqlPageList {
            items: items.into_iter().map(Into::into).collect(),
            total,
        })
    }
}

fn auth_context_to_security(ctx: &Context<'_>) -> SecurityContext {
    ctx.data::<AuthContext>()
        .map(|a| a.security_context())
        .unwrap_or_else(|_| SecurityContext::system())
}
