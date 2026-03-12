use rustok_iggy::IggyConfig;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const DEFAULT_TENANT_ID: Uuid = Uuid::from_u128(1);

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RustokSettings {
    #[serde(default)]
    pub tenant: TenantSettings,
    #[serde(default)]
    pub search: SearchSettings,
    #[serde(default)]
    pub features: FeatureSettings,
    #[serde(default)]
    pub rate_limit: RateLimitSettings,
    #[serde(default)]
    pub events: EventSettings,
    #[serde(default)]
    pub email: EmailSettings,
    #[serde(default)]
    pub runtime: RuntimeSettings,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmailSettings {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub smtp: SmtpSettings,
    #[serde(default = "default_email_from")]
    pub from: String,
    #[serde(default = "default_reset_base_url")]
    pub reset_base_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SmtpSettings {
    #[serde(default = "default_smtp_host")]
    pub host: String,
    #[serde(default = "default_smtp_port")]
    pub port: u16,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
}

impl Default for SmtpSettings {
    fn default() -> Self {
        Self {
            host: default_smtp_host(),
            port: default_smtp_port(),
            username: String::new(),
            password: String::new(),
        }
    }
}

impl Default for EmailSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            smtp: SmtpSettings::default(),
            from: default_email_from(),
            reset_base_url: default_reset_base_url(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventSettings {
    #[serde(default)]
    pub transport: EventTransportKind,
    #[serde(default)]
    pub relay_target: RelayTargetKind,
    #[serde(default)]
    pub allow_relay_target_fallback: bool,
    #[serde(default = "default_relay_interval_ms")]
    pub relay_interval_ms: u64,
    #[serde(default = "default_event_channel_capacity")]
    pub channel_capacity: usize,
    #[serde(default)]
    pub relay_retry_policy: RelayRetryPolicy,
    #[serde(default)]
    pub dlq: DlqSettings,
    #[serde(default)]
    pub backpressure: EventBackpressureSettings,
    #[serde(default)]
    pub iggy: IggyConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventBackpressureSettings {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_backpressure_max_queue_depth")]
    pub max_queue_depth: usize,
    #[serde(default = "default_backpressure_warning_threshold")]
    pub warning_threshold: f64,
    #[serde(default = "default_backpressure_critical_threshold")]
    pub critical_threshold: f64,
}

impl Default for EventBackpressureSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            max_queue_depth: default_backpressure_max_queue_depth(),
            warning_threshold: default_backpressure_warning_threshold(),
            critical_threshold: default_backpressure_critical_threshold(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RelayRetryPolicy {
    #[serde(default = "default_relay_max_attempts")]
    pub max_attempts: i32,
    #[serde(default = "default_relay_backoff_base_ms")]
    pub base_backoff_ms: u64,
    #[serde(default = "default_relay_backoff_max_ms")]
    pub max_backoff_ms: u64,
}

impl Default for RelayRetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: default_relay_max_attempts(),
            base_backoff_ms: default_relay_backoff_base_ms(),
            max_backoff_ms: default_relay_backoff_max_ms(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DlqSettings {
    #[serde(default = "default_dlq_enabled")]
    pub enabled: bool,
    #[serde(default = "default_dlq_max_attempts")]
    pub max_attempts: i32,
}

impl Default for DlqSettings {
    fn default() -> Self {
        Self {
            enabled: default_dlq_enabled(),
            max_attempts: default_dlq_max_attempts(),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RelayTargetKind {
    #[default]
    Memory,
    Iggy,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EventTransportKind {
    #[default]
    Memory,
    Outbox,
    Iggy,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TenantSettings {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_resolution")]
    pub resolution: String,
    #[serde(default = "default_header_name")]
    pub header_name: String,
    #[serde(default = "default_tenant_id")]
    pub default_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeatureSettings {
    #[serde(default = "default_true")]
    pub registration_enabled: bool,
    #[serde(default)]
    pub email_verification: bool,
    #[serde(default = "default_true")]
    pub multi_tenant: bool,
    #[serde(default = "default_true")]
    pub search_indexing: bool,
    #[serde(default)]
    pub oauth_enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchSettings {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_search_driver")]
    pub driver: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default = "default_index_prefix")]
    pub index_prefix: String,
    #[serde(default)]
    pub reindex: SearchReindexSettings,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchReindexSettings {
    #[serde(default = "default_search_reindex_parallelism")]
    pub parallelism: usize,
    #[serde(default = "default_search_reindex_entity_budget")]
    pub entity_budget: usize,
    #[serde(default = "default_search_reindex_yield_every")]
    pub yield_every: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RateLimitSettings {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub backend: RateLimitBackendKind,
    #[serde(default = "default_rate_limit_redis_key_prefix")]
    pub redis_key_prefix: String,
    #[serde(default = "default_requests_per_minute")]
    pub requests_per_minute: u32,
    #[serde(default = "default_burst")]
    pub burst: u32,
    #[serde(default = "default_auth_requests_per_minute")]
    pub auth_requests_per_minute: u32,
    #[serde(default = "default_auth_burst")]
    pub auth_burst: u32,
    #[serde(default = "default_oauth_requests_per_minute")]
    pub oauth_requests_per_minute: u32,
    #[serde(default = "default_oauth_burst")]
    pub oauth_burst: u32,
    #[serde(default = "default_trusted_auth_dimensions")]
    pub trusted_auth_dimensions: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RuntimeSettings {
    #[serde(default)]
    pub guardrails: RuntimeGuardrailSettings,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RuntimeGuardrailSettings {
    #[serde(default)]
    pub rollout: GuardrailRolloutMode,
    #[serde(default)]
    pub rate_limit_memory_thresholds: RateLimitMemoryGuardrailSettings,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RateLimitMemoryGuardrailSettings {
    #[serde(default = "default_runtime_guardrail_api_warning_entries")]
    pub api_warning_entries: usize,
    #[serde(default = "default_runtime_guardrail_api_critical_entries")]
    pub api_critical_entries: usize,
    #[serde(default = "default_runtime_guardrail_auth_warning_entries")]
    pub auth_warning_entries: usize,
    #[serde(default = "default_runtime_guardrail_auth_critical_entries")]
    pub auth_critical_entries: usize,
    #[serde(default = "default_runtime_guardrail_oauth_warning_entries")]
    pub oauth_warning_entries: usize,
    #[serde(default = "default_runtime_guardrail_oauth_critical_entries")]
    pub oauth_critical_entries: usize,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RateLimitBackendKind {
    #[default]
    Memory,
    Redis,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GuardrailRolloutMode {
    Observe,
    #[default]
    Enforce,
}

impl Default for TenantSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            resolution: default_resolution(),
            header_name: default_header_name(),
            default_id: default_tenant_id(),
        }
    }
}

impl Default for FeatureSettings {
    fn default() -> Self {
        Self {
            registration_enabled: true,
            email_verification: false,
            multi_tenant: true,
            search_indexing: true,
            oauth_enabled: false,
        }
    }
}

impl Default for SearchSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            driver: default_search_driver(),
            url: String::new(),
            api_key: None,
            index_prefix: default_index_prefix(),
            reindex: SearchReindexSettings::default(),
        }
    }
}

impl Default for EventSettings {
    fn default() -> Self {
        Self {
            transport: EventTransportKind::default(),
            relay_target: RelayTargetKind::default(),
            allow_relay_target_fallback: false,
            relay_interval_ms: default_relay_interval_ms(),
            channel_capacity: default_event_channel_capacity(),
            relay_retry_policy: RelayRetryPolicy::default(),
            dlq: DlqSettings::default(),
            backpressure: EventBackpressureSettings::default(),
            iggy: IggyConfig::default(),
        }
    }
}

impl Default for SearchReindexSettings {
    fn default() -> Self {
        Self {
            parallelism: default_search_reindex_parallelism(),
            entity_budget: default_search_reindex_entity_budget(),
            yield_every: default_search_reindex_yield_every(),
        }
    }
}

impl Default for RateLimitSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            backend: RateLimitBackendKind::Memory,
            redis_key_prefix: default_rate_limit_redis_key_prefix(),
            requests_per_minute: default_requests_per_minute(),
            burst: default_burst(),
            auth_requests_per_minute: default_auth_requests_per_minute(),
            auth_burst: default_auth_burst(),
            oauth_requests_per_minute: default_oauth_requests_per_minute(),
            oauth_burst: default_oauth_burst(),
            trusted_auth_dimensions: default_trusted_auth_dimensions(),
        }
    }
}

impl Default for RuntimeSettings {
    fn default() -> Self {
        Self {
            guardrails: RuntimeGuardrailSettings::default(),
        }
    }
}

impl Default for RuntimeGuardrailSettings {
    fn default() -> Self {
        Self {
            rollout: GuardrailRolloutMode::Enforce,
            rate_limit_memory_thresholds: RateLimitMemoryGuardrailSettings::default(),
        }
    }
}

impl Default for RateLimitMemoryGuardrailSettings {
    fn default() -> Self {
        Self {
            api_warning_entries: default_runtime_guardrail_api_warning_entries(),
            api_critical_entries: default_runtime_guardrail_api_critical_entries(),
            auth_warning_entries: default_runtime_guardrail_auth_warning_entries(),
            auth_critical_entries: default_runtime_guardrail_auth_critical_entries(),
            oauth_warning_entries: default_runtime_guardrail_oauth_warning_entries(),
            oauth_critical_entries: default_runtime_guardrail_oauth_critical_entries(),
        }
    }
}

impl RustokSettings {
    pub fn from_settings(settings: &Option<serde_json::Value>) -> Result<Self, serde_json::Error> {
        let root = settings.clone().unwrap_or_else(|| serde_json::json!({}));
        let rustok = root
            .get("rustok")
            .cloned()
            .unwrap_or_else(|| serde_json::json!({}));
        let mut parsed: Self = serde_json::from_value(rustok)?;

        if let Ok(raw_transport) = std::env::var("RUSTOK_EVENT_TRANSPORT") {
            parsed.events.transport = parse_event_transport(&raw_transport)?;
        }

        if parsed.events.relay_retry_policy.max_attempts <= 0 {
            return Err(serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "rustok.events.relay_retry_policy.max_attempts must be > 0",
            )));
        }

        if parsed.events.dlq.max_attempts <= 0 {
            return Err(serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "rustok.events.dlq.max_attempts must be > 0",
            )));
        }

        if parsed.events.channel_capacity == 0 {
            return Err(serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "rustok.events.channel_capacity must be > 0",
            )));
        }

        let backpressure = &parsed.events.backpressure;
        if backpressure.enabled {
            if backpressure.max_queue_depth == 0 {
                return Err(serde_json::Error::io(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "rustok.events.backpressure.max_queue_depth must be > 0",
                )));
            }

            if !(0.0..1.0).contains(&backpressure.warning_threshold) {
                return Err(serde_json::Error::io(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "rustok.events.backpressure.warning_threshold must be in range (0, 1)",
                )));
            }

            if !(backpressure.warning_threshold..=1.0).contains(&backpressure.critical_threshold)
                || backpressure.critical_threshold <= backpressure.warning_threshold
            {
                return Err(serde_json::Error::io(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "rustok.events.backpressure.critical_threshold must be in range (warning_threshold, 1]",
                )));
            }
        }

        if parsed.search.reindex.parallelism == 0 {
            return Err(serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "rustok.search.reindex.parallelism must be > 0",
            )));
        }

        if parsed.search.reindex.entity_budget == 0 {
            return Err(serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "rustok.search.reindex.entity_budget must be > 0",
            )));
        }

        if parsed.search.reindex.yield_every == 0 {
            return Err(serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "rustok.search.reindex.yield_every must be > 0",
            )));
        }

        validate_guardrail_threshold(
            "rustok.runtime.guardrails.rate_limit_memory_thresholds.api",
            parsed
                .runtime
                .guardrails
                .rate_limit_memory_thresholds
                .api_warning_entries,
            parsed
                .runtime
                .guardrails
                .rate_limit_memory_thresholds
                .api_critical_entries,
        )?;
        validate_guardrail_threshold(
            "rustok.runtime.guardrails.rate_limit_memory_thresholds.auth",
            parsed
                .runtime
                .guardrails
                .rate_limit_memory_thresholds
                .auth_warning_entries,
            parsed
                .runtime
                .guardrails
                .rate_limit_memory_thresholds
                .auth_critical_entries,
        )?;
        validate_guardrail_threshold(
            "rustok.runtime.guardrails.rate_limit_memory_thresholds.oauth",
            parsed
                .runtime
                .guardrails
                .rate_limit_memory_thresholds
                .oauth_warning_entries,
            parsed
                .runtime
                .guardrails
                .rate_limit_memory_thresholds
                .oauth_critical_entries,
        )?;

        Ok(parsed)
    }
}

