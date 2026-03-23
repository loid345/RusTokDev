# rustok-api docs

This folder contains the local documentation for `crates/rustok-api`.

## Scope

`rustok-api` is the shared API adapter layer that sits between `rustok-core` and application-specific wiring in `apps/server`. It owns reusable request/auth/tenant/GraphQL primitives, while module-specific resolvers and controllers are migrated into module crates incrementally.

## Architectural Boundary

- `rustok-api` stays intentionally thin.
- It is the single shared host/API layer for request, tenant, auth, and GraphQL helper contracts.
- Do not introduce a second parallel implementation of the same layer inside `apps/server` or in per-module helper crates.
- If a helper is module-specific, keep it inside that module. If it becomes a shared host contract, move it into `rustok-api`.
