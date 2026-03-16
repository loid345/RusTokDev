use async_graphql::{InputObject, SimpleObject};

/// A single platform settings category and its JSON payload.
#[derive(Debug, Clone, SimpleObject)]
pub struct PlatformSettingsPayload {
    pub category: String,
    /// Settings serialised as a JSON string so clients can parse it dynamically.
    pub settings: String,
}

/// Input for updating a single category.
#[derive(Debug, Clone, InputObject)]
pub struct UpdatePlatformSettingsInput {
    pub category: String,
    /// Full replacement JSON string for the category settings.
    pub settings: String,
}

#[derive(Debug, Clone, SimpleObject)]
pub struct UpdatePlatformSettingsPayload {
    pub success: bool,
    pub category: String,
    pub settings: String,
}
