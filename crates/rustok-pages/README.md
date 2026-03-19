# rustok-pages

## Purpose

`rustok-pages` owns static pages, blocks, and menus for RusToK.

## Responsibilities

- Provide `PagesModule` metadata for the runtime registry.
- Own page, block, and menu services layered on top of content storage.
- Publish the typed RBAC surface for `pages:*` and the node-based block helpers it needs.

## Interactions

- Depends on `rustok-content` for shared node storage and content helpers.
- Depends on `rustok-core` for module contracts, permissions, and `SecurityContext`.
- Used directly by `apps/server` pages GraphQL and REST adapters.
- Declares permissions via `rustok-core::Permission`.
- `apps/server` enforces page permissions through `RbacService` or RBAC extractors, then passes
  a permission-aware `SecurityContext` into page services.

## Entry points

- `PagesModule`
- `PageService`
- `BlockService`
- `MenuService`
