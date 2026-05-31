use crate::api::{self, ApiError};
use crate::core::ProductStorefrontFetchRequest;
use crate::model::StorefrontProductsData;

pub async fn fetch_products(
    request: ProductStorefrontFetchRequest,
) -> Result<StorefrontProductsData, ApiError> {
    api::fetch_storefront_products(
        request.selected_handle,
        request.locale,
        request.currency_code,
        request.region_id,
        request.price_list_id,
        request.channel_id,
        request.channel_slug,
        request.quantity,
    )
    .await
}