fn parse_event_transport(value: &str) -> Result<EventTransportKind, serde_json::Error> {
    match value.trim().to_ascii_lowercase().as_str() {
        "memory" => Ok(EventTransportKind::Memory),
        "outbox" => Ok(EventTransportKind::Outbox),
        "iggy" => Ok(EventTransportKind::Iggy),
        _ => Err(serde_json::Error::io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "Invalid RUSTOK_EVENT_TRANSPORT='{value}'. Expected one of: memory, outbox, iggy"
            ),
        ))),
    }
}

fn default_tenant_id() -> Uuid {
    DEFAULT_TENANT_ID
}

fn default_resolution() -> String {
    "header".to_string()
}

fn default_header_name() -> String {
    "X-Tenant-ID".to_string()
}

fn default_true() -> bool {
    true
}

fn default_search_driver() -> String {
    "meilisearch".to_string()
}

fn default_index_prefix() -> String {
    "rustok_".to_string()
}

fn default_search_reindex_parallelism() -> usize {
    4
}

fn default_search_reindex_entity_budget() -> usize {
    500
}

fn default_search_reindex_yield_every() -> u64 {
    50
}

fn default_requests_per_minute() -> u32 {
    60
}

fn default_rate_limit_redis_key_prefix() -> String {
    "rate-limit:v1".to_string()
}

