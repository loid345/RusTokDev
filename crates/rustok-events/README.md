# rustok-events

`rustok-events` is the shared event-contracts crate for RusToK.

## Current state

This crate is implemented as **Phase 1 extraction**: it re-exports `DomainEvent` and
`EventEnvelope` from `rustok-core` to provide a stable import path for event consumers.

## Goal

Move domain event contracts out of `rustok-core` incrementally, while preserving
backward compatibility for existing modules.

## Public API

- `rustok_events::DomainEvent`
- `rustok_events::EventEnvelope`
