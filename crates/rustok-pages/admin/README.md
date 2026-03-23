# rustok-pages-admin

## Purpose

`rustok-pages-admin` publishes the Leptos admin root page for the `rustok-pages` module.

## Responsibilities

- Export the module-owned `PagesAdmin` root component for `apps/admin`.
- Keep pages-specific admin UI inside the module boundary instead of `apps/admin`.
- Act as the publishable root package for future page CRUD, menu, and builder screens.

## Interactions

- Used by `apps/admin` through manifest-driven generated wiring.
- Depends on `rustok-pages` for module ownership and shared types.
- Follows the generic host route contract `/modules/:module_slug`.

## Entry points

- `PagesAdmin`
