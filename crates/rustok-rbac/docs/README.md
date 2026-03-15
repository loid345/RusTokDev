# rustok-rbac docs

В этой папке хранится документация модуля `crates/rustok-rbac`.

## Documents

- [Implementation plan](./implementation-plan.md)

## Event contracts

- [Event flow contract (central)](../../../docs/architecture/event-flow-contract.md)


## Runtime contracts

- `PermissionResolver` / `PermissionResolution` define tenant-aware RBAC resolver contract for adapter-based integrations (`apps/server` and future transports), including default `has_permission/has_any_permission/has_all_permissions` use-cases powered by module evaluator APIs.
- `permission_policy` and `permission_evaluator` remain canonical policy/evaluation helpers for allow/deny semantics.
- `RuntimePermissionResolver` composes relation-store/cache/role-assignment adapters and supports unified resolver error mapping (`Into<E>`) so integration layers can keep their own adapter-specific error types; the same resolver contract is used for both read-path permission resolution and write-path role assignment/replacement/removal use-cases.
- `RbacAuthzMode` defines the shared rollout-mode contract for authorization execution and is configured only through `RUSTOK_RBAC_AUTHZ_MODE` (`relation_only|relation-only|relation`, `casbin_shadow|casbin-shadow|casbin_shadow_read`, `casbin_only|casbin-only|casbin`). `RbacAuthzMode::try_parse` exposes typed validation errors (`RbacError::InvalidAuthzMode`) for strict callers, while `parse` keeps fallback to `RelationOnly`.
- `permission_authorizer` is mode-aware: relation modes use the relation evaluator, while `casbin_only` routes allow/deny through the Casbin-compatible evaluator and stamps the active engine into `AuthorizationDecision`.
- `casbin_shadow_evaluator` exports `evaluate_casbin_shadow`, `compare_casbin_shadow_decision`, `evaluate_casbin_shadow_comparison`, and `permissions_for_shadow_check` for matcher-compatible in-module Casbin shadow checks and unified mismatch payload generation (`single|any|all`) without changing active decision path; `CasbinShadowComparison::checked_permissions_total()` provides stable mismatch cardinality for adapter telemetry.
- `shadow_decision` exports `ShadowCheck`, which keeps stable `single|any|all` labels and flat permission iteration for shadow evaluators and adapter observability.
- `integration` exports canonical RBAC cross-module event contract for role-assignment change notifications: `RbacRoleAssignmentEvent`, `RbacIntegrationEventKind`, and stable event-type constants (`rbac.role_permissions_assigned`, `rbac.user_role_replaced`, `rbac.tenant_role_assignments_removed`, `rbac.user_role_assignment_removed`). Integration payloads are `serde`-serializable (`snake_case` enum tags) for transport-agnostic publish/consume flows.


## Ownership and release gates

- **Module owner:** Platform foundation team (`apps/server` + `crates/rustok-core` ownership group from repository map).
- **Change scope requiring owner review:**
  - public API exports in `crates/rustok-rbac/src/lib.rs`;
  - resolver contracts (`PermissionResolver`, `RuntimePermissionResolver`, relation/cache assignment traits);
  - rollout controls (`RbacAuthzMode`, Casbin shadow contracts);
  - integration event contracts under `integration` (`rbac.*` event-type constants and payload structs).
- **Release gate checklist for RBAC module changes:**
  1. Unit tests for changed domain logic are present/updated in `crates/rustok-rbac/src/**` (or explained why not needed).
  2. `rustfmt` passes for touched Rust files.
  3. `apps/server` adapter compatibility is validated (compile/tests in network-enabled CI or documented local limitation).
  4. Module docs are updated (`crates/rustok-rbac/docs/*`) and, when migration milestones change, central architecture docs are synced (`docs/architecture/rbac-relation-migration-plan.md`).


## Incremental rollout strategy

For safe incremental adoption of relation RBAC in integration layers (`apps/server` and future transports), use the following sequence:

1. **relation-only baseline**: keep `RUSTOK_RBAC_AUTHZ_MODE=relation_only` (or alias `relation`) as the authoritative decision source.
2. **Casbin shadow**: switch to `RUSTOK_RBAC_AUTHZ_MODE=casbin_shadow` for relation-vs-Casbin parity checks while keeping relation decision authoritative.
3. **mismatch burn-down**: track `rustok_rbac_engine_mismatch_total` and shadow compare failures in transport observability and resolve drift sources before cutover.
4. **Casbin cutover**: switch to `casbin_only` after a stable zero-engine-mismatch window.

Operational note: when `RUSTOK_RBAC_AUTHZ_MODE` is absent, runtime defaults to `relation_only`.
