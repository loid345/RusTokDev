# rustok-workflow-admin

Leptos admin UI package for the `rustok-workflow` module.

## Responsibilities

- Exposes the workflow admin root view used by `apps/admin`.
- Keeps workflow-specific admin UI inside the module boundary instead of `apps/admin`.
- Participates in manifest-driven host composition through `rustok-module.toml`.

## Entry Points

- `WorkflowAdmin` — root admin page component for the module.
- `rustok-module.toml [provides.admin_ui]` advertises `leptos_crate`, `route_segment`, and `nav_label`.

## Interactions

- Consumed by `apps/admin` via manifest-driven `build.rs` code generation.
- Mounted by the Leptos admin host under `/modules/workflow` through the generic module page route.
- Uses shared auth and GraphQL transport hooks exposed to admin-side Leptos packages.
- Temporarily links workflow rows back to the legacy `/workflows/:id` detail flow until richer nested admin routes are implemented.

## Documentation

- See [platform docs](../../../docs/index.md).
