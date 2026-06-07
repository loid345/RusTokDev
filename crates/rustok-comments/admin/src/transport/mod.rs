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
use crate::core::{
    CommentThreadDetailRequest, CommentThreadsRequest, SetCommentStatusCommand,
    SetThreadStatusCommand,
};
use rustok_comments::{CommentRecord, CommentThreadDetail, CommentThreadSummary};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CommentsAdminTransportPath {
    NativeServer,
}

pub(crate) const ACTIVE_TRANSPORT_PATH: CommentsAdminTransportPath =
    CommentsAdminTransportPath::NativeServer;

pub(crate) async fn fetch_threads(
    request: CommentThreadsRequest,
) -> Result<CommentThreadsPayload, ApiError> {
    match ACTIVE_TRANSPORT_PATH {
        CommentsAdminTransportPath::NativeServer => {
            native_server_adapter::fetch_threads(request).await
        }
    }
}

pub(crate) async fn fetch_thread_detail(
    request: CommentThreadDetailRequest,
) -> Result<CommentThreadDetail, ApiError> {
    match ACTIVE_TRANSPORT_PATH {
        CommentsAdminTransportPath::NativeServer => {
            native_server_adapter::fetch_thread_detail(request).await
        }
    }
}

pub(crate) async fn set_thread_status(
    command: SetThreadStatusCommand,
) -> Result<CommentThreadSummary, ApiError> {
    match ACTIVE_TRANSPORT_PATH {
        CommentsAdminTransportPath::NativeServer => {
            native_server_adapter::set_thread_status(command).await
        }
    }
}

pub(crate) async fn set_comment_status(
    command: SetCommentStatusCommand,
) -> Result<CommentRecord, ApiError> {
    match ACTIVE_TRANSPORT_PATH {
        CommentsAdminTransportPath::NativeServer => {
            native_server_adapter::set_comment_status(command).await
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
