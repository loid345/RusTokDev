#![cfg(feature = "server")]

use std::sync::Arc;

use super::direct_product_attributes::ProductAttributesHandler;
use super::{DirectExecutionRegistry, DirectTaskHandler, ProductCopyHandler};

/// Registers commerce-owned AI direct handlers as a separate domain slice.
///
/// This keeps `rustok-ai` runtime/core handlers isolated from ecommerce verticals,
/// while preserving current behavior for default composition.
pub fn register_commerce_direct_handlers(registry: &mut DirectExecutionRegistry) {
    registry.register(Arc::new(ProductCopyHandler) as Arc<dyn DirectTaskHandler>);
    registry.register(Arc::new(ProductAttributesHandler) as Arc<dyn DirectTaskHandler>);
}
