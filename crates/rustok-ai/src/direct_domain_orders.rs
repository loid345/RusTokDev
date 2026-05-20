#![cfg(feature = "server")]

use std::sync::Arc;

use super::direct_order_tasks::{OrderAnalyticsHandler, OrderOpsAssistantHandler};
use super::{DirectExecutionRegistry, DirectTaskHandler};

/// Registers order-owned AI direct handlers as a separate domain slice.
pub fn register_order_direct_handlers(registry: &mut DirectExecutionRegistry) {
    registry.register(Arc::new(OrderAnalyticsHandler) as Arc<dyn DirectTaskHandler>);
    registry.register(Arc::new(OrderOpsAssistantHandler) as Arc<dyn DirectTaskHandler>);
}