fn default_burst() -> u32 {
    10
}

fn default_auth_requests_per_minute() -> u32 {
    20
}

fn default_auth_burst() -> u32 {
    0
}

fn default_oauth_requests_per_minute() -> u32 {
    30
}

fn default_oauth_burst() -> u32 {
    5
}

fn default_trusted_auth_dimensions() -> bool {
    true
}

fn default_relay_interval_ms() -> u64 {
    1_000
}

fn default_event_channel_capacity() -> usize {
    128
}

fn default_relay_max_attempts() -> i32 {
    5
}

fn default_relay_backoff_base_ms() -> u64 {
    1_000
}

fn default_relay_backoff_max_ms() -> u64 {
    60_000
}

fn default_dlq_enabled() -> bool {
    true
}

fn default_dlq_max_attempts() -> i32 {
    10
}

fn default_backpressure_max_queue_depth() -> usize {
    10_000
}

fn default_backpressure_warning_threshold() -> f64 {
    0.7
}

fn default_backpressure_critical_threshold() -> f64 {
    0.9
}

fn default_email_from() -> String {
    "no-reply@rustok.local".to_string()
}

fn default_runtime_guardrail_api_warning_entries() -> usize {
    5_000
}

fn default_runtime_guardrail_api_critical_entries() -> usize {
    20_000
}

