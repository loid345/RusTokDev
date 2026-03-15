use crate::error::RbacError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthzEngine {
    Relation,
    Casbin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RbacAuthzMode {
    RelationOnly,
    CasbinShadow,
    CasbinOnly,
}

const AUTHZ_MODE_ENV: &str = "RUSTOK_RBAC_AUTHZ_MODE";

impl RbacAuthzMode {
    pub fn try_parse(value: &str) -> Result<Self, RbacError> {
        match value.trim().to_ascii_lowercase().as_str() {
            "relation_only" | "relation-only" | "relation" => Ok(Self::RelationOnly),
            "casbin_shadow" | "casbin-shadow" | "casbin_shadow_read" => Ok(Self::CasbinShadow),
            "casbin_only" | "casbin-only" | "casbin" => Ok(Self::CasbinOnly),
            _ => Err(RbacError::InvalidAuthzMode {
                value: value.to_string(),
            }),
        }
    }

    pub fn parse(value: &str) -> Self {
        Self::try_parse(value).unwrap_or(Self::RelationOnly)
    }

    pub fn from_env() -> Self {
        if let Ok(raw_mode) = std::env::var(AUTHZ_MODE_ENV) {
            return Self::parse(&raw_mode);
        }

        Self::RelationOnly
    }

    pub fn active_engine(self) -> AuthzEngine {
        match self {
            Self::CasbinOnly => AuthzEngine::Casbin,
            Self::RelationOnly | Self::CasbinShadow => AuthzEngine::Relation,
        }
    }

    pub fn is_casbin_shadow(self) -> bool {
        self == Self::CasbinShadow
    }

    pub fn is_casbin_only(self) -> bool {
        self == Self::CasbinOnly
    }

    pub fn should_run_casbin_shadow(self) -> bool {
        self == Self::CasbinShadow
    }
}

#[cfg(test)]
mod tests {
    use super::{AuthzEngine, RbacAuthzMode, AUTHZ_MODE_ENV};
    use crate::error::RbacError;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
            .lock()
            .expect("env lock")
    }

    struct EnvVarGuard {
        name: &'static str,
        previous: Option<String>,
    }

    impl EnvVarGuard {
        fn capture(name: &'static str) -> Self {
            let previous = std::env::var(name).ok();
            Self { name, previous }
        }

        fn set(&self, value: &str) {
            unsafe { std::env::set_var(self.name, value) };
        }

        fn remove(&self) {
            unsafe { std::env::remove_var(self.name) };
        }

        fn restore(&self) {
            if let Some(previous) = self.previous.as_ref() {
                unsafe { std::env::set_var(self.name, previous) };
            } else {
                unsafe { std::env::remove_var(self.name) };
            }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            self.restore();
        }
    }

    #[test]
    fn parse_defaults_to_relation_only() {
        assert_eq!(RbacAuthzMode::parse("legacy"), RbacAuthzMode::RelationOnly);
    }

    #[test]
    fn try_parse_returns_validation_error_for_unknown_mode() {
        assert_eq!(
            RbacAuthzMode::try_parse("legacy"),
            Err(RbacError::InvalidAuthzMode {
                value: "legacy".to_string(),
            }),
        );
    }

    #[test]
    fn parse_supports_relation_only_aliases() {
        assert_eq!(
            RbacAuthzMode::parse("relation_only"),
            RbacAuthzMode::RelationOnly
        );
        assert_eq!(
            RbacAuthzMode::parse("relation-only"),
            RbacAuthzMode::RelationOnly
        );
        assert_eq!(
            RbacAuthzMode::parse("relation"),
            RbacAuthzMode::RelationOnly
        );
    }

    #[test]
    fn parse_supports_casbin_modes() {
        assert_eq!(
            RbacAuthzMode::parse("casbin-shadow"),
            RbacAuthzMode::CasbinShadow
        );
        assert_eq!(RbacAuthzMode::parse("casbin"), RbacAuthzMode::CasbinOnly);
    }

    #[test]
    fn from_env_defaults_to_relation_only_when_env_missing() {
        let _lock = env_lock();
        let mode_env = EnvVarGuard::capture(AUTHZ_MODE_ENV);
        mode_env.remove();

        assert_eq!(RbacAuthzMode::from_env(), RbacAuthzMode::RelationOnly);
    }

    #[test]
    fn from_env_reads_canonical_casbin_shadow_mode() {
        let _lock = env_lock();
        let mode_env = EnvVarGuard::capture(AUTHZ_MODE_ENV);
        mode_env.set("casbin_shadow");

        assert_eq!(RbacAuthzMode::from_env(), RbacAuthzMode::CasbinShadow);
    }

    #[test]
    fn from_env_reads_canonical_casbin_only_mode() {
        let _lock = env_lock();
        let mode_env = EnvVarGuard::capture(AUTHZ_MODE_ENV);
        mode_env.set("casbin_only");

        assert_eq!(RbacAuthzMode::from_env(), RbacAuthzMode::CasbinOnly);
    }

    #[test]
    fn from_env_falls_back_to_relation_only_for_unknown_mode() {
        let _lock = env_lock();
        let mode_env = EnvVarGuard::capture(AUTHZ_MODE_ENV);
        mode_env.set("legacy");

        assert_eq!(RbacAuthzMode::from_env(), RbacAuthzMode::RelationOnly);
    }

    #[test]
    fn exposes_active_engine_and_shadow_controls() {
        assert_eq!(
            RbacAuthzMode::RelationOnly.active_engine(),
            AuthzEngine::Relation
        );
        assert_eq!(
            RbacAuthzMode::CasbinShadow.active_engine(),
            AuthzEngine::Relation
        );
        assert_eq!(
            RbacAuthzMode::CasbinOnly.active_engine(),
            AuthzEngine::Casbin
        );
        assert!(RbacAuthzMode::CasbinShadow.should_run_casbin_shadow());
        assert!(!RbacAuthzMode::CasbinOnly.should_run_casbin_shadow());
    }
}
