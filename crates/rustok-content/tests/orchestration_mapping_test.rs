use rustok_content::entities::node::ContentStatus;
use rustok_content::services::orchestration_mapping::{
    map_post_metadata_to_topic_metadata, map_post_status_to_topic_status,
    map_topic_metadata_to_post_metadata, map_topic_status_to_post_status,
};

#[test]
fn topic_status_mapping_respects_archived_forum_status() {
    let metadata = serde_json::json!({"forum_status": "archived"});
    let mapped = map_topic_status_to_post_status(&ContentStatus::Published, &metadata);
    assert_eq!(mapped, Some(ContentStatus::Archived));
}

#[test]
fn post_status_mapping_respects_commenting_disabled_flag() {
    let metadata = serde_json::json!({"commenting_enabled": false});
    let mapped = map_post_status_to_topic_status(&ContentStatus::Published, &metadata);
    assert_eq!(mapped, Some(ContentStatus::Archived));
}

#[test]
fn topic_metadata_is_mapped_to_post_shape() {
    let metadata = serde_json::json!({"forum_status": "open", "tags": ["rust"]});
    let mapped = map_topic_metadata_to_post_metadata(metadata);

    assert_eq!(mapped["discussion_status"], "open");
    assert_eq!(mapped["origin_kind"], "forum_topic");
    assert_eq!(mapped["tags"], serde_json::json!(["rust"]));
}

#[test]
fn post_metadata_is_mapped_to_topic_shape() {
    let metadata = serde_json::json!({"discussion_status": "closed"});
    let mapped = map_post_metadata_to_topic_metadata(metadata);

    assert_eq!(mapped["forum_status"], "closed");
    assert_eq!(mapped["origin_kind"], "blog_post");
}
