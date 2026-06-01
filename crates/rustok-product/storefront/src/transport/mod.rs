mod graphql_adapter;
mod native_server_adapter;

use crate::api::ApiError;
use crate::core::ProductStorefrontFetchRequest;
use crate::model::StorefrontProductsData;

pub async fn fetch_products(
    request: ProductStorefrontFetchRequest,
) -> Result<StorefrontProductsData, ApiError> {
    match native_server_adapter::fetch_products(request.clone()).await {
        Ok(data) => Ok(data),
        Err(_) => graphql_adapter::fetch_products(request).await,
    }
}
