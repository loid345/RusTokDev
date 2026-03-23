use leptos_graphql::{execute as execute_graphql, GraphqlHttpError, GraphqlRequest};
use serde::{Deserialize, Serialize};

use crate::model::StorefrontPagesData;

pub type ApiError = GraphqlHttpError;

const STOREFRONT_PAGES_QUERY: &str = "query StorefrontPages($homeSlug: String!, $filter: ListGqlPagesFilter) { home: pageBySlug(slug: $homeSlug) { effectiveLocale translation { locale title slug metaTitle metaDescription } body { locale content format } } pages(filter: $filter) { total items { id title slug status template } } }";

#[derive(Debug, Deserialize)]
struct StorefrontPagesResponse {
    home: Option<crate::model::PageDetail>,
    pages: crate::model::PageList,
}

#[derive(Debug, Serialize)]
struct StorefrontPagesVariables {
    #[serde(rename = "homeSlug")]
    home_slug: String,
    filter: ListPagesFilter,
}

#[derive(Debug, Serialize)]
struct ListPagesFilter {
    page: u64,
    #[serde(rename = "perPage")]
    per_page: u64,
}

fn configured_tenant_slug() -> Option<String> {
    [
        "RUSTOK_TENANT_SLUG",
        "NEXT_PUBLIC_TENANT_SLUG",
        "NEXT_PUBLIC_DEFAULT_TENANT_SLUG",
    ]
    .into_iter()
    .find_map(|key| {
        std::env::var(key).ok().and_then(|value| {
            let trimmed = value.trim().to_string();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        })
    })
}

fn graphql_url() -> String {
    if let Some(url) = option_env!("RUSTOK_GRAPHQL_URL") {
        return url.to_string();
    }

    #[cfg(target_arch = "wasm32")]
    {
        let origin = web_sys::window()
            .and_then(|window| window.location().origin().ok())
            .unwrap_or_else(|| "http://localhost:5150".to_string());
        format!("{origin}/api/graphql")
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let base =
            std::env::var("RUSTOK_API_URL").unwrap_or_else(|_| "http://localhost:5150".to_string());
        format!("{base}/api/graphql")
    }
}

async fn request<V, T>(query: &str, variables: V) -> Result<T, ApiError>
where
    V: Serialize,
    T: for<'de> Deserialize<'de>,
{
    execute_graphql(
        &graphql_url(),
        GraphqlRequest::new(query, Some(variables)),
        None,
        configured_tenant_slug(),
        None,
    )
    .await
}

pub async fn fetch_storefront_pages() -> Result<StorefrontPagesData, ApiError> {
    let response: StorefrontPagesResponse = request(
        STOREFRONT_PAGES_QUERY,
        StorefrontPagesVariables {
            home_slug: "home".to_string(),
            filter: ListPagesFilter {
                page: 1,
                per_page: 6,
            },
        },
    )
    .await?;

    Ok(StorefrontPagesData {
        home: response.home,
        pages: response.pages,
    })
}
