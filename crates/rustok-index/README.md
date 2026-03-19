# rustok-index

## Purpose

`rustok-index` owns read-model and indexing contracts for RusToK.

## Responsibilities

- Provide `IndexModule` metadata for the runtime registry.
- Define indexer traits and indexing runtime contracts.
- Own index migrations and index rebuild helpers.

## Interactions

- Depends on `rustok-core` for module contracts.
- Consumes domain events published by content, commerce, blog, forum, pages, and workflow paths.
- Used by `apps/server` runtime wiring for index rebuild and search-related integrations.
- Does not publish its own RBAC surface.
- Admin access to indexing operations is enforced by `apps/server` through the permissions
  of the domain being managed, not through direct role checks inside the module.

## Entry points

- `IndexModule`
- `Indexer`
- `LocaleIndexer`
- `IndexerContext`
- `IndexerRuntimeConfig`
