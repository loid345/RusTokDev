# alloy-scripting

## Purpose

`alloy-scripting` owns the Rhai-based scripting runtime for RusToK automation.

## Responsibilities

- Provide the `AlloyModule` runtime metadata used by `apps/server`.
- Own script storage, execution contracts, and migrations.
- Publish the typed `scripts:*` RBAC surface.

## Interactions

- Depends on `rustok-core` for module contracts and permission vocabulary.
- Used by `apps/server` for script CRUD, execution, and automation wiring.
- Used by `rustok-workflow` for script-backed workflow steps.
- Declares permissions via `rustok-core::Permission`.
- `apps/server` enforces `scripts:*` through `RbacService` or RBAC extractors before
  dispatching script operations.

## Entry points

- `AlloyModule` via `apps/server/src/modules/alloy.rs`
- script storage and execution APIs from `alloy-scripting`
- migrations and runtime configuration helpers