fn default_runtime_guardrail_auth_warning_entries() -> usize {
    1_000
}

fn default_runtime_guardrail_auth_critical_entries() -> usize {
    5_000
}

fn default_runtime_guardrail_oauth_warning_entries() -> usize {
    1_000
}

fn default_runtime_guardrail_oauth_critical_entries() -> usize {
    5_000
}

fn default_reset_base_url() -> String {
    "http://localhost:3000/reset-password".to_string()
}

fn validate_guardrail_threshold(
    namespace: &str,
    warning_entries: usize,
    critical_entries: usize,
) -> Result<(), serde_json::Error> {
    if warning_entries == 0 {
        return Err(serde_json::Error::io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("{namespace}.warning_entries must be > 0"),
        )));
    }

    if critical_entries <= warning_entries {
        return Err(serde_json::Error::io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("{namespace}.critical_entries must be > warning_entries"),
        )));
    }

    Ok(())
}

fn default_smtp_host() -> String {
    "localhost".to_string()
}

fn default_smtp_port() -> u16 {
    1025
}

#[cfg(test)]
mod tests {
    use super::{
        EventTransportKind, GuardrailRolloutMode, RateLimitBackendKind, RelayTargetKind,
        RustokSettings,
    };
    use std::sync::{Mutex, OnceLock};

