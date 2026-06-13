//! AI admin transport facade.
//!
//! This facade keeps the current native server-function implementation in
//! `api.rs` while preventing the Leptos adapter from depending on that raw
//! pre-FFA module directly. Splitting native/GraphQL adapters can happen behind
//! this module without changing the UI call sites.

pub use crate::api::{
    create_provider, create_task_profile, create_tool_profile, deactivate_provider,
    fetch_bootstrap, fetch_session, resume_approval, run_task_job, send_message, start_session,
    test_provider, update_provider, update_task_profile, update_tool_profile, ApiError,
};
