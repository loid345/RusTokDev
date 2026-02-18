# tailwind-rs (vendored in RusToK)

This crate is vendored locally because the upstream `leptos + tailwind-rs` toolchain was unstable for our build.

## Why only a part was fixed

We intentionally fixed and vendored only the path that RusToK actually executes in CI and local builds:

- `tailwind-rs` CLI integration used by `apps/admin/Trunk.toml`.
- Core translation path through `tailwind-css`/`tailwind-ast`.
- Compatibility patches required for current workspace dependency resolution (`tailwind-error`, `parcel_css`).

The full upstream surface was **not** completed because:

1. Upstream still contains explicit `todo!()` / `unimplemented!()` branches in error adapters and some instruction paths.
2. `tailwind-rs` in this workspace is effectively focused on HTML processing (`feature = "html"`), not every possible parser/input mode.
3. We prioritized build stability for RusToK apps over full parity with all Tailwind semantics.

## Practical implication

If a Tailwind class/syntax path reaches an unimplemented branch upstream, the local fork may still not support it yet.

For product work this is acceptable as long as used classes compile in `admin`/`storefront` pipelines.