    const EVENT_TRANSPORT_ENV: &str = "RUSTOK_EVENT_TRANSPORT";

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    struct EnvVarGuard {
        key: &'static str,
        original: Option<String>,
    }

    impl EnvVarGuard {
        fn clear(key: &'static str) -> Self {
            let original = std::env::var(key).ok();
            unsafe {
                std::env::remove_var(key);
            }
            Self { key, original }
        }

        fn set(key: &'static str, value: &str) -> Self {
            let original = std::env::var(key).ok();
            unsafe {
                std::env::set_var(key, value);
            }
            Self { key, original }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            unsafe {
                match &self.original {
                    Some(value) => std::env::set_var(self.key, value),
                    None => std::env::remove_var(self.key),
                }
            }
        }
    }

    #[test]
    fn reads_transport_from_config() {
        let _guard = env_lock().lock().expect("env lock poisoned");
        let _env_guard = EnvVarGuard::clear(EVENT_TRANSPORT_ENV);

        let raw = serde_json::json!({
            "rustok": {
                "events": {
                    "transport": "outbox"
                }
            }
        });

        let settings = RustokSettings::from_settings(&Some(raw)).expect("settings parsed");
        assert_eq!(settings.events.transport, EventTransportKind::Outbox);
    }

    #[test]
    fn rejects_invalid_env_transport() {
        let _guard = env_lock().lock().expect("env lock poisoned");
        let _env_guard = EnvVarGuard::set(EVENT_TRANSPORT_ENV, "broken");

        let err = RustokSettings::from_settings(&Some(serde_json::json!({ "rustok": {} })))
            .expect_err("transport should fail");
        assert!(err
            .to_string()
            .contains("Invalid RUSTOK_EVENT_TRANSPORT='broken'"));
    }

    #[test]
    fn reads_relay_defaults_from_config() {
        let _guard = env_lock().lock().expect("env lock poisoned");
        let _env_guard = EnvVarGuard::clear(EVENT_TRANSPORT_ENV);

        let raw = serde_json::json!({
            "rustok": {
                "events": {
                    "transport": "outbox",
                    "relay_target": "iggy"
                }
            }
        });

        let settings = RustokSettings::from_settings(&Some(raw)).expect("settings parsed");
        assert_eq!(settings.events.transport, EventTransportKind::Outbox);
        assert_eq!(settings.events.relay_target, RelayTargetKind::Iggy);
        assert!(!settings.events.allow_relay_target_fallback);
        assert_eq!(settings.events.channel_capacity, 128);
        assert_eq!(settings.events.relay_retry_policy.max_attempts, 5);
        assert_eq!(settings.events.relay_retry_policy.base_backoff_ms, 1_000);
        assert_eq!(settings.events.relay_retry_policy.max_backoff_ms, 60_000);
        assert!(settings.events.dlq.enabled);
        assert_eq!(settings.events.dlq.max_attempts, 10);
    }

    #[test]
    fn rejects_non_positive_retry_and_dlq_attempts() {
        let _guard = env_lock().lock().expect("env lock poisoned");
        let _env_guard = EnvVarGuard::clear(EVENT_TRANSPORT_ENV);

        let bad_retry = serde_json::json!({
            "rustok": {
                "events": {
                    "relay_retry_policy": { "max_attempts": 0 }
                }
            }
        });

        let err =
            RustokSettings::from_settings(&Some(bad_retry)).expect_err("retry validation expected");
        assert!(err
            .to_string()
            .contains("relay_retry_policy.max_attempts must be > 0"));

        let bad_dlq = serde_json::json!({
            "rustok": {
                "events": {
                    "dlq": { "max_attempts": 0 }
                }
            }
        });

        let err =
            RustokSettings::from_settings(&Some(bad_dlq)).expect_err("dlq validation expected");
        assert!(err.to_string().contains("dlq.max_attempts must be > 0"));
    }

