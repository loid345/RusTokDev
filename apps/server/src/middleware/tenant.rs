use axum::{
    body::Body,
    extract::State,
    http::{header::HOST, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use loco_rs::prelude::*;
use uuid::Uuid;

use crate::context::{TenantContext, TenantContextExt, TenantError};
use crate::models::tenants;

pub async fn resolve(
    State(ctx): State<AppContext>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    if should_skip_tenant(req.uri().path()) {
        return next.run(req).await;
    }

    match resolve_tenant(&ctx, &req).await {
        Ok(tenant_ctx) => {
            req.extensions_mut().insert(TenantContextExt(tenant_ctx));
            next.run(req).await
        }
        Err(err) => {
            let status = err.status_code();
            let body = serde_json::json!({
                "error": err.to_string(),
                "code": "TENANT_ERROR",
            });
            (status, axum::Json(body)).into_response()
        }
    }
}

async fn resolve_tenant(ctx: &AppContext, req: &Request<Body>) -> Result<TenantContext, TenantError> {
    let identifier = extract_tenant_identifier(req).ok_or(TenantError::MissingIdentifier)?;

    let tenant = if let Ok(uuid) = Uuid::parse_str(&identifier) {
        tenants::Entity::find_by_id(&ctx.db, uuid).await?
    } else if let Some(tenant) = tenants::Entity::find_by_slug(&ctx.db, &identifier).await? {
        Some(tenant)
    } else if let Some(tenant) = tenants::Entity::find_by_domain(&ctx.db, &identifier).await? {
        Some(tenant)
    } else {
        None
    };

    let tenant = tenant.ok_or(TenantError::NotFound)?;
    if !tenant.is_enabled() {
        return Err(TenantError::Disabled);
    }

    Ok(TenantContext::from_model(&tenant))
}

fn extract_tenant_identifier(req: &Request<Body>) -> Option<String> {
    header_value(req, "x-tenant-id")
        .or_else(|| header_value(req, "x-tenant-slug"))
        .or_else(|| path_identifier(req))
        .or_else(|| host_identifier(req))
}

fn should_skip_tenant(path: &str) -> bool {
    path.starts_with("/health")
}

fn header_value(req: &Request<Body>, name: &str) -> Option<String> {
    req.headers()
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(String::from)
}

fn host_identifier(req: &Request<Body>) -> Option<String> {
    let host = req.headers().get(HOST)?.to_str().ok()?;
    let trimmed = host.trim();
    if trimmed.is_empty() {
        return None;
    }
    let host_only = trimmed.split(':').next().unwrap_or(trimmed);
    if host_only.is_empty() {
        return None;
    }
    Some(host_only.to_string())
}

fn path_identifier(req: &Request<Body>) -> Option<String> {
    let path = req.uri().path();
    if !(path.starts_with("/t/") || path.starts_with("/api/t/")) {
        return None;
    }

    let segments: Vec<&str> = path.split('/').collect();
    segments
        .iter()
        .position(|&segment| segment == "t")
        .and_then(|index| segments.get(index + 1))
        .filter(|segment| !segment.is_empty())
        .map(|segment| segment.to_string())
}
