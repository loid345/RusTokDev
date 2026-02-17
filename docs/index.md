# Documentation Map

This index serves as the central navigation hub for RusToK documentation.

## Root Documents

- [System Manifest](../RUSTOK_MANIFEST.md) - Core architecture, philosophy, and invariants.
- [Admin Panel Review](../ADMIN_PANEL_REVIEW.md) - Detailed review of the admin panel.
- [Agent Rules](../AGENTS.md) - Guidelines for AI agents working on the codebase.
- [Changelog](../CHANGELOG.md) - Version history and changes.
- [License](../LICENSE) - MIT License.

## Architecture (`docs/architecture/`)

- [Overview](./architecture/overview.md) - High-level system architecture.
- [Database Schema](./architecture/database.md) - Database tables and relationships.
- [API Architecture](./architecture/api.md) - API design and patterns.
- [RBAC Enforcement](./architecture/rbac.md) - Role-based access control.
- [Dataloader](./architecture/dataloader.md) - Efficient data fetching patterns.
- [Modules Overview](./architecture/modules.md) - Module system and dependency matrix.
- [Routing Policy](./architecture/routing.md) - API routing and request handling.
- [Events & Outbox](./architecture/events.md) - Event-driven architecture details.
- [Transactional Publishing](./architecture/events-transactional.md) - Atomic event publishing.
- [Tenancy](./architecture/tenancy.md) - Multi-tenancy implementation.
- [Principles](./architecture/principles.md) - Core architectural principles.

- [Admin Review Improvement Plan](./admin-review-improvement-plan.md) - Implementation roadmap for Leptos admin hardening.

## Guides (`docs/guides/`)

- [Quick Start](./guides/quickstart.md) - Getting started with RusToK.
- [Observability](./guides/observability-quickstart.md) - Setting up and using observability tools.
- [Circuit Breaker](./guides/circuit-breaker.md) - Resilience patterns.
- [State Machines](./guides/state-machine.md) - Type-safe state machine guide.
- [Error Handling](./guides/error-handling.md) - Error handling policies and patterns.
- [Input Validation](./guides/input-validation.md) - Data validation standards.
- [Rate Limiting](./guides/rate-limiting.md) - API rate limiting guide.
- [Module Metrics](./guides/module-metrics.md) - Tracking module performance.
- [Testing](./guides/testing.md) - General testing guidelines.
- [Integration Testing](./guides/testing-integration.md) - Writing integration tests.
- [Property Testing](./guides/testing-property.md) - Property-based testing guide.
- [Security Audit](./guides/security-audit.md) - Security audit procedures.
- [Lockfile Troubleshooting](./guides/lockfile-troubleshooting.md) - Handling Cargo.lock issues.

## Modules (`docs/modules/`)

- [Overview](./modules/overview.md) - General module system documentation.
- [Registry](./modules/registry.md) - Module registry and lifecycle.
- [Manifest](./modules/manifest.md) - Module manifest format.

## UI (`docs/ui/`)

- [UI Overview](./ui/README.md) - Entry point for UI documentation.
- [GraphQL Architecture](./ui/graphql-architecture.md) - Frontend GraphQL usage.

## Application Documentation

- [Server](../apps/server/docs/README.md)
- [Admin Panel](../apps/admin/docs/README.md)
- [Storefront](../apps/storefront/README.md)
- [Next.js Storefront](../apps/next-frontend/docs/README.md)

## Crate Documentation

Documentation for specific crates is located in their respective `crates/*/README.md` files.
