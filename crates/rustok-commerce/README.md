# rustok-commerce

## Purpose

`rustok-commerce` owns the commerce domain for RusToK: catalog, pricing, orders, customers,
inventory, and discounts.

## Responsibilities

- Provide `CommerceModule` metadata for the runtime registry.
- Own commerce services, entities, and migrations.
- Publish the typed RBAC surface for commerce resources.

## Interactions

- Depends on `rustok-core` for module contracts and permission vocabulary.
- Used directly by `apps/server` commerce REST and GraphQL adapters.
- Declares permissions via `rustok-core::Permission` for `products`, `orders`, `customers`,
  `inventory`, and `discounts`.
- `apps/server` enforces those permissions through `RbacService` and RBAC extractors before
  invoking commerce services.

## Entry points

- `CommerceModule`
- `CatalogService`
- `PricingService`
- `InventoryService`
- commerce DTO and state-machine re-exports
