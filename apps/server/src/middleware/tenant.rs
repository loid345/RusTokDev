use axum::{
    body::Body,
    extract::State,
    http::{
        header::{FORWARDED, HOST},
        Request, StatusCode,
    },
    middleware::Next,
    response::Response,
};
use loco_rs::app::AppContext;
use moka::future::Cache;
use once_cell::sync::Lazy;
use std::time::Duration;
use uuid::Uuid;

use crate::common::settings::RustokSettings;
use crate::context::{TenantContext, TenantContextExtension};
use crate::models::tenants;

// Tenant cache: identifier (host/slug/uuid) -> TenantContext
// TTL: 5 minutes, Max entries: 1000
static TENANT_CACHE: Lazy<Cache<String, TenantContext>> = Lazy::new(|| {
    Cache::builder()
        .time_to_live(Duration::from_secs(300))
        .max_capacity(1_000)
        .build()
});

pub async fn resolve(
    State(ctx): State<AppContext>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let settings = RustokSettings::from_settings(&ctx.config.settings)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let identifier = resolve_identifier(&req, &settings)?;

    // Check cache first
    if let Some(cached_context) = TENANT_CACHE.get(&identifier).await {
        req.extensions_mut()
            .insert(TenantContextExtension(cached_context));
        return Ok(next.run(req).await);
    }

    // Cache miss — query database
    let tenant = if let Ok(uuid) = Uuid::parse_str(&identifier) {
        tenants::Entity::find_by_id(&ctx.db, uuid)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else if let Some(tenant) = tenants::Entity::find_by_slug(&ctx.db, &identifier)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        Some(tenant)
    } else {
        tenants::Entity::find_by_domain(&ctx.db, &identifier)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    match tenant {
        Some(tenant) => {
            let context = TenantContext::from_model(&tenant);
            // Store in cache
            TENANT_CACHE.insert(identifier, context.clone()).await;
            req.extensions_mut().insert(TenantContextExtension(context));
            Ok(next.run(req).await)
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

fn resolve_identifier(
    req: &Request<Body>,
    settings: &RustokSettings,
) -> Result<String, StatusCode> {
    if !settings.tenant.enabled {
        return Ok(settings.tenant.default_id.to_string());
    }

    match settings.tenant.resolution.as_str() {
        "header" => {
            let header_value = req
                .headers()
                .get(&settings.tenant.header_name)
                .and_then(|value| value.to_str().ok());

            Ok(header_value
                .map(|value| value.to_string())
                .unwrap_or_else(|| settings.tenant.default_id.to_string()))
        }
        "host" | "domain" | "subdomain" => {
            let host = extract_host(req.headers()).ok_or(StatusCode::BAD_REQUEST)?;
            Ok(host.split(':').next().unwrap_or(host).to_string())
        }
        _ => Ok(settings.tenant.default_id.to_string()),
    }
}

fn extract_host(headers: &axum::http::HeaderMap) -> Option<&str> {
    if let Some(host) = headers
        .get("x-forwarded-host")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(',').next())
    {
        return Some(host.trim());
    }

    if let Some(forwarded) = headers.get(FORWARDED).and_then(|value| value.to_str().ok()) {
        if let Some(host) = parse_forwarded_host(forwarded) {
            return Some(host);
        }
    }

    headers.get(HOST).and_then(|value| value.to_str().ok())
}

fn parse_forwarded_host(forwarded: &str) -> Option<&str> {
    forwarded
        .split(',')
        .next()
        .and_then(|entry| {
            entry
                .split(';')
                .find(|part| part.trim_start().starts_with("host="))
        })
        .and_then(|part| part.trim_start().strip_prefix("host="))
        .map(|host| host.trim_matches('"').trim())
}

/// Invalidate cached tenant (call after tenant update)
pub async fn invalidate_tenant_cache(identifier: &str) {
    TENANT_CACHE.invalidate(identifier).await;
}
