use crate::api::{self, ApiError, CommentThreadsPayload};
use crate::core::{
    CommentThreadDetailRequest, CommentThreadsRequest, SetCommentStatusCommand,
    SetThreadStatusCommand,
};
use rustok_comments::{CommentRecord, CommentThreadDetail, CommentThreadSummary};

pub(crate) async fn fetch_threads(
    request: CommentThreadsRequest,
) -> Result<CommentThreadsPayload, ApiError> {
    api::fetch_threads(
        request.page,
        request.per_page,
        request.target_type,
        request.thread_status,
        request.comment_status,
    )
    .await
}

pub(crate) async fn fetch_thread_detail(
    request: CommentThreadDetailRequest,
) -> Result<CommentThreadDetail, ApiError> {
    api::fetch_thread_detail(
        request.thread_id,
        request.locale,
        request.page,
        request.per_page,
    )
    .await
}

pub(crate) async fn set_thread_status(
    command: SetThreadStatusCommand,
) -> Result<CommentThreadSummary, ApiError> {
    api::set_thread_status(command.thread_id, command.status).await
}

pub(crate) async fn set_comment_status(
    command: SetCommentStatusCommand,
) -> Result<CommentRecord, ApiError> {
    api::set_comment_status(command.comment_id, command.status, command.locale).await
}
