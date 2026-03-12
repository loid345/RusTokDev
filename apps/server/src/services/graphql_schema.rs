use std::sync::Arc;

use loco_rs::app::AppContext;

use crate::graphql::alloy::AlloyState;
use crate::graphql::{build_schema, AppSchema, SharedGraphqlSchema};
use crate::services::build_event_hub::build_event_hub_from_context;
use crate::services::event_bus::{event_bus_from_context, transactional_event_bus_from_context};

pub fn init_graphql_schema(ctx: &AppContext, alloy_state: AlloyState) -> Arc<AppSchema> {
    if let Some(shared) = ctx.shared_store.get::<SharedGraphqlSchema>() {
        return shared.0.clone();
    }

    let schema = Arc::new(build_schema(
        ctx.db.clone(),
        event_bus_from_context(ctx),
        transactional_event_bus_from_context(ctx),
        build_event_hub_from_context(ctx),
        alloy_state,
    ));

    ctx.shared_store.insert(SharedGraphqlSchema(schema.clone()));

    schema
}
