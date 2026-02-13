/// Platform Detection Middleware for RusToK
///
/// Detects client platform information from User-Agent header.
/// Provides OS, browser, and device type detection for analytics and optimization.
use axum::{
    extract::Request,
    http::header::USER_AGENT,
    middleware::Next,
    response::Response,
};
use std::fmt;

/// Detected platform information
#[derive(Clone, Debug, PartialEq)]
pub struct PlatformInfo {
    /// Operating system (e.g., "Windows", "macOS", "Linux", "iOS", "Android")
    pub os: OperatingSystem,
    /// Browser engine (e.g., "Chrome", "Firefox", "Safari", "Edge")
    pub browser: Browser,
    /// Device type (e.g., "Desktop", "Mobile", "Tablet")
    pub device_type: DeviceType,
    /// Raw User-Agent string (for logging/debugging)
    pub raw_user_agent: Option<String>,
}

impl PlatformInfo {
    /// Create unknown platform info
    pub fn unknown() -> Self {
        Self {
            os: OperatingSystem::Unknown,
            browser: Browser::Unknown,
            device_type: DeviceType::Unknown,
            raw_user_agent: None,
        }
    }

    /// Check if the client is a mobile device
    pub fn is_mobile(&self) -> bool {
        matches!(self.device_type, DeviceType::Mobile)
    }

    /// Check if the client is a desktop device
    pub fn is_desktop(&self) -> bool {
        matches!(self.device_type, DeviceType::Desktop)
    }

    /// Check if the client is a tablet device
    pub fn is_tablet(&self) -> bool {
        matches!(self.device_type, DeviceType::Tablet)
    }

    /// Parse User-Agent string and detect platform
    pub fn from_user_agent(user_agent: &str) -> Self {
        let ua = user_agent.to_lowercase();

        let os = detect_os(&ua);
        let device_type = detect_device_type(&ua, &os);
        let browser = detect_browser(&ua);

        Self {
            os,
            browser,
            device_type,
            raw_user_agent: Some(user_agent.to_string()),
        }
    }
}

impl fmt::Display for PlatformInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} / {} / {}",
            self.os, self.browser, self.device_type
        )
    }
}

impl Default for PlatformInfo {
    fn default() -> Self {
        Self::unknown()
    }
}

/// Operating system types
#[derive(Clone, Debug, PartialEq)]
pub enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
    iOS,
    Android,
    ChromeOS,
    Unknown,
}

impl fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperatingSystem::Windows => write!(f, "Windows"),
            OperatingSystem::MacOS => write!(f, "macOS"),
            OperatingSystem::Linux => write!(f, "Linux"),
            OperatingSystem::iOS => write!(f, "iOS"),
            OperatingSystem::Android => write!(f, "Android"),
            OperatingSystem::ChromeOS => write!(f, "ChromeOS"),
            OperatingSystem::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Browser types
#[derive(Clone, Debug, PartialEq)]
pub enum Browser {
    Chrome,
    Firefox,
    Safari,
    Edge,
    Opera,
    Brave,
    Unknown,
}

impl fmt::Display for Browser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Browser::Chrome => write!(f, "Chrome"),
            Browser::Firefox => write!(f, "Firefox"),
            Browser::Safari => write!(f, "Safari"),
            Browser::Edge => write!(f, "Edge"),
            Browser::Opera => write!(f, "Opera"),
            Browser::Brave => write!(f, "Brave"),
            Browser::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Device types
