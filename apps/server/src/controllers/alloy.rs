use alloy_scripting::{AppState, SeaOrmStorage};
use axum::Router;
use std::sync::Arc;

pub fn router(state: Arc<AppState<SeaOrmStorage>>) -> Router {
    alloy_scripting::create_router(state)
}
