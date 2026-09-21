
use loco_rs::Error;

use crate::ForumError;
use axum::{http::StatusCode, routing::get};
use loco_rs::controller::{ErrorDetail, Routes};

pub mod categories;
pub mod replies;
pub mod topics;
pub mod users;
pub mod widgets;

pub(crate) fn forum_forbidden(message: &str) -> Error {
    Error::CustomError(
        StatusCode::FORBIDDEN,
        ErrorDetail::new("forbidden".to_string(), message.to_string()),
    )
}

pub(crate) fn map_forum_error(error: ForumError) -> Error {
    match error {
        ForumError::CategoryNotFound(_)
        | ForumError::TopicNotFound(_)
        | ForumError::ReplyNotFound(_)
        | ForumError::SolutionNotFound(_) => Error::NotFound,
        ForumError::Database(_) | ForumError::Internal(_) => Error::InternalServerError,
        ForumError::Forbidden(message) => forum_forbidden(&message),
        other => Error::BadRequest(other.to_string()),
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/forum")
        .add(
            "/categories",
            get(categories::list_categories).post(categories::create_category),
        )
        .add(
            "/categories/{id}",
            get(categories::get_category)
                .put(categories::update_category)
                .delete(categories::delete_category),
        )
        .add(
            "/categories/{id}/subscription",
            axum::routing::post(categories::subscribe_category)
                .delete(categories::unsubscribe_category),
        )
        .add(
            "/topics",
            get(topics::list_topics).post(topics::create_topic),
        )
        .add(
            "/topics/{id}",
            get(topics::get_topic)
                .put(topics::update_topic)
                .delete(topics::delete_topic),
        )
        .add(
            "/topics/{topic_id}/pin",
            axum::routing::post(topics::pin_topic).delete(topics::unpin_topic),
        )
        .add(
            "/topics/{topic_id}/lock",
            axum::routing::post(topics::lock_topic).delete(topics::unlock_topic),
        )
        .add("/topics/{topic_id}/close", axum::routing::post(topics::close_topic))
        .add("/topics/{topic_id}/reopen", axum::routing::post(topics::reopen_topic))
        .add(
            "/topics/{topic_id}/archive",
            axum::routing::post(topics::archive_topic),
        )
        .add(
            "/topics/{topic_id}/solution/{reply_id}",
            axum::routing::post(topics::mark_topic_solution),
        )
        .add(
            "/topics/{topic_id}/solution",
            axum::routing::delete(topics::clear_topic_solution),
        )
        .add(
            "/topics/{topic_id}/vote/{value}",
            axum::routing::post(topics::set_topic_vote),
        )
        .add(
            "/topics/{topic_id}/vote",
            axum::routing::delete(topics::clear_topic_vote),
        )
        .add(
            "/topics/{topic_id}/subscription",
            axum::routing::post(topics::subscribe_topic).delete(topics::unsubscribe_topic),
        )
        .add(
            "/topics/{id}/replies",
            get(replies::list_replies).post(replies::create_reply),
        )
        .add(
            "/topics/{topic_id}/replies/{reply_id}/approve",
            axum::routing::post(replies::approve_reply),
        )
        .add(
            "/topics/{topic_id}/replies/{reply_id}/reject",
            axum::routing::post(replies::reject_reply),
        )
        .add(
            "/topics/{topic_id}/replies/{reply_id}/hide",
            axum::routing::post(replies::hide_reply),
        )
        .add(
            "/replies/{id}",
            get(replies::get_reply)
                .put(replies::update_reply)
                .delete(replies::delete_reply),
        )
        .add(
            "/replies/{reply_id}/vote/{value}",
            axum::routing::post(replies::set_reply_vote),
        )
        .add(
            "/replies/{reply_id}/vote",
            axum::routing::delete(replies::clear_reply_vote),
        )
        .add("/widgets/catalog", get(widgets::get_widget_catalog))
        .add(
            "/widgets/validate",
            axum::routing::post(widgets::validate_widget_props),
        )
        .add("/users/{user_id}/stats", get(users::get_user_stats))
}
