# rustok-rbac module implementation plan (`rustok-rbac`)

## Scope and objective

This document captures the current implementation plan for `rustok-rbac` in RusToK and
serves as the source of truth for rollout sequencing in `crates/rustok-rbac`.

Primary objective: evolve `rustok-rbac` in small, testable increments while preserving
compatibility with platform-level contracts.

## Target architecture

- `rustok-rbac` remains focused on its bounded context and public crate API.
- Integrations with other modules go through stable interfaces in `rustok-core`
  (or dedicated integration crates where applicable).
- Behavior changes are introduced through additive, backward-compatible steps.
- Observability and operability requirements are part of delivery readiness.

## Delivery phases

### Phase 0 — Foundation (done)

- [x] Baseline crate/module structure is in place.
- [x] Base docs and registry presence are established.
- [x] Core compile-time integration with the workspace is available.

### Phase 1 — Contract hardening (in progress)

- [x] Freeze initial public RBAC runtime API: exported `permission_policy`/`permission_evaluator` + trait contract `PermissionResolver`/`PermissionResolution` with default use-case methods (`has_*`) for adapter-driven integrations.
- [x] Introduce shared permission-policy helpers (`permission_policy`) and start consuming them from `apps/server` extractors/service wiring to reduce server-owned policy logic.
- [x] Introduce shared permission evaluation API (`permission_evaluator`) and move allow/deny + missing-permissions outcome assembly from server-side RBAC wiring into `rustok-rbac`.
- [x] Align error/validation conventions with platform guidance (added typed `RbacError` validation path for authz-mode parsing via `RbacAuthzMode::try_parse` while preserving backward-compatible fallback in `parse`).
- [x] Expand automated tests around core invariants and boundary behavior (including stable normalized permission payload from both relation and cache paths, empty-requirements decision contract, and resolver error propagation in `permission_authorizer`).

### Phase 2 — Domain expansion (planned)

- [x] Implement prioritized domain capabilities for `rustok-rbac` (module now owns `permission_authorizer` use-case evaluation, relation-resolve orchestration via `RelationPermissionStore`, shared cache-aware resolver path (`resolve_permissions_with_cache` + `PermissionCache`) and runtime resolver service `RuntimePermissionResolver` with assignment contract `RoleAssignmentStore` (including role-assignment removal operations); `apps/server` consumes module runtime resolver instead of local `ServerPermissionResolver`).
- [x] Move authz rollout mode contract (`RbacAuthzMode` for `relation_only`/`casbin_shadow`/`casbin_only`) into `rustok-rbac` to reduce server-owned RBAC control-plane logic and keep rollout switching on one canonical env key: `RUSTOK_RBAC_AUTHZ_MODE`.
- [x] Keep shadow-check shape primitives (`shadow_decision::ShadowCheck`) inside `rustok-rbac` so `apps/server` keeps only transport/observability concerns for shadow execution.
- [x] Add module-level Casbin shadow runtime orchestration so `apps/server` does not keep local branching for skip/compare outcomes.
- [x] Standardize cross-module integration points and events (published canonical RBAC role-assignment integration event contract: `RbacRoleAssignmentEvent` + `RbacIntegrationEventKind` + stable `rbac.*` event-type constants).
- [x] Document ownership and release gates for new capabilities (added module owner, review boundaries, and release-gate checklist to `crates/rustok-rbac/docs/README.md`).

### Phase 3 — Productionization (planned)

- [x] Finalize rollout and migration strategy for incremental adoption (documented module-level incremental rollout sequence and compatibility behavior in `crates/rustok-rbac/docs/README.md`).
- [ ] Complete security/tenancy/rbac checks relevant to the module.
- [ ] Validate observability, runbooks, and operational readiness.

## Tracking and updates

When updating `rustok-rbac` architecture, API contracts, tenancy behavior, routing,
or observability expectations:

1. Update this file first.
2. Update `crates/rustok-rbac/README.md` and `crates/rustok-rbac/docs/README.md` when public behavior changes.
3. Update `docs/index.md` links if documentation structure changes.
4. If module responsibilities change, update `docs/modules/registry.md` accordingly.

## Checklist

- [x] контрактные тесты покрывают все публичные use-case.


