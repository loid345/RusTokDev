/// Platform Extractor for RusToK
///
/// Provides easy access to detected platform information in request handlers.
use crate::middleware::platform::PlatformInfo;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

/// Extractor that provides platform information from the request
///
/// # Example
/// ```rust
/// use axum::response::Json;
/// use serde_json::json;
///
/// async fn handler(platform: Platform) -> Json<serde_json::Value> {
///     Json(json!({
///         "os": platform.os.to_string(),
///         "browser": platform.browser.to_string(),
///         "device": platform.device_type.to_string(),
///         "is_mobile": platform.is_mobile(),
///     }))
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Platform(pub PlatformInfo);

impl Platform {
    /// Get the operating system
    pub fn os(&self) -> &crate::middleware::platform::OperatingSystem {
        &self.0.os
    }

    /// Get the browser
    pub fn browser(&self) -> &crate::middleware::platform::Browser {
        &self.0.browser
    }

    /// Get the device type
    pub fn device_type(&self) -> &crate::middleware::platform::DeviceType {
        &self.0.device_type
    }

    /// Check if the client is a mobile device
    pub fn is_mobile(&self) -> bool {
        self.0.is_mobile()
    }

    /// Check if the client is a desktop device
    pub fn is_desktop(&self) -> bool {
        self.0.is_desktop()
    }

    /// Check if the client is a tablet device
    pub fn is_tablet(&self) -> bool {
        self.0.is_tablet()
    }

    /// Get the raw User-Agent string if available
    pub fn raw_user_agent(&self) -> Option<&str> {
        self.0.raw_user_agent.as_deref()
    }
}

impl std::ops::Deref for Platform {
    type Target = PlatformInfo;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<S> FromRequestParts<S> for Platform
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<PlatformInfo>()
            .cloned()
            .map(Platform)
            .ok_or((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Platform information not available - ensure platform detection middleware is configured",
            ))
    }
}

/// Optional platform extractor that doesn't fail if platform info is not available
#[derive(Clone, Debug)]
pub struct OptionalPlatform(pub Option<PlatformInfo>);

impl OptionalPlatform {
    /// Returns true if platform information is available
    pub fn is_known(&self) -> bool {
        self.0.is_some()
    }

    /// Get the platform info if available
    pub fn platform(&self) -> Option<&PlatformInfo> {
        self.0.as_ref()
    }
}

impl<S> FromRequestParts<S> for OptionalPlatform
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(OptionalPlatform(parts.extensions.get::<PlatformInfo>().cloned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::platform::{Browser, DeviceType, OperatingSystem};

    fn create_test_platform_info() -> PlatformInfo {
        PlatformInfo {
            os: OperatingSystem::Windows,
            browser: Browser::Chrome,
            device_type: DeviceType::Desktop,
            raw_user_agent: Some("Test UA".to_string()),
        }
    }

    #[test]
    fn test_platform_deref() {
        let info = create_test_platform_info();
        let platform = Platform(info.clone());

        assert_eq!(platform.os, OperatingSystem::Windows);
        assert_eq!(platform.browser, Browser::Chrome);
        assert_eq!(platform.device_type, DeviceType::Desktop);
    }

    #[test]
    fn test_platform_methods() {
        let info = create_test_platform_info();
        let platform = Platform(info);

        assert!(platform.is_desktop());
        assert!(!platform.is_mobile());
        assert!(!platform.is_tablet());
        assert_eq!(platform.raw_user_agent(), Some("Test UA"));
    }

    #[test]
    fn test_platform_display() {
        let info = create_test_platform_info();
        let platform = Platform(info);

        assert_eq!(platform.to_string(), "Windows / Chrome / Desktop");
    }

    #[test]
    fn test_optional_platform_known() {
        let info = create_test_platform_info();
        let optional = OptionalPlatform(Some(info));

        assert!(optional.is_known());
        assert!(optional.platform().is_some());

        let optional_none = OptionalPlatform(None);
        assert!(!optional_none.is_known());
        assert!(optional_none.platform().is_none());
    }
}
