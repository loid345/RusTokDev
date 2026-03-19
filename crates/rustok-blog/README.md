# rustok-blog

## Purpose

`rustok-blog` owns the blog domain built on top of the content module.

## Responsibilities

- Provide `BlogModule` metadata for the runtime registry.
- Own blog-specific post lifecycle, SEO, and localized blog orchestration.
- Publish the typed `blog_posts:*` RBAC surface.

## Interactions

- Depends on `rustok-content` for shared content storage and orchestration primitives.
- Depends on `rustok-core` for module contracts, permissions, and `SecurityContext`.
- Used directly by `apps/server` blog GraphQL and REST adapters.
- Declares permissions via `rustok-core::Permission`.
- `apps/server` enforces `blog_posts:*` through `RbacService` or RBAC extractors, then passes
  a permission-aware `SecurityContext` into blog services.

## Entry points

- `BlogModule`
- `PostService`
- `CommentService`
- `CategoryService`
- `TagService`
