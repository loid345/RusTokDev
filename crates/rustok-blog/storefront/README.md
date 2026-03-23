# rustok-blog-storefront

Leptos storefront UI package for the `rustok-blog` module.

## Responsibilities

- Exposes the blog storefront root view used by `apps/storefront`.
- Keeps blog-specific storefront UI inside the module package.
- Participates in the manifest-driven UI composition path through `rustok-module.toml`.

## Entry Points

- `BlogView` — root storefront view rendered from the host storefront slot registry.

## Interactions

- Consumed by `apps/storefront` via manifest-driven `build.rs` code generation.
- Uses `rustok-blog` domain contracts and shared Leptos host libraries.
- Should remain compatible with the host storefront slot contract.

## Documentation

- See [platform docs](../../../docs/index.md).
