use crate::{has_effective_permission_in_set, RbacAuthzMode, ShadowCheck};
use rustok_core::Permission;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CasbinShadowComparison {
    pub decision: CasbinShadowDecision,
    pub checked_permissions: Vec<Permission>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CasbinShadowEvaluation {
    pub skip_reason: Option<CasbinShadowSkipReason>,
    pub mismatch_records: Vec<CasbinShadowMismatchRecord>,
}

impl CasbinShadowEvaluation {
    pub fn skipped(&self) -> bool {
        self.skip_reason.is_some()
    }

    pub fn has_mismatch(&self) -> bool {
        !self.mismatch_records.is_empty()
    }

    pub fn telemetry(&self) -> CasbinShadowTelemetry {
        if self.skipped() {
            return CasbinShadowTelemetry {
                relation_decisions_delta: 0,
                casbin_decisions_delta: 0,
                engine_mismatch_delta: 0,
            };
        }

        CasbinShadowTelemetry {
            relation_decisions_delta: 1,
            casbin_decisions_delta: 1,
            engine_mismatch_delta: u64::from(self.has_mismatch()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CasbinShadowTelemetry {
    pub relation_decisions_delta: u64,
    pub casbin_decisions_delta: u64,
    pub engine_mismatch_delta: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CasbinShadowSkipReason {
    ModeDisabled,
}

impl CasbinShadowComparison {
    pub fn checked_permissions_total(&self) -> usize {
        self.checked_permissions.len()
    }

    pub fn mismatch_records(
        &self,
        shadow_check: ShadowCheck<'_>,
    ) -> Vec<CasbinShadowMismatchRecord> {
        if !self.decision.mismatch() {
            return Vec::new();
        }

        let shadow_check = shadow_check.as_str();
        let checked_permissions_total = self.checked_permissions_total();

        self.checked_permissions
            .iter()
            .copied()
            .map(|permission| CasbinShadowMismatchRecord {
                shadow_check,
                checked_permissions_total,
                permission,
                relation_allowed: self.decision.relation_allowed,
                casbin_allowed: self.decision.casbin_allowed,
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CasbinShadowMismatchRecord {
    pub shadow_check: &'static str,
    pub checked_permissions_total: usize,
    pub permission: Permission,
    pub relation_allowed: bool,
    pub casbin_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CasbinShadowDecision {
    pub relation_allowed: bool,
    pub casbin_allowed: bool,
}

impl CasbinShadowDecision {
    pub fn mismatch(self) -> bool {
        self.relation_allowed != self.casbin_allowed
    }
}

pub fn evaluate_casbin_shadow(
    _tenant_id: &uuid::Uuid,
    resolved_permissions: &[Permission],
    shadow_check: ShadowCheck<'_>,
) -> bool {
    match shadow_check {
        ShadowCheck::Single(permission) => {
            has_effective_permission_in_set(resolved_permissions, permission)
        }
        ShadowCheck::Any(required_permissions) => required_permissions
            .iter()
            .any(|permission| has_effective_permission_in_set(resolved_permissions, permission)),
        ShadowCheck::All(required_permissions) => required_permissions
            .iter()
            .all(|permission| has_effective_permission_in_set(resolved_permissions, permission)),
    }
}

pub fn compare_casbin_shadow_decision(
    tenant_id: &uuid::Uuid,
    resolved_permissions: &[Permission],
    shadow_check: ShadowCheck<'_>,
    relation_allowed: bool,
) -> CasbinShadowDecision {
    CasbinShadowDecision {
        relation_allowed,
        casbin_allowed: evaluate_casbin_shadow(tenant_id, resolved_permissions, shadow_check),
    }
}

pub fn evaluate_casbin_shadow_comparison(
    tenant_id: &uuid::Uuid,
    resolved_permissions: &[Permission],
    shadow_check: ShadowCheck<'_>,
    relation_allowed: bool,
) -> CasbinShadowComparison {
    let decision = compare_casbin_shadow_decision(
        tenant_id,
        resolved_permissions,
        shadow_check,
        relation_allowed,
    );
    let checked_permissions = permissions_for_shadow_check(shadow_check);

    CasbinShadowComparison {
        decision,
        checked_permissions,
    }
}

pub fn evaluate_casbin_shadow_result(
    tenant_id: &uuid::Uuid,
    resolved_permissions: &[Permission],
    shadow_check: ShadowCheck<'_>,
    relation_allowed: bool,
) -> CasbinShadowEvaluation {
    let comparison = evaluate_casbin_shadow_comparison(
        tenant_id,
        resolved_permissions,
        shadow_check,
        relation_allowed,
    );

    CasbinShadowEvaluation {
        skip_reason: None,
        mismatch_records: comparison.mismatch_records(shadow_check),
    }
}

pub fn evaluate_casbin_shadow_for_mode(
    authz_mode: RbacAuthzMode,
    tenant_id: &uuid::Uuid,
    resolved_permissions: &[Permission],
    shadow_check: ShadowCheck<'_>,
    relation_allowed: bool,
) -> CasbinShadowEvaluation {
    if !authz_mode.should_run_casbin_shadow() {
        return CasbinShadowEvaluation {
            skip_reason: Some(CasbinShadowSkipReason::ModeDisabled),
            mismatch_records: Vec::new(),
        };
    }

    evaluate_casbin_shadow_result(
        tenant_id,
        resolved_permissions,
        shadow_check,
        relation_allowed,
    )
}

pub fn permissions_for_shadow_check(shadow_check: ShadowCheck<'_>) -> Vec<Permission> {
    let mut permissions = Vec::new();
    shadow_check.for_each_permission(|permission| permissions.push(permission));
    permissions
}

#[cfg(test)]
mod tests {
    use super::{
        compare_casbin_shadow_decision, evaluate_casbin_shadow, evaluate_casbin_shadow_comparison,
        evaluate_casbin_shadow_for_mode, evaluate_casbin_shadow_result,
        permissions_for_shadow_check, CasbinShadowEvaluation, CasbinShadowMismatchRecord,
        CasbinShadowSkipReason, CasbinShadowTelemetry,
    };
    use crate::{RbacAuthzMode, ShadowCheck};
    use rustok_core::Permission;

    #[test]
    fn casbin_shadow_allows_single_matching_permission() {
        let result = evaluate_casbin_shadow(
            &uuid::Uuid::new_v4(),
            &[Permission::USERS_READ],
            ShadowCheck::Single(&Permission::USERS_READ),
        );

        assert!(result);
    }

    #[test]
    fn casbin_shadow_denies_missing_permission() {
        let result = evaluate_casbin_shadow(
            &uuid::Uuid::new_v4(),
            &[Permission::USERS_READ],
            ShadowCheck::Single(&Permission::USERS_UPDATE),
        );

        assert!(!result);
    }

    #[test]
    fn casbin_shadow_any_all_respect_manage_wildcard() {
        let tenant_id = uuid::Uuid::new_v4();
        let permissions = [Permission::USERS_MANAGE];

        let allows_any = evaluate_casbin_shadow(
            &tenant_id,
            &permissions,
            ShadowCheck::Any(&[Permission::USERS_READ, Permission::USERS_DELETE]),
        );
        let allows_all = evaluate_casbin_shadow(
            &tenant_id,
            &permissions,
            ShadowCheck::All(&[Permission::USERS_READ, Permission::USERS_UPDATE]),
        );

        assert!(allows_any);
        assert!(allows_all);
    }

    #[test]
    fn compare_reports_mismatch_state() {
        let tenant_id = uuid::Uuid::new_v4();
        let decision = compare_casbin_shadow_decision(
            &tenant_id,
            &[Permission::USERS_READ],
            ShadowCheck::Single(&Permission::USERS_READ),
            false,
        );

        assert!(decision.mismatch());
        assert!(decision.casbin_allowed);
        assert!(!decision.relation_allowed);
    }

    #[test]
    fn permissions_for_shadow_check_returns_flat_set() {
        let single = permissions_for_shadow_check(ShadowCheck::Single(&Permission::USERS_READ));
        let any = permissions_for_shadow_check(ShadowCheck::Any(&[
            Permission::USERS_READ,
            Permission::USERS_UPDATE,
        ]));

        assert_eq!(single, vec![Permission::USERS_READ]);
        assert_eq!(any, vec![Permission::USERS_READ, Permission::USERS_UPDATE]);
    }

    #[test]
    fn comparison_includes_decision_and_checked_permissions() {
        let tenant_id = uuid::Uuid::new_v4();
        let comparison = evaluate_casbin_shadow_comparison(
            &tenant_id,
            &[Permission::USERS_READ],
            ShadowCheck::Any(&[Permission::USERS_READ, Permission::USERS_UPDATE]),
            true,
        );

        assert!(!comparison.decision.mismatch());
        assert_eq!(
            comparison.checked_permissions,
            vec![Permission::USERS_READ, Permission::USERS_UPDATE]
        );
        assert_eq!(comparison.checked_permissions_total(), 2);
    }

    #[test]
    fn mismatch_records_are_empty_when_decisions_match() {
        let tenant_id = uuid::Uuid::new_v4();
        let comparison = evaluate_casbin_shadow_comparison(
            &tenant_id,
            &[Permission::USERS_READ],
            ShadowCheck::Single(&Permission::USERS_READ),
            true,
        );

        assert!(comparison
            .mismatch_records(ShadowCheck::Single(&Permission::USERS_READ))
            .is_empty());
    }

    #[test]
    fn mismatch_records_include_structured_payload_for_each_checked_permission() {
        let tenant_id = uuid::Uuid::new_v4();
        let required = [Permission::USERS_READ, Permission::USERS_UPDATE];
        let comparison = evaluate_casbin_shadow_comparison(
            &tenant_id,
            &[Permission::USERS_READ],
            ShadowCheck::Any(&required),
            false,
        );

        assert_eq!(
            comparison.mismatch_records(ShadowCheck::Any(&required)),
            vec![
                CasbinShadowMismatchRecord {
                    shadow_check: "any",
                    checked_permissions_total: 2,
                    permission: Permission::USERS_READ,
                    relation_allowed: false,
                    casbin_allowed: true,
                },
                CasbinShadowMismatchRecord {
                    shadow_check: "any",
                    checked_permissions_total: 2,
                    permission: Permission::USERS_UPDATE,
                    relation_allowed: false,
                    casbin_allowed: true,
                },
            ]
        );
    }

    #[test]
    fn result_returns_empty_mismatch_records_when_decisions_match() {
        let tenant_id = uuid::Uuid::new_v4();

        assert_eq!(
            evaluate_casbin_shadow_result(
                &tenant_id,
                &[Permission::USERS_READ],
                ShadowCheck::Single(&Permission::USERS_READ),
                true,
            ),
            CasbinShadowEvaluation {
                skip_reason: None,
                mismatch_records: vec![],
            }
        );
    }

    #[test]
    fn result_returns_structured_mismatch_records() {
        let tenant_id = uuid::Uuid::new_v4();
        let required = [Permission::USERS_READ, Permission::USERS_UPDATE];

        assert_eq!(
            evaluate_casbin_shadow_result(
                &tenant_id,
                &[Permission::USERS_READ],
                ShadowCheck::Any(&required),
                false,
            ),
            CasbinShadowEvaluation {
                skip_reason: None,
                mismatch_records: vec![
                    CasbinShadowMismatchRecord {
                        shadow_check: "any",
                        checked_permissions_total: 2,
                        permission: Permission::USERS_READ,
                        relation_allowed: false,
                        casbin_allowed: true,
                    },
                    CasbinShadowMismatchRecord {
                        shadow_check: "any",
                        checked_permissions_total: 2,
                        permission: Permission::USERS_UPDATE,
                        relation_allowed: false,
                        casbin_allowed: true,
                    },
                ],
            }
        );
    }

    #[test]
    fn result_exposes_counter_delta_telemetry() {
        let tenant_id = uuid::Uuid::new_v4();
        let required = [Permission::USERS_READ, Permission::USERS_UPDATE];

        assert_eq!(
            evaluate_casbin_shadow_result(
                &tenant_id,
                &[Permission::USERS_READ],
                ShadowCheck::Any(&required),
                false,
            )
            .telemetry(),
            CasbinShadowTelemetry {
                relation_decisions_delta: 1,
                casbin_decisions_delta: 1,
                engine_mismatch_delta: 1,
            }
        );

        assert_eq!(
            evaluate_casbin_shadow_result(
                &tenant_id,
                &[Permission::USERS_READ],
                ShadowCheck::Single(&Permission::USERS_READ),
                true,
            )
            .telemetry(),
            CasbinShadowTelemetry {
                relation_decisions_delta: 1,
                casbin_decisions_delta: 1,
                engine_mismatch_delta: 0,
            }
        );
    }

    #[test]
    fn mode_aware_result_skips_when_casbin_shadow_disabled() {
        let tenant_id = uuid::Uuid::new_v4();

        assert_eq!(
            evaluate_casbin_shadow_for_mode(
                RbacAuthzMode::RelationOnly,
                &tenant_id,
                &[Permission::USERS_READ],
                ShadowCheck::Single(&Permission::USERS_READ),
                true,
            ),
            CasbinShadowEvaluation {
                skip_reason: Some(CasbinShadowSkipReason::ModeDisabled),
                mismatch_records: vec![],
            }
        );
    }
}
