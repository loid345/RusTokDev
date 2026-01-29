# Storefront (Leptos SSR)

RusToK storefront is an SSR-first Leptos app styled with Tailwind + DaisyUI. It
ships with a minimal landing layout that can be extended with product listings,
content blocks, and checkout flows.

## Run locally

```bash
cargo run -p rustok-storefront
```

The server listens on `http://localhost:3100`.

## Localization

The storefront currently supports English and Russian strings. Switch language
with the `lang` query parameter:

- English: `http://localhost:3100?lang=en`
- Russian: `http://localhost:3100?lang=ru`

Add more locales by extending the `locale_strings` mapping in
`apps/storefront/src/main.rs`.
