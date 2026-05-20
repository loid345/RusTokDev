#![cfg(feature = "server")]

use std::sync::Arc;

use super::direct_content_moderation::ContentModerationHandler;
use super::{DirectExecutionRegistry, DirectTaskHandler};

/// Registers content-owned AI direct handlers as a separate domain slice.
pub fn register_content_direct_handlers(registry: &mut DirectExecutionRegistry) {
    registry.register(Arc::new(ContentModerationHandler) as Arc<dyn DirectTaskHandler>);
}
