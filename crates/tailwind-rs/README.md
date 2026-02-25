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

## Can we vendor the whole library?

Yes. For easier future switch back to upstream, the best strategy is:

1. Keep a **full mirror** of upstream crates in our vendored paths.
2. Keep RusToK-only fixes in **separate commits** on top of that mirror.
3. On each upstream update, resync the mirror first, then reapply/adapt our local fixes.

To reduce manual copy-paste we added:

- `scripts/tailwind/vendor_tailwind_rs.sh` — syncs full upstream crates into:
  - `crates/tailwind-rs`
  - `crates/tailwind-css`
  - `crates/tailwind-ast`
  - `third_party/patches/tailwind-error`

Examples:

```bash
# sync latest default branch
scripts/tailwind/vendor_tailwind_rs.sh

# sync a specific tag/branch/sha
scripts/tailwind/vendor_tailwind_rs.sh --ref v0.2.0

# check drift without changing files
scripts/tailwind/vendor_tailwind_rs.sh --check
```

## Practical implication

If a Tailwind class/syntax path reaches an unimplemented branch upstream, the local fork may still not support it yet.

For product work this is acceptable as long as used classes compile in `admin`/`storefront` pipelines.
