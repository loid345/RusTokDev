# rustok-pages-storefront

## Purpose

`rustok-pages-storefront` publishes the Leptos storefront root view for the `rustok-pages` module.

## Responsibilities

- Export the module-owned `PagesView` root component for `apps/storefront`.
- Keep pages-specific storefront rendering inside the module boundary.
- Act as the publishable storefront package for future page and menu rendering flows.

## Interactions

- Used by `apps/storefront` through manifest-driven generated wiring.
- Depends on `rustok-pages` for module ownership and shared types.
- Follows the generic storefront host contract: slots plus `/modules/:route_segment`.

## Entry points

- `PagesView`
