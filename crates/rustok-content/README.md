# rustok-content

## Purpose

`rustok-content` owns the core CMS domain for RusToK: nodes, posts, media, comments,
categories, and tags.

## Responsibilities

- Provide `ContentModule` metadata for the runtime registry.
- Own content entities, services, orchestration, and migrations.
- Publish the typed RBAC surface for content resources such as `nodes:*`, `posts:*`,
  `media:*`, `comments:*`, `categories:*`, and `tags:*`.

## Interactions

- Depends on `rustok-core` for permissions, events, and `SecurityContext`.
- Used directly by `apps/server` content REST and GraphQL adapters.
- Used as a storage/orchestration dependency by `rustok-blog`, `rustok-forum`, and `rustok-pages`.
- Declares permissions via `rustok-core::Permission`.
- `apps/server` enforces those permissions through `RbacService` or RBAC extractors, then passes
  a permission-aware `SecurityContext` into content services.

## Entry points

- `ContentModule`
- `NodeService`
- `ContentOrchestrationService`
- `CategoryService`
- `TagService`
- content DTO and entity re-exports
