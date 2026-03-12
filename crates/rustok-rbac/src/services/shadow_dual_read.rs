use rustok_core::UserRole;

use super::{
    authz_mode::RbacAuthzMode,
    shadow_decision::{compare_shadow_decision, ShadowCheck, ShadowDecision},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DualReadOutcome {
    Skipped,
    Compared(DualReadComparison),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DualReadEvaluation {
    pub skip_reason: Option<DualReadSkipReason>,
    pub mismatch_records: Vec<DualReadMismatchRecord>,
}

impl DualReadEvaluation {
    pub fn skipped(&self) -> bool {
        self.skip_reason.is_some()
    }

    pub fn has_mismatch(&self) -> bool {
        !self.mismatch_records.is_empty()
    }

    pub fn telemetry(&self) -> DualReadTelemetry {
        DualReadTelemetry {
            decision_mismatch_delta: u64::from(self.has_mismatch()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DualReadTelemetry {
    pub decision_mismatch_delta: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DualReadSkipReason {
    ModeDisabled,
    UserNotFound,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DualReadComparison {
    pub decision: ShadowDecision,
    pub legacy_role: UserRole,
    pub required_permissions: Vec<rustok_core::Permission>,
}

impl DualReadComparison {
    pub fn mismatch_records(&self, shadow_check: ShadowCheck<'_>) -> Vec<DualReadMismatchRecord> {
        if !self.decision.mismatch() {
            return Vec::new();
        }

        vec![DualReadMismatchRecord {
            shadow_check: shadow_check.as_str(),
            legacy_role: self.legacy_role.clone(),
            required_permissions: self.required_permissions.clone(),
            relation_allowed: self.decision.relation_allowed,
            legacy_allowed: self.decision.legacy_allowed,
        }]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DualReadMismatchRecord {
    pub shadow_check: &'static str,
    pub legacy_role: UserRole,
    pub required_permissions: Vec<rustok_core::Permission>,
    pub relation_allowed: bool,
    pub legacy_allowed: bool,
}

pub fn evaluate_dual_read(
    legacy_role: Option<&UserRole>,
    shadow_check: ShadowCheck<'_>,
    relation_allowed: bool,
) -> DualReadOutcome {
    match legacy_role {
        Some(legacy_role) => DualReadOutcome::Compared(DualReadComparison {
            decision: compare_shadow_decision(legacy_role, shadow_check, relation_allowed),
            legacy_role: legacy_role.clone(),
            required_permissions: shadow_check_permissions(shadow_check),
        }),
        None => DualReadOutcome::Skipped,
    }
}

pub fn evaluate_dual_read_result(
    legacy_role: Option<&UserRole>,
    shadow_check: ShadowCheck<'_>,
    relation_allowed: bool,
) -> DualReadEvaluation {
    match evaluate_dual_read(legacy_role, shadow_check, relation_allowed) {
        DualReadOutcome::Skipped => DualReadEvaluation {
            skip_reason: Some(DualReadSkipReason::UserNotFound),
            mismatch_records: Vec::new(),
        },
        DualReadOutcome::Compared(comparison) => DualReadEvaluation {
            skip_reason: None,
            mismatch_records: comparison.mismatch_records(shadow_check),
        },
    }
}

pub fn evaluate_dual_read_for_mode(
    authz_mode: RbacAuthzMode,
    legacy_role: Option<&UserRole>,
    shadow_check: ShadowCheck<'_>,
    relation_allowed: bool,
) -> DualReadEvaluation {
    if !authz_mode.should_run_legacy_role_shadow() {
        return DualReadEvaluation {
            skip_reason: Some(DualReadSkipReason::ModeDisabled),
            mismatch_records: Vec::new(),
        };
    }

    evaluate_dual_read_result(legacy_role, shadow_check, relation_allowed)
}

fn shadow_check_permissions(shadow_check: ShadowCheck<'_>) -> Vec<rustok_core::Permission> {
    let mut permissions = Vec::new();
    shadow_check.for_each_permission(|permission| permissions.push(permission));
    permissions
}

#[cfg(test)]
mod tests {
    use super::{
        evaluate_dual_read, evaluate_dual_read_for_mode, evaluate_dual_read_result,
        DualReadComparison, DualReadEvaluation, DualReadMismatchRecord, DualReadOutcome,
        DualReadSkipReason, DualReadTelemetry,
    };
    use crate::services::{
        authz_mode::RbacAuthzMode,
        shadow_decision::{ShadowCheck, ShadowDecision},
    };
    use rustok_core::{Action, Permission, Resource, UserRole};

    fn permission(resource: Resource, action: Action) -> Permission {
        Permission::new(resource, action)
    }

    #[test]
    fn returns_compared_when_legacy_role_present() {
        let required = permission(Resource::BlogPosts, Action::Read);

        let outcome = evaluate_dual_read(
            Some(&UserRole::Manager),
            ShadowCheck::Single(&required),
            true,
        );

        assert_eq!(
            outcome,
            DualReadOutcome::Compared(DualReadComparison {
                decision: ShadowDecision {
                    legacy_allowed: true,
                    relation_allowed: true,
                },
                legacy_role: UserRole::Manager,
                required_permissions: vec![required],
            })
        );
    }

    #[test]
    fn returns_skipped_when_legacy_role_missing() {
        let required = permission(Resource::Users, Action::Read);

        let outcome = evaluate_dual_read(None, ShadowCheck::Single(&required), true);
        assert_eq!(outcome, DualReadOutcome::Skipped);
    }

    #[test]
    fn returns_matched_when_relation_equals_legacy() {
        let required = permission(Resource::BlogPosts, Action::Read);

        let outcome = evaluate_dual_read(
            Some(&UserRole::Manager),
            ShadowCheck::Single(&required),
            true,
        );

        assert_eq!(
            outcome,
            DualReadOutcome::Compared(DualReadComparison {
                decision: ShadowDecision {
                    legacy_allowed: true,
                    relation_allowed: true,
                },
                legacy_role: UserRole::Manager,
                required_permissions: vec![required],
            })
        );
    }

    #[test]
    fn returns_mismatch_when_relation_differs_from_legacy() {
        let required = permission(Resource::Users, Action::Delete);

        let outcome = evaluate_dual_read(
            Some(&UserRole::Manager),
            ShadowCheck::Single(&required),
            true,
        );

        assert_eq!(
            outcome,
            DualReadOutcome::Compared(DualReadComparison {
                decision: ShadowDecision {
                    legacy_allowed: false,
                    relation_allowed: true,
                },
                legacy_role: UserRole::Manager,
                required_permissions: vec![required],
            })
        );
    }

    #[test]
    fn compared_outcome_returns_structured_mismatch_record() {
        let required = permission(Resource::Users, Action::Delete);
        let outcome = evaluate_dual_read(
            Some(&UserRole::Manager),
            ShadowCheck::Single(&required),
            true,
        );

        let DualReadOutcome::Compared(comparison) = outcome else {
            panic!("expected compared outcome");
        };

        assert_eq!(
            comparison.mismatch_records(ShadowCheck::Single(&required)),
            vec![DualReadMismatchRecord {
                shadow_check: "single",
                legacy_role: UserRole::Manager,
                required_permissions: vec![required],
                relation_allowed: true,
                legacy_allowed: false,
            }]
        );
    }

    #[test]
    fn compared_outcome_returns_no_records_when_no_mismatch() {
        let required = permission(Resource::BlogPosts, Action::Read);
        let outcome = evaluate_dual_read(
            Some(&UserRole::Manager),
            ShadowCheck::Single(&required),
            true,
        );

        let DualReadOutcome::Compared(comparison) = outcome else {
            panic!("expected compared outcome");
        };

        assert!(comparison
            .mismatch_records(ShadowCheck::Single(&required))
            .is_empty());
    }

    #[test]
    fn evaluation_reports_skipped_state() {
        let required = permission(Resource::Users, Action::Read);

        assert_eq!(
            evaluate_dual_read_result(None, ShadowCheck::Single(&required), true),
            DualReadEvaluation {
                skip_reason: Some(DualReadSkipReason::UserNotFound),
                mismatch_records: vec![],
            }
        );
    }

    #[test]
    fn evaluation_reports_mismatch_records_without_exposing_compared_branch() {
        let required = permission(Resource::Users, Action::Delete);

        assert_eq!(
            evaluate_dual_read_result(
                Some(&UserRole::Manager),
                ShadowCheck::Single(&required),
                true,
            ),
            DualReadEvaluation {
                skip_reason: None,
                mismatch_records: vec![DualReadMismatchRecord {
                    shadow_check: "single",
                    legacy_role: UserRole::Manager,
                    required_permissions: vec![required],
                    relation_allowed: true,
                    legacy_allowed: false,
                }],
            }
        );
    }

    #[test]
    fn evaluation_exposes_counter_delta_telemetry() {
        let required = permission(Resource::Users, Action::Delete);

        assert_eq!(
            evaluate_dual_read_result(
                Some(&UserRole::Manager),
                ShadowCheck::Single(&required),
                true,
            )
            .telemetry(),
            DualReadTelemetry {
                decision_mismatch_delta: 1,
            }
        );

        assert_eq!(
            evaluate_dual_read_result(
                Some(&UserRole::Manager),
                ShadowCheck::Single(&permission(Resource::BlogPosts, Action::Read)),
                true,
            )
            .telemetry(),
            DualReadTelemetry {
                decision_mismatch_delta: 0,
            }
        );
    }

    #[test]
    fn mode_aware_evaluation_skips_when_dual_read_disabled() {
        let required = permission(Resource::Users, Action::Delete);

        assert_eq!(
            evaluate_dual_read_for_mode(
                RbacAuthzMode::RelationOnly,
                Some(&UserRole::Manager),
                ShadowCheck::Single(&required),
                true,
            ),
            DualReadEvaluation {
                skip_reason: Some(DualReadSkipReason::ModeDisabled),
                mismatch_records: vec![],
            }
        );
    }
}
