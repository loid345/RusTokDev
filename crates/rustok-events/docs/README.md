# rustok-events documentation

## Purpose

`rustok-events` provides the canonical, shared import surface for RusToK domain event
contracts. It owns `DomainEvent`, `EventEnvelope`, schema metadata, and validation rules,
while `rustok-core` exposes compatibility re-exports for transition safety.

## Responsibilities

- Maintain a stable API path for event contracts.
- Maintain schema metadata and validation rules used by transports, outbox, and consumers.
- Decouple consumer crates from direct event-type imports in `rustok-core`.
- Provide compatibility aliases and contract tests for non-breaking event evolution.

## Interactions

- **Compatibility adapter:** `rustok-core::events` re-exports canonical contracts.
- **Downstream consumers:** domain modules, transport/runtime crates, and test utilities
  that exchange events through RusToK event flows.
- **Architecture alignment:** event contracts are aligned with centralized event flow
  documentation and outbox/runtime guidance.

## Entry points

- `rustok_events::DomainEvent`
- `rustok_events::EventEnvelope`
- `rustok_events::EventSchema`
- `rustok_events::FieldSchema`
- `rustok_events::event_schema`
- `rustok_events::EVENT_SCHEMAS`
- `rustok_events::ValidateEvent`
- Compatibility exports:
  - `rustok_events::RootDomainEvent`
  - `rustok_events::RootEventEnvelope`

## Related docs

- [Implementation plan](./implementation-plan.md)
- [Platform documentation map](../../../docs/index.md)
- [Event flow contract](../../../docs/architecture/event-flow-contract.md)

## Versioning policy

- Import event contracts from `rustok_events`; `rustok_core::events` is compatibility-only.
- `event_type()` strings are stable API and must not change for an existing payload shape.
- Additive changes are the default path: only append optional fields or relax validation in a
  backward-compatible way.
- Breaking payload changes require a schema-version bump, explicit migration note, and dual-read
  plan for consumers that deserialize stored envelopes.
- Every new `DomainEvent` variant must ship with validation rules, a schema registry entry, and
  a representative contract test sample.

## Release gate

Before merging an event-contract change:

1. Run `cargo test -p rustok-events`.
2. Run `cargo check -p rustok-events -p rustok-core`.
3. Confirm `EVENT_SCHEMAS` exactly matches `DomainEvent::event_type()` coverage.
4. Document any breaking or deprecating change in this docs folder and the architecture notes.
5. Verify envelope JSON still roundtrips for canonical and compatibility aliases.