#[derive(Clone, Debug, PartialEq)]
pub enum DeviceType {
    Desktop,
    Mobile,
    Tablet,
    SmartTV,
    Unknown,
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceType::Desktop => write!(f, "Desktop"),
            DeviceType::Mobile => write!(f, "Mobile"),
            DeviceType::Tablet => write!(f, "Tablet"),
            DeviceType::SmartTV => write!(f, "SmartTV"),
            DeviceType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Detect operating system from User-Agent string
fn detect_os(user_agent: &str) -> OperatingSystem {
    if user_agent.contains("windows phone") || user_agent.contains("windows mobile") {
        OperatingSystem::Windows
    } else if user_agent.contains("windows nt") || user_agent.contains("windows") {
        OperatingSystem::Windows
    } else if user_agent.contains("macintosh") || user_agent.contains("mac os x") {
        OperatingSystem::MacOS
    } else if user_agent.contains("iphone") || user_agent.contains("ipad") {
        OperatingSystem::iOS
    } else if user_agent.contains("android") {
        OperatingSystem::Android
    } else if user_agent.contains("cros") || user_agent.contains("chromebook") {
        OperatingSystem::ChromeOS
    } else if user_agent.contains("linux") || user_agent.contains("x11") {
        OperatingSystem::Linux
    } else {
        OperatingSystem::Unknown
    }
}

/// Detect device type from User-Agent string and OS
fn detect_device_type(user_agent: &str, os: &OperatingSystem) -> DeviceType {
    // Check for tablets first (some tablets report as mobile)
    if user_agent.contains("ipad")
        || user_agent.contains("tablet")
        || user_agent.contains("kindle")
        || user_agent.contains("silk")
        || user_agent.contains("playbook")
    {
        return DeviceType::Tablet;
    }

    // Check for mobile devices
    if user_agent.contains("mobile")
        || user_agent.contains("iphone")
        || user_agent.contains("android")
            && !user_agent.contains("tablet")
        || user_agent.contains("windows phone")
        || user_agent.contains("windows mobile")
        || user_agent.contains("blackberry")
        || user_agent.contains("nokia")
        || user_agent.contains("opera mini")
    {
        return DeviceType::Mobile;
    }

    // Check for smart TV
    if user_agent.contains("smart-tv")
        || user_agent.contains("smarttv")
        || user_agent.contains("googletv")
        || user_agent.contains("appletv")
        || user_agent.contains("hbbtv")
        || user_agent.contains("pov_tv")
        || user_agent.contains("netcast.tv")
    {
        return DeviceType::SmartTV;
    }

    // Desktop detection based on OS
    match os {
        OperatingSystem::Windows
        | OperatingSystem::MacOS
        | OperatingSystem::Linux
        | OperatingSystem::ChromeOS => DeviceType::Desktop,
        _ => DeviceType::Unknown,
    }
}

/// Detect browser from User-Agent string
fn detect_browser(user_agent: &str) -> Browser {
    // Check for Edge first (as it also contains Chrome/Safari)
    if user_agent.contains("edg/") || user_agent.contains("edge/") {
        return Browser::Edge;
    }

    // Check for Opera
    if user_agent.contains("opr/") || user_agent.contains("opera/") || user_agent.contains("opios/")
    {
        return Browser::Opera;
    }

    // Check for Brave (contains Chrome but also has Brave-specific indicators)
    if user_agent.contains("brave/") {
        return Browser::Brave;
    }

    // Check for Chrome (but not Edge/Opera/Brave which also contain Chrome)
    if user_agent.contains("chrome/") || user_agent.contains("crios/") {
        return Browser::Chrome;
    }

    // Check for Safari (but not Chrome/Edge which also contain Safari)
    if user_agent.contains("safari/") && !user_agent.contains("chrome/") {
        return Browser::Safari;
    }

    // Check for Firefox
    if user_agent.contains("firefox/") || user_agent.contains("fxios/") {
        return Browser::Firefox;
    }

    Browser::Unknown
}

/// Middleware that detects platform from User-Agent header
///
/// This middleware extracts the User-Agent header, parses it to detect
/// the client's platform information, and stores it in request extensions
/// for use by downstream handlers.
pub async fn detect_platform(mut request: Request, next: Next) -> Response {
    let platform_info = if let Some(user_agent) = request.headers().get(USER_AGENT) {
        if let Ok(ua_str) = user_agent.to_str() {
            PlatformInfo::from_user_agent(ua_str)
        } else {
            tracing::debug!("Invalid User-Agent header encoding");
            PlatformInfo::unknown()
        }
    } else {
        tracing::debug!("No User-Agent header present");
        PlatformInfo::unknown()
    };

    tracing::debug!(
        platform = %platform_info,
        "Detected client platform"
    );

    request.extensions_mut().insert(platform_info);
    next.run(request).await
}

/// Extension trait to easily access platform info from request parts
pub trait PlatformExt {
    /// Get platform information from the request
    fn platform(&self) -> PlatformInfo;
}

impl PlatformExt for axum::extract::Request {
    fn platform(&self) -> PlatformInfo {
        self.extensions()
            .get::<PlatformInfo>()
            .cloned()
            .unwrap_or_else(PlatformInfo::unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_windows_chrome() {
        let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
        let info = PlatformInfo::from_user_agent(ua);
        assert_eq!(info.os, OperatingSystem::Windows);
        assert_eq!(info.browser, Browser::Chrome);
        assert_eq!(info.device_type, DeviceType::Desktop);
    }

    #[test]
    fn test_detect_macos_safari() {
        let ua = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15";
        let info = PlatformInfo::from_user_agent(ua);
        assert_eq!(info.os, OperatingSystem::MacOS);
        assert_eq!(info.browser, Browser::Safari);
        assert_eq!(info.device_type, DeviceType::Desktop);
    }

    #[test]
    fn test_detect_iphone_safari() {
        let ua = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1";
        let info = PlatformInfo::from_user_agent(ua);
        assert_eq!(info.os, OperatingSystem::iOS);
        assert_eq!(info.browser, Browser::Safari);
        assert_eq!(info.device_type, DeviceType::Mobile);
    }

    #[test]
    fn test_detect_ipad() {
        let ua = "Mozilla/5.0 (iPad; CPU OS 17_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1";
        let info = PlatformInfo::from_user_agent(ua);
        assert_eq!(info.os, OperatingSystem::iOS);
        assert_eq!(info.browser, Browser::Safari);
        assert_eq!(info.device_type, DeviceType::Tablet);
    }

    #[test]
    fn test_detect_android_chrome() {
        let ua = "Mozilla/5.0 (Linux; Android 14; SM-S918B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36";
        let info = PlatformInfo::from_user_agent(ua);
        assert_eq!(info.os, OperatingSystem::Android);
        assert_eq!(info.browser, Browser::Chrome);
        assert_eq!(info.device_type, DeviceType::Mobile);
    }

    #[test]
    fn test_detect_linux_firefox() {
        let ua = "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/121.0";
        let info = PlatformInfo::from_user_agent(ua);
        assert_eq!(info.os, OperatingSystem::Linux);
        assert_eq!(info.browser, Browser::Firefox);
        assert_eq!(info.device_type, DeviceType::Desktop);
    }

    #[test]
    fn test_detect_edge() {
        let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.0 Edg/120.0.0.0";
        let info = PlatformInfo::from_user_agent(ua);
        assert_eq!(info.os, OperatingSystem::Windows);
        assert_eq!(info.browser, Browser::Edge);
    }

    #[test]
    fn test_unknown_user_agent() {
        let ua = "SomeBot/1.0";
        let info = PlatformInfo::from_user_agent(ua);
        assert_eq!(info.os, OperatingSystem::Unknown);
        assert_eq!(info.browser, Browser::Unknown);
    }

    #[test]
    fn test_display_format() {
        let info = PlatformInfo {
            os: OperatingSystem::Windows,
            browser: Browser::Chrome,
            device_type: DeviceType::Desktop,
            raw_user_agent: None,
        };
        assert_eq!(info.to_string(), "Windows / Chrome / Desktop");
    }

    #[test]
    fn test_is_mobile() {
        let mobile = PlatformInfo {
            os: OperatingSystem::iOS,
            browser: Browser::Safari,
            device_type: DeviceType::Mobile,
            raw_user_agent: None,
        };
        let desktop = PlatformInfo {
            os: OperatingSystem::Windows,
            browser: Browser::Chrome,
            device_type: DeviceType::Desktop,
            raw_user_agent: None,
        };
        assert!(mobile.is_mobile());
        assert!(!desktop.is_mobile());
    }
}
