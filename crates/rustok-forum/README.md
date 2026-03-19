# rustok-forum

## Purpose

`rustok-forum` owns the forum domain built on top of the content module.

## Responsibilities

- Provide `ForumModule` metadata for the runtime registry.
- Own forum categories, topics, replies, and moderation workflows.
- Publish the typed RBAC surface for `forum_categories:*`, `forum_topics:*`,
  and `forum_replies:*`.

## Interactions

- Depends on `rustok-content` for shared content storage and orchestration primitives.
- Depends on `rustok-core` for module contracts, permissions, and `SecurityContext`.
- Used directly by `apps/server` forum GraphQL and REST adapters.
- Declares permissions via `rustok-core::Permission`.
- `apps/server` enforces forum permissions through `RbacService` or RBAC extractors, then passes
  a permission-aware `SecurityContext` into forum services.

## Entry points

- `ForumModule`
- `TopicService`
- `ReplyService`
- `CategoryService`
- `ModerationService`
