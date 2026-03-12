# rustok-events implementation plan

## Current status

- Status: **Phase 3 (contract hardening implemented)**.
- Summary: canonical event types, schema metadata, and validation now live in
  `rustok-events`; `rustok-core` remains a compatibility re-export layer; internal
  consumers import event contracts directly from `rustok-events`.
- Hardening additions now include versioning/deprecation guidance, release-gate
  documentation, full-variant contract tests, and exact parity checks between
  `DomainEvent::event_type()` and `EVENT_SCHEMAS`.

## Gap analysis

### Completed

- Created a dedicated `rustok-events` crate.
- Moved canonical `DomainEvent` and `EventEnvelope` ownership into `rustok-events`.
- Preserved backward compatibility through `rustok-core::events` re-exports.
- Migrated internal crates and apps to the canonical import path.
- Added schema registry coverage and compatibility-oriented contract tests.

### Remaining follow-up

- Wire the documented release gate into CI when crate publishing and release
  automation are formalized.
- Keep validating adjacent runtime and integration suites that still exercise the
  compatibility path around `rustok-core`, `rustok-outbox`, and transport layers.

## Work phases

### Phase 1 - Contract stabilization

- Confirm the public API as the recommended import surface.
- Document payload-evolution rules and non-breaking change expectations.
- Define release review requirements for event-schema changes.

### Phase 2 - Canonical ownership extraction

- Move event contracts from `rustok-core` into `rustok-events`.
- Keep a compatibility adapter in `rustok-core` during migration.
- Migrate downstream consumers to the new canonical source.

### Phase 3 - Production hardening

- Add contract tests for schema/version compatibility.
- Introduce a release checklist for backward compatibility review.
- Align event-contract changes with outbox, DLQ, replay, and reindex guidance.

## Definition of done

- Internal `rustok-*` crates import event contracts through `rustok-events`.
- Canonical `DomainEvent` and `EventEnvelope` definitions live in `rustok-events`.
- Schema changes include documented migration or deprecation notes when needed.
- Contract tests cover the current supported payload surface and envelope
  serialization behavior.

## Verification metrics

- Adoption rate: share of internal crates importing event contracts through
  `rustok-events` (target: 100%).
- Compatibility health: share of successful contract tests across supported
  versions (target: 100%).
- Breaking-change leakage: undocumented breaking event-schema changes per release
  (target: 0).
- Docs freshness: relevant architecture and crate docs updated with structural
  event-contract changes (target: 100% of relevant PRs).

## Checklist

<!-- Legacy test anchor kept until contract_surface.rs is updated:
РєРѕРЅС‚СЂР°РєС‚РЅС‹Рµ С‚РµСЃС‚С‹ РїРѕРєСЂС‹РІР°СЋС‚ РІСЃРµ РїСѓР±Р»РёС‡РЅС‹Рµ use-case
-->
- [x] Contract tests cover public event-contract use cases.
- [x] Canonical `DomainEvent` and `EventEnvelope` definitions live in `rustok-events`.
- [x] `rustok-core::events` works as a compatibility adapter over `rustok-events`.
- [x] Internal crates and apps import event contracts through `rustok-events`.
- [x] Release gates include schema-registry and envelope-serialization checks.
- [x] Schema registry exactly matches `DomainEvent::event_type()` across all variants.
- [x] Contract tests cover validation, schema coverage, and JSON roundtrip for the
  full `DomainEvent` enum.
