use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Transport-agnostic context that must cross every Fluid Backend Architecture port.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FbaContext {
    pub tenant_id: String,
    pub actor: FbaActor,
    pub claims: Vec<String>,
    pub roles: Vec<String>,
    pub channel: Option<String>,
    pub locale: String,
    pub correlation_id: String,
    pub causation_id: Option<String>,
    pub traceparent: Option<String>,
    pub idempotency_key: Option<String>,
    pub deadline_ms: Option<u64>,
}

impl FbaContext {
    pub fn new(
        tenant_id: impl Into<String>,
        actor: FbaActor,
        locale: impl Into<String>,
        correlation_id: impl Into<String>,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            actor,
            claims: Vec::new(),
            roles: Vec::new(),
            channel: None,
            locale: locale.into(),
            correlation_id: correlation_id.into(),
            causation_id: None,
            traceparent: None,
            idempotency_key: None,
            deadline_ms: None,
        }
    }

    pub fn with_idempotency_key(mut self, key: impl Into<String>) -> Self {
        self.idempotency_key = Some(key.into());
        self
    }

    pub fn with_deadline(mut self, deadline: Duration) -> Self {
        self.deadline_ms = Some(deadline.as_millis().min(u128::from(u64::MAX)) as u64);
        self
    }

    pub fn require_write_semantics(&self) -> Result<(), FbaError> {
        if self
            .idempotency_key
            .as_deref()
            .unwrap_or_default()
            .is_empty()
        {
            return Err(FbaError::validation(
                "fba.idempotency_key_required",
                "write port calls require a non-empty idempotency key",
            ));
        }
        if self.deadline_ms.unwrap_or_default() == 0 {
            return Err(FbaError::timeout(
                "fba.deadline_required",
                "write port calls require deadline semantics",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FbaActor {
    pub kind: FbaActorKind,
    pub id: String,
}

impl FbaActor {
    pub fn user(id: impl Into<String>) -> Self {
        Self {
            kind: FbaActorKind::User,
            id: id.into(),
        }
    }

    pub fn service(id: impl Into<String>) -> Self {
        Self {
            kind: FbaActorKind::Service,
            id: id.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FbaActorKind {
    User,
    Service,
    System,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FbaError {
    pub kind: FbaErrorKind,
    pub code: String,
    pub message: String,
    pub retryable: bool,
}

impl FbaError {
    pub fn validation(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(FbaErrorKind::Validation, code, message, false)
    }

    pub fn timeout(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(FbaErrorKind::Timeout, code, message, true)
    }

    pub fn unavailable(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(FbaErrorKind::Unavailable, code, message, true)
    }

    pub fn new(
        kind: FbaErrorKind,
        code: impl Into<String>,
        message: impl Into<String>,
        retryable: bool,
    ) -> Self {
        Self {
            kind,
            code: code.into(),
            message: message.into(),
            retryable,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FbaErrorKind {
    Validation,
    NotFound,
    Conflict,
    Forbidden,
    Unavailable,
    Timeout,
    InvariantViolation,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_semantics_require_idempotency_key_and_deadline() {
        let context = FbaContext::new("tenant-a", FbaActor::user("user-a"), "ru", "corr-a");

        assert_eq!(
            context.require_write_semantics().unwrap_err().kind,
            FbaErrorKind::Validation
        );

        let context = context
            .with_idempotency_key("idem-a")
            .with_deadline(Duration::from_secs(3));
        assert!(context.require_write_semantics().is_ok());
    }

    #[test]
    fn unavailable_errors_are_retryable() {
        let error = FbaError::unavailable("inventory.remote_unavailable", "try later");

        assert_eq!(error.kind, FbaErrorKind::Unavailable);
        assert!(error.retryable);
    }
}
