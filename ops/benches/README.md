# Benchmarks (`ops/benches`)

This is a standalone workspace crate named `rustok-benchmarks`.

## What is here

- `Cargo.toml` — benchmark crate manifest.
- `benches/*.rs` — Criterion benchmark suites:
  - `tenant_cache.rs`
  - `state_machine.rs`
  - `event_bus.rs`
  - `content_operations.rs`
  - `order_operations.rs`

## Purpose

These benchmarks detect performance regressions and provide repeatable latency/throughput baselines for critical platform paths.

## Typical usage

- Run all benchmarks through workspace tooling.
- Add a new benchmark under `benches/` and register it in `Cargo.toml`.

This folder is operationally important; it is not a scaffold or placeholder.
