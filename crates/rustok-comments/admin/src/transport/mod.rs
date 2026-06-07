//! Module-owned transport facade for the comments admin UI.
//!
//! The current admin surface intentionally exposes a temporary single-adapter
//! native server-function transport. Keeping this facade between `ui/leptos` and
//! adapter implementation code prevents render code from owning transport calls
//! and leaves room for a future GraphQL/headless fallback adapter without
//! changing UI state wiring.

pub(crate) mod native_server_adapter;

pub(crate) use crate::api::ApiError;
use crate::api::CommentThreadsPayload;
use rustok_comments::{
    CommentRecord, CommentStatus, CommentThreadDetail, CommentThreadStatus, CommentThreadSummary,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CommentsAdminTransportPath {
    NativeServer,
}

pub(crate) const ACTIVE_TRANSPORT_PATH: CommentsAdminTransportPath =
    CommentsAdminTransportPath::NativeServer;

pub(crate) async fn fetch_threads(
    page: u64,
    per_page: u64,
    target_type: String,
    thread_status: Option<CommentThreadStatus>,
    comment_status: Option<CommentStatus>,
) -> Result<CommentThreadsPayload, ApiError> {
    match ACTIVE_TRANSPORT_PATH {
        CommentsAdminTransportPath::NativeServer => {
            native_server_adapter::fetch_threads(
                page,
                per_page,
                target_type,
                thread_status,
                comment_status,
            )
            .await
        }
    }
}

pub(crate) async fn fetch_thread_detail(
    thread_id: String,
    locale: String,
    page: u64,
    per_page: u64,
) -> Result<CommentThreadDetail, ApiError> {
    match ACTIVE_TRANSPORT_PATH {
        CommentsAdminTransportPath::NativeServer => {
            native_server_adapter::fetch_thread_detail(thread_id, locale, page, per_page).await
        }
    }
}

pub(crate) async fn set_thread_status(
    thread_id: String,
    status: CommentThreadStatus,
) -> Result<CommentThreadSummary, ApiError> {
    match ACTIVE_TRANSPORT_PATH {
        CommentsAdminTransportPath::NativeServer => {
            native_server_adapter::set_thread_status(thread_id, status).await
        }
    }
}

pub(crate) async fn set_comment_status(
    comment_id: String,
    status: CommentStatus,
    locale: String,
) -> Result<CommentRecord, ApiError> {
    match ACTIVE_TRANSPORT_PATH {
        CommentsAdminTransportPath::NativeServer => {
            native_server_adapter::set_comment_status(comment_id, status, locale).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_transport_path_documents_temporary_native_only_state() {
        assert_eq!(
            ACTIVE_TRANSPORT_PATH,
            CommentsAdminTransportPath::NativeServer
        );
    }
}
