# rustok-events

`rustok-events` is the canonical event-contracts crate for RusToK.

## Current state

This crate owns `DomainEvent`, `EventEnvelope`, event schema metadata, and event validation
rules. `rustok-core` keeps compatibility re-exports so existing consumers can migrate
without a breaking cut-over.

## Goal

Keep domain event contracts independent from runtime infrastructure while preserving
backward compatibility for downstream modules.

## Public API

- `rustok_events::DomainEvent`
- `rustok_events::EventEnvelope`
- `rustok_events::EventSchema`
- `rustok_events::FieldSchema`
- `rustok_events::event_schema`
- `rustok_events::EVENT_SCHEMAS`
- `rustok_events::ValidateEvent`
- `rustok_events::EventValidationError`

## Compatibility

- `rustok_core::events::{DomainEvent, EventEnvelope}` remains available as a transition path.
- `rustok_events::RootDomainEvent` and `rustok_events::RootEventEnvelope` are stable aliases
  for compatibility-oriented consumers and tests.

## Contract policy

- Prefer additive payload evolution; keep `event_type()` stable for non-breaking changes.
- Treat schema-version bumps as breaking changes that require docs and migration notes.
- Require validation, schema registry coverage, and roundtrip tests for every event variant.

## Release gate

- `cargo test -p rustok-events`
- `cargo check -p rustok-events -p rustok-core`
- Confirm schema registry coverage matches the full `DomainEvent` variant set
