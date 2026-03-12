use rustok_core::{Permission, UserRole};

use super::{
    authz_mode::RbacAuthzMode,
    casbin_shadow_evaluator::{
        evaluate_casbin_shadow_for_mode, CasbinShadowEvaluation, CasbinShadowMismatchRecord,
    },
    shadow_decision::ShadowCheck,
    shadow_dual_read::{
        evaluate_dual_read_for_mode, DualReadEvaluation, DualReadMismatchRecord, DualReadSkipReason,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShadowRuntimeEvaluation {
    pub dual_read: DualReadEvaluation,
    pub casbin_shadow: CasbinShadowEvaluation,
}

impl ShadowRuntimeEvaluation {
    pub fn telemetry(&self) -> ShadowRuntimeTelemetry {
        let dual_read = self.dual_read.telemetry();
        let casbin_shadow = self.casbin_shadow.telemetry();

        ShadowRuntimeTelemetry {
            decision_mismatch_delta: dual_read.decision_mismatch_delta,
            relation_decisions_delta: casbin_shadow.relation_decisions_delta,
            casbin_decisions_delta: casbin_shadow.casbin_decisions_delta,
            engine_mismatch_delta: casbin_shadow.engine_mismatch_delta,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShadowRuntimeTelemetry {
    pub decision_mismatch_delta: u64,
    pub relation_decisions_delta: u64,
    pub casbin_decisions_delta: u64,
    pub engine_mismatch_delta: u64,
}

pub struct ShadowRuntimeInput<'a> {
    pub authz_mode: RbacAuthzMode,
    pub tenant_id: &'a uuid::Uuid,
    pub legacy_role: Option<&'a UserRole>,
    pub resolved_permissions: &'a [Permission],
    pub shadow_check: ShadowCheck<'a>,
    pub relation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShadowRuntimeContext<'a> {
    pub tenant_id: &'a uuid::Uuid,
    pub user_id: &'a uuid::Uuid,
}

pub trait ShadowRuntimeObserver {
    fn record_decision_mismatch_delta(&self, delta: u64);

    fn record_engine_decision_deltas(&self, relation_delta: u64, casbin_delta: u64);

    fn record_engine_mismatch_delta(&self, delta: u64);

    fn record_engine_eval_duration(&self, latency_ms: u64);

    fn on_dual_read_user_not_found(&self, context: ShadowRuntimeContext<'_>);

    fn on_dual_read_mismatch(
        &self,
        context: ShadowRuntimeContext<'_>,
        mismatch: &DualReadMismatchRecord,
    );

    fn on_casbin_mismatch(
        &self,
        context: ShadowRuntimeContext<'_>,
        mismatch: &CasbinShadowMismatchRecord,
    );
}

pub fn evaluate_shadow_runtime_for_mode(input: ShadowRuntimeInput<'_>) -> ShadowRuntimeEvaluation {
    ShadowRuntimeEvaluation {
        dual_read: evaluate_dual_read_for_mode(
            input.authz_mode,
            input.legacy_role,
            input.shadow_check,
            input.relation_allowed,
        ),
        casbin_shadow: evaluate_casbin_shadow_for_mode(
            input.authz_mode,
            input.tenant_id,
            input.resolved_permissions,
            input.shadow_check,
            input.relation_allowed,
        ),
    }
}

pub fn shadow_runtime_needs_legacy_role(authz_mode: RbacAuthzMode) -> bool {
    authz_mode.should_run_legacy_role_shadow()
}

pub fn shadow_runtime_runs_casbin(authz_mode: RbacAuthzMode) -> bool {
    authz_mode.should_run_casbin_shadow()
}

pub fn observe_shadow_runtime(
    observer: &impl ShadowRuntimeObserver,
    context: ShadowRuntimeContext<'_>,
    evaluation: &ShadowRuntimeEvaluation,
    engine_eval_latency_ms: Option<u64>,
) {
    let telemetry = evaluation.telemetry();
    observer.record_decision_mismatch_delta(telemetry.decision_mismatch_delta);
    observer.record_engine_decision_deltas(
        telemetry.relation_decisions_delta,
        telemetry.casbin_decisions_delta,
    );
    observer.record_engine_mismatch_delta(telemetry.engine_mismatch_delta);

    if let Some(latency_ms) = engine_eval_latency_ms {
        observer.record_engine_eval_duration(latency_ms);
    }

    if evaluation.dual_read.skip_reason == Some(DualReadSkipReason::UserNotFound) {
        observer.on_dual_read_user_not_found(context);
    } else {
        for mismatch in &evaluation.dual_read.mismatch_records {
            observer.on_dual_read_mismatch(context, mismatch);
        }
    }

    for mismatch in &evaluation.casbin_shadow.mismatch_records {
        observer.on_casbin_mismatch(context, mismatch);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        evaluate_shadow_runtime_for_mode, observe_shadow_runtime, shadow_runtime_needs_legacy_role,
        shadow_runtime_runs_casbin, ShadowRuntimeContext, ShadowRuntimeEvaluation,
        ShadowRuntimeInput, ShadowRuntimeObserver, ShadowRuntimeTelemetry,
    };
    use crate::{
        CasbinShadowEvaluation, CasbinShadowMismatchRecord, CasbinShadowSkipReason,
        DualReadEvaluation, DualReadMismatchRecord, DualReadSkipReason, RbacAuthzMode, ShadowCheck,
    };
    use rustok_core::{Action, Permission, Resource, UserRole};
    use std::cell::RefCell;

    fn permission(resource: Resource, action: Action) -> Permission {
        Permission::new(resource, action)
    }

    #[test]
    fn runtime_combines_dual_read_and_casbin_evaluations() {
        let tenant_id = uuid::Uuid::new_v4();
        let required = permission(Resource::Users, Action::Delete);

        assert_eq!(
            evaluate_shadow_runtime_for_mode(ShadowRuntimeInput {
                authz_mode: RbacAuthzMode::CasbinShadow,
                tenant_id: &tenant_id,
                legacy_role: Some(&UserRole::Manager),
                resolved_permissions: &[permission(Resource::Users, Action::Read)],
                shadow_check: ShadowCheck::Single(&required),
                relation_allowed: true,
            }),
            ShadowRuntimeEvaluation {
                dual_read: DualReadEvaluation {
                    skip_reason: Some(DualReadSkipReason::ModeDisabled),
                    mismatch_records: vec![],
                },
                casbin_shadow: CasbinShadowEvaluation {
                    skip_reason: None,
                    mismatch_records: vec![CasbinShadowMismatchRecord {
                        shadow_check: "single",
                        checked_permissions_total: 1,
                        permission: required,
                        relation_allowed: true,
                        casbin_allowed: false,
                    }],
                },
            }
        );
    }

    #[test]
    fn runtime_telemetry_sums_shadow_paths() {
        let tenant_id = uuid::Uuid::new_v4();
        let required = permission(Resource::Users, Action::Delete);

        assert_eq!(
            evaluate_shadow_runtime_for_mode(ShadowRuntimeInput {
                authz_mode: RbacAuthzMode::DualRead,
                tenant_id: &tenant_id,
                legacy_role: Some(&UserRole::Manager),
                resolved_permissions: &[permission(Resource::Users, Action::Read)],
                shadow_check: ShadowCheck::Single(&required),
                relation_allowed: true,
            })
            .telemetry(),
            ShadowRuntimeTelemetry {
                decision_mismatch_delta: 1,
                relation_decisions_delta: 0,
                casbin_decisions_delta: 0,
                engine_mismatch_delta: 0,
            }
        );
    }

    #[test]
    fn runtime_reports_skip_reasons_for_disabled_paths() {
        let tenant_id = uuid::Uuid::new_v4();
        let required = permission(Resource::Users, Action::Read);

        let evaluation = evaluate_shadow_runtime_for_mode(ShadowRuntimeInput {
            authz_mode: RbacAuthzMode::RelationOnly,
            tenant_id: &tenant_id,
            legacy_role: None,
            resolved_permissions: &[required],
            shadow_check: ShadowCheck::Single(&required),
            relation_allowed: true,
        });

        assert_eq!(
            evaluation.dual_read.skip_reason,
            Some(DualReadSkipReason::ModeDisabled)
        );
        assert_eq!(
            evaluation.casbin_shadow.skip_reason,
            Some(CasbinShadowSkipReason::ModeDisabled)
        );
    }

    #[test]
    fn runtime_exposes_mode_helpers_for_server_adapters() {
        assert!(shadow_runtime_needs_legacy_role(RbacAuthzMode::DualRead));
        assert!(!shadow_runtime_needs_legacy_role(
            RbacAuthzMode::CasbinShadow
        ));
        assert!(shadow_runtime_runs_casbin(RbacAuthzMode::CasbinShadow));
        assert!(!shadow_runtime_runs_casbin(RbacAuthzMode::DualRead));
    }

    #[test]
    fn runtime_keeps_dual_read_mismatch_payloads() {
        let tenant_id = uuid::Uuid::new_v4();
        let required = permission(Resource::Users, Action::Delete);

        let evaluation = evaluate_shadow_runtime_for_mode(ShadowRuntimeInput {
            authz_mode: RbacAuthzMode::DualRead,
            tenant_id: &tenant_id,
            legacy_role: Some(&UserRole::Manager),
            resolved_permissions: &[permission(Resource::Users, Action::Read)],
            shadow_check: ShadowCheck::Single(&required),
            relation_allowed: true,
        });

        assert_eq!(
            evaluation.dual_read.mismatch_records,
            vec![DualReadMismatchRecord {
                shadow_check: "single",
                legacy_role: UserRole::Manager,
                required_permissions: vec![required],
                relation_allowed: true,
                legacy_allowed: false,
            }]
        );
    }

    #[derive(Debug, Clone, Default, PartialEq, Eq)]
    struct ObservedCalls {
        decision_mismatch_delta: u64,
        relation_decisions_delta: u64,
        casbin_decisions_delta: u64,
        engine_mismatch_delta: u64,
        engine_eval_duration_ms: Vec<u64>,
        dual_read_user_not_found_calls: usize,
        dual_read_mismatch_shadow_checks: Vec<&'static str>,
        casbin_mismatch_shadow_checks: Vec<&'static str>,
    }

    #[derive(Default)]
    struct RecordingObserver {
        calls: RefCell<ObservedCalls>,
    }

    impl RecordingObserver {
        fn snapshot(&self) -> ObservedCalls {
            self.calls.borrow().clone()
        }
    }

    impl ShadowRuntimeObserver for RecordingObserver {
        fn record_decision_mismatch_delta(&self, delta: u64) {
            self.calls.borrow_mut().decision_mismatch_delta += delta;
        }

        fn record_engine_decision_deltas(&self, relation_delta: u64, casbin_delta: u64) {
            let mut calls = self.calls.borrow_mut();
            calls.relation_decisions_delta += relation_delta;
            calls.casbin_decisions_delta += casbin_delta;
        }

        fn record_engine_mismatch_delta(&self, delta: u64) {
            self.calls.borrow_mut().engine_mismatch_delta += delta;
        }

        fn record_engine_eval_duration(&self, latency_ms: u64) {
            self.calls
                .borrow_mut()
                .engine_eval_duration_ms
                .push(latency_ms);
        }

        fn on_dual_read_user_not_found(&self, _context: ShadowRuntimeContext<'_>) {
            self.calls.borrow_mut().dual_read_user_not_found_calls += 1;
        }

        fn on_dual_read_mismatch(
            &self,
            _context: ShadowRuntimeContext<'_>,
            mismatch: &DualReadMismatchRecord,
        ) {
            self.calls
                .borrow_mut()
                .dual_read_mismatch_shadow_checks
                .push(mismatch.shadow_check);
        }

        fn on_casbin_mismatch(
            &self,
            _context: ShadowRuntimeContext<'_>,
            mismatch: &CasbinShadowMismatchRecord,
        ) {
            self.calls
                .borrow_mut()
                .casbin_mismatch_shadow_checks
                .push(mismatch.shadow_check);
        }
    }

    #[test]
    fn observer_helper_dispatches_telemetry_and_mismatch_records() {
        let observer = RecordingObserver::default();
        let tenant_id = uuid::Uuid::new_v4();
        let user_id = uuid::Uuid::new_v4();
        let required = permission(Resource::Users, Action::Delete);
        let evaluation = evaluate_shadow_runtime_for_mode(ShadowRuntimeInput {
            authz_mode: RbacAuthzMode::CasbinShadow,
            tenant_id: &tenant_id,
            legacy_role: Some(&UserRole::Manager),
            resolved_permissions: &[permission(Resource::Users, Action::Read)],
            shadow_check: ShadowCheck::Single(&required),
            relation_allowed: true,
        });

        observe_shadow_runtime(
            &observer,
            ShadowRuntimeContext {
                tenant_id: &tenant_id,
                user_id: &user_id,
            },
            &evaluation,
            Some(42),
        );

        assert_eq!(
            observer.snapshot(),
            ObservedCalls {
                decision_mismatch_delta: 0,
                relation_decisions_delta: 1,
                casbin_decisions_delta: 1,
                engine_mismatch_delta: 1,
                engine_eval_duration_ms: vec![42],
                dual_read_user_not_found_calls: 0,
                dual_read_mismatch_shadow_checks: vec![],
                casbin_mismatch_shadow_checks: vec!["single"],
            }
        );
    }

    #[test]
    fn observer_helper_dispatches_user_not_found_skip_without_fake_mismatch() {
        let observer = RecordingObserver::default();
        let tenant_id = uuid::Uuid::new_v4();
        let user_id = uuid::Uuid::new_v4();
        let required = permission(Resource::Users, Action::Read);
        let evaluation = evaluate_shadow_runtime_for_mode(ShadowRuntimeInput {
            authz_mode: RbacAuthzMode::DualRead,
            tenant_id: &tenant_id,
            legacy_role: None,
            resolved_permissions: &[required],
            shadow_check: ShadowCheck::Single(&required),
            relation_allowed: true,
        });

        observe_shadow_runtime(
            &observer,
            ShadowRuntimeContext {
                tenant_id: &tenant_id,
                user_id: &user_id,
            },
            &evaluation,
            None,
        );

        assert_eq!(
            observer.snapshot(),
            ObservedCalls {
                decision_mismatch_delta: 0,
                relation_decisions_delta: 0,
                casbin_decisions_delta: 0,
                engine_mismatch_delta: 0,
                engine_eval_duration_ms: vec![],
                dual_read_user_not_found_calls: 1,
                dual_read_mismatch_shadow_checks: vec![],
                casbin_mismatch_shadow_checks: vec![],
            }
        );
    }
}
