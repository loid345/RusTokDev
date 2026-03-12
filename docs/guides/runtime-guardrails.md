# Runtime Guardrails

This guide describes the operator-facing contract for runtime guardrails in `apps/server`.

## Why It Exists

Runtime guardrails aggregate three production signals into one rollout-aware snapshot:

- HTTP rate-limit backend health and saturation;
- event transport fallback state;
- event bus backpressure state.

The goal is to let operators answer two questions quickly:

1. Is the runtime healthy enough to keep serving traffic?
2. Which subsystem is currently pushing the platform into `degraded` or `critical` mode?

## Endpoints

Structured snapshot:

```text
GET /health/runtime
```

Machine-readable readiness:

```text
GET /health/ready
```

Prometheus metrics:

```text
GET /metrics
```

## Snapshot Shape

`GET /health/runtime` returns:

- `status` - effective runtime status after rollout policy is applied;
- `observed_status` - raw status before rollout softening;
- `rollout` - `observe` or `enforce`;
- `reasons` - human-readable reasons for degradation;
- `rate_limits` - per-namespace limiter state (`api`, `auth`, `oauth`);
- `event_bus` - backpressure budget snapshot;
- `event_transport` - relay fallback state.

## Rollout Semantics

`observe` means the runtime can surface a problem without failing the whole guardrail policy hard.

- `observed_status=critical`
- `status=degraded`

This is useful during staged rollout when you want visibility before making the signal stop-the-line.

`enforce` means the effective status follows the observed severity exactly.

## What To Look At First

If `status != ok`, check fields in this order:

1. `reasons`
2. `rate_limits[*].healthy`
3. `rate_limits[*].state`
4. `event_transport.relay_fallback_active`
5. `event_bus.state`

This ordering matches the current production policy:

- backend-unavailable limiter state is critical;
- memory limiter saturation becomes warning or critical by configured per-namespace thresholds;
- relay fallback is critical;
- event bus backpressure warning/critical maps directly from queue state.

## Common Cases

Rate-limit backend unavailable:

- `rate_limits[*].healthy = false`
- likely means Redis or another distributed backend is unreachable;
- `/health/ready` should include a matching `runtime_guardrails` reason.

Memory limiter saturation:

- `rate_limits[*].distributed = false`
- `total_entries` crossed warning or critical thresholds;
- the fix is usually to reduce cardinality, shorten retention, or move to a distributed backend.
- current cardinality can include trusted JWT dimensions: base IP plus verified `tenant` and, for OAuth tokens, `oauth_app`.
- thresholds are namespace-specific, so `api`, `auth`, and `oauth` can page at different saturation levels.

Event relay fallback active:

- `event_transport.relay_fallback_active = true`
- production should treat this as a real degradation, not a harmless warning.

Event bus backpressure warning:

- `event_bus.state = degraded` or `critical`
- `current_depth` is approaching or hitting `max_depth`;
- `events_rejected` tells you whether the system already started dropping work.

## Suggested Operator Flow

1. Check `GET /health/runtime`.
2. Confirm the same condition appears in `GET /health/ready`.
3. Check `/metrics` for the matching Prometheus series.
4. If the issue is limiter-related, verify backend reachability and cardinality growth.
5. If the issue is event-related, inspect relay target and queue depth before restarting anything.

## Trusted Rate-Limit Dimensions

When `rustok.rate_limit.trusted_auth_dimensions=true`, limiter keys keep the base IP bucket and extend it only from verified bearer-token claims:

- direct JWT: `ip + tenant`
- OAuth JWT: `ip + tenant + oauth_app`

This avoids spoofable identity headers while still making multi-tenant and OAuth traffic less noisy than pure per-IP limiting.

## Namespace Thresholds

Runtime guardrails evaluate memory-backed limiter saturation independently for:

- `api`
- `auth`
- `oauth`

These thresholds are configured under `rustok.runtime.guardrails.rate_limit_memory_thresholds`.

## Related Files

- [`health.rs`](/C:/проекты/RusTok/apps/server/src/controllers/health.rs)
- [`metrics.rs`](/C:/проекты/RusTok/apps/server/src/controllers/metrics.rs)
- [`runtime_guardrails.rs`](/C:/проекты/RusTok/apps/server/src/services/runtime_guardrails.rs)
- [`rate-limiting.md`](/C:/проекты/RusTok/docs/guides/rate-limiting.md)