    #[test]
    fn rejects_zero_event_channel_capacity() {
        let _guard = env_lock().lock().expect("env lock poisoned");
        let _env_guard = EnvVarGuard::clear(EVENT_TRANSPORT_ENV);

        let raw = serde_json::json!({
            "rustok": {
                "events": {
                    "channel_capacity": 0
                }
            }
        });

        let err = RustokSettings::from_settings(&Some(raw)).expect_err("capacity validation");
        assert!(err
            .to_string()
            .contains("rustok.events.channel_capacity must be > 0"));
    }

    #[test]
    fn reads_rate_limit_backend_defaults() {
        let _guard = env_lock().lock().expect("env lock poisoned");
        let _env_guard = EnvVarGuard::clear(EVENT_TRANSPORT_ENV);

        let settings =
            RustokSettings::from_settings(&Some(serde_json::json!({ "rustok": {} }))).unwrap();

        assert_eq!(settings.rate_limit.backend, RateLimitBackendKind::Memory);
        assert_eq!(settings.rate_limit.redis_key_prefix, "rate-limit:v1");
        assert_eq!(settings.rate_limit.oauth_requests_per_minute, 30);
        assert_eq!(settings.rate_limit.oauth_burst, 5);
        assert!(settings.rate_limit.trusted_auth_dimensions);
        assert_eq!(settings.events.channel_capacity, 128);
        assert_eq!(settings.events.relay_interval_ms, 1_000);
        assert_eq!(settings.email.from, "no-reply@rustok.local");
        assert_eq!(
            settings.email.reset_base_url,
            "http://localhost:3000/reset-password"
        );
        assert_eq!(
            settings.runtime.guardrails.rollout,
            GuardrailRolloutMode::Enforce
        );
        assert_eq!(
            settings
                .runtime
                .guardrails
                .rate_limit_memory_thresholds
                .api_warning_entries,
            5_000
        );
        assert_eq!(
            settings
                .runtime
                .guardrails
                .rate_limit_memory_thresholds
                .api_critical_entries,
            20_000
        );
        assert_eq!(
            settings
                .runtime
                .guardrails
                .rate_limit_memory_thresholds
                .auth_warning_entries,
            1_000
        );
        assert_eq!(
            settings
                .runtime
                .guardrails
                .rate_limit_memory_thresholds
                .auth_critical_entries,
            5_000
        );
        assert_eq!(
            settings
                .runtime
                .guardrails
                .rate_limit_memory_thresholds
                .oauth_warning_entries,
            1_000
        );
        assert_eq!(
            settings
                .runtime
                .guardrails
                .rate_limit_memory_thresholds
                .oauth_critical_entries,
            5_000
        );
        assert_eq!(settings.search.reindex.parallelism, 4);
        assert_eq!(settings.search.reindex.entity_budget, 500);
        assert_eq!(settings.search.reindex.yield_every, 50);
    }

    #[test]
    fn rejects_zero_search_reindex_budget_values() {
        let _guard = env_lock().lock().expect("env lock poisoned");
        let _env_guard = EnvVarGuard::clear(EVENT_TRANSPORT_ENV);

        let raw = serde_json::json!({
            "rustok": {
                "search": {
                    "reindex": {
                        "parallelism": 0
                    }
                }
            }
        });

        let err = RustokSettings::from_settings(&Some(raw)).expect_err("search reindex validation");
        assert!(err
            .to_string()
            .contains("rustok.search.reindex.parallelism must be > 0"));
    }

    #[test]
    fn rejects_invalid_runtime_guardrail_thresholds() {
        let _guard = env_lock().lock().expect("env lock poisoned");
        let _env_guard = EnvVarGuard::clear(EVENT_TRANSPORT_ENV);

        let raw = serde_json::json!({
            "rustok": {
                "runtime": {
                    "guardrails": {
                        "rate_limit_memory_thresholds": {
                            "auth_warning_entries": 100,
                            "auth_critical_entries": 100
                        }
                    }
                }
            }
        });

        let err =
            RustokSettings::from_settings(&Some(raw)).expect_err("guardrail validation expected");
        assert!(err.to_string().contains(
            "rustok.runtime.guardrails.rate_limit_memory_thresholds.auth.critical_entries must be > warning_entries"
        ));
    }
}
