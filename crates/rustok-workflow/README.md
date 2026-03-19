# rustok-workflow

## Purpose

`rustok-workflow` owns workflow automation and execution history for RusToK.

## Responsibilities

- Provide `WorkflowModule` metadata for the runtime registry.
- Own workflow CRUD, execution engine, schedules, webhooks, and execution history.
- Publish the typed `workflows:*` and `workflow_executions:*` RBAC surface.

## Interactions

- Depends on `alloy` for script-backed workflow steps.
- Depends on `rustok-core` for module contracts, permissions, and shared runtime types.
- Used directly by `apps/server` workflow GraphQL, REST, and background runtime wiring.
- Declares permissions via `rustok-core::Permission`.
- `apps/server` enforces workflow permissions through `RbacService` or RBAC extractors before
  invoking workflow services.

## Entry points

- `WorkflowModule`
- `WorkflowService`
- `WorkflowEngine`
- `WorkflowCronScheduler`
- `WorkflowTriggerHandler`
