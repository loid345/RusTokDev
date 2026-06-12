use leptos::prelude::*;

use crate::model::{
    InventoryAdminBootstrap, InventoryAvailabilityCheckResult, InventoryProductDetail,
    InventoryProductList, InventoryQuantityWriteResult, InventoryReservationReleaseWriteResult,
    InventoryReservationWriteResult,
};

pub(crate) async fn fetch_bootstrap() -> Result<InventoryAdminBootstrap, ServerFnError> {
    crate::native::fetch_bootstrap().await
}

pub(crate) async fn fetch_products(
    tenant_id: String,
    locale: Option<String>,
    search: Option<String>,
    status: Option<String>,
) -> Result<InventoryProductList, ServerFnError> {
    crate::native::fetch_products(tenant_id, locale, search, status).await
}

pub(crate) async fn fetch_product(
    tenant_id: String,
    id: String,
    locale: Option<String>,
) -> Result<Option<InventoryProductDetail>, ServerFnError> {
    crate::native::fetch_product(tenant_id, id, locale).await
}

pub(crate) async fn set_variant_quantity(
    tenant_id: String,
    variant_id: String,
    quantity: i32,
) -> Result<InventoryQuantityWriteResult, ServerFnError> {
    crate::native::set_variant_quantity(tenant_id, variant_id, quantity).await
}

pub(crate) async fn adjust_variant_quantity(
    tenant_id: String,
    variant_id: String,
    adjustment: i32,
) -> Result<InventoryQuantityWriteResult, ServerFnError> {
    crate::native::adjust_variant_quantity(tenant_id, variant_id, adjustment).await
}

pub(crate) async fn reserve_variant_quantity(
    tenant_id: String,
    variant_id: String,
    quantity: i32,
) -> Result<InventoryReservationWriteResult, ServerFnError> {
    crate::native::reserve_variant_quantity(tenant_id, variant_id, quantity).await
}

pub(crate) async fn check_variant_availability(
    tenant_id: String,
    variant_id: String,
    requested_quantity: i32,
) -> Result<InventoryAvailabilityCheckResult, ServerFnError> {
    crate::native::check_variant_availability(tenant_id, variant_id, requested_quantity).await
}

pub(crate) async fn release_reservation_quantity(
    tenant_id: String,
    variant_id: String,
    quantity: i32,
) -> Result<InventoryReservationReleaseWriteResult, ServerFnError> {
    crate::native::release_reservation_quantity(tenant_id, variant_id, quantity).await
}
