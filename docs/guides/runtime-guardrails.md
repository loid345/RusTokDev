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
- `rate_limits[*].policy` - effective limiter contract on this instance:
  `enabled`, `max_requests`, `window_seconds`, `trusted_auth_dimensions`,
  `memory_warning_entries`, `memory_critical_entries`;
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
4. `rate_limits[*].policy`
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

The same effective limiter contract is also exported through `/metrics` as:

- `rustok_runtime_guardrail_rate_limit_config{namespace,setting}`

This is useful for rollout diffing across multiple instances.

## Namespace Thresholds

Runtime guardrails evaluate memory-backed limiter saturation independently for:

- `api`
- `auth`
- `oauth`

These thresholds are configured under `rustok.runtime.guardrails.rate_limit_memory_thresholds`.

## Alert Matrix

Recommended baseline alerting:

| Signal | Namespace | Severity | Suggested `for` | Why |
| --- | --- | --- | --- | --- |
| `rustok_runtime_guardrail_rate_limit_backend_healthy == 0` | `api` | critical | `1m` | shared API limiter backend is unavailable |
| `rustok_runtime_guardrail_rate_limit_backend_healthy == 0` | `auth` | critical | `1m` | auth protection is degraded |
| `rustok_runtime_guardrail_rate_limit_backend_healthy == 0` | `oauth` | critical | `1m` | token/revoke flows lost their backend |
| `rustok_runtime_guardrail_rate_limit_state == 1` | `api` | warning | `10m` | memory limiter cardinality is growing |
| `rustok_runtime_guardrail_rate_limit_state == 2` | `api` | critical | `5m` | API limiter is at stop-the-line saturation |
| `rustok_runtime_guardrail_rate_limit_state == 1` | `auth` | warning | `5m` | brute-force protection buckets are accumulating too fast |
| `rustok_runtime_guardrail_rate_limit_state == 2` | `auth` | critical | `2m` | auth limiter is saturated |
| `rustok_runtime_guardrail_rate_limit_state == 1` | `oauth` | warning | `5m` | OAuth client activity is producing excessive limiter cardinality |
| `rustok_runtime_guardrail_rate_limit_state == 2` | `oauth` | critical | `2m` | OAuth limiter is saturated |
| `rustok_runtime_guardrail_event_transport_fallback_active == 1` | global | critical | `1m` | relay target is in fallback mode |
| `rustok_runtime_guardrail_event_backpressure_state == 1` | global | warning | `5m` | queue is approaching budget |
| `rustok_runtime_guardrail_event_backpressure_state == 2` | global | critical | `2m` | queue is in critical backpressure |

## Example Prometheus Rules

```yaml
groups:
  - name: rustok_runtime_guardrails
    rules:
      - alert: RustokRateLimitBackendUnavailable
        expr: rustok_runtime_guardrail_rate_limit_backend_healthy == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "Rate-limit backend unavailable for {{ $labels.namespace }}"
          description: "The {{ $labels.namespace }} limiter backend is unhealthy."

      - alert: RustokRateLimitSaturationWarning
        expr: rustok_runtime_guardrail_rate_limit_state == 1
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Rate-limit saturation warning for {{ $labels.namespace }}"
          description: "The {{ $labels.namespace }} limiter crossed its warning threshold."

      - alert: RustokRateLimitSaturationCritical
        expr: rustok_runtime_guardrail_rate_limit_state == 2
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "Rate-limit saturation critical for {{ $labels.namespace }}"
          description: "The {{ $labels.namespace }} limiter crossed its critical threshold."

      - alert: RustokEventRelayFallbackActive
        expr: rustok_runtime_guardrail_event_transport_fallback_active == 1
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "Event relay fallback is active"
          description: "Event transport is running in fallback mode."
```

## Multi-Instance Rollout

Recommended rollout for multi-instance deployments:

1. Keep `runtime.guardrails.rollout=observe` during the first Redis-backed rollout window.
2. Set `rate_limit.backend=redis` for all limiter namespaces in the deployment.
3. Verify `/health/runtime` shows `rate_limits[*].distributed=true`.
4. Watch `rustok_runtime_guardrail_rate_limit_backend_healthy` and backend latency/errors before switching to `enforce`.
5. Only then promote the deployment to `runtime.guardrails.rollout=enforce`.

Repository preset:

- [`production.redis.example.yaml`](/C:/проекты/RusTok/apps/server/config/production.redis.example.yaml)

Stop-the-line conditions for rollout:

- any limiter backend becomes unhealthy;
- event relay fallback becomes active;
- event bus backpressure reaches critical state;
- Redis is reachable but limiter state oscillates between healthy and unavailable.

## Related Files

- [`health.rs`](/C:/проекты/RusTok/apps/server/src/controllers/health.rs)
- [`metrics.rs`](/C:/проекты/RusTok/apps/server/src/controllers/metrics.rs)
- [`runtime_guardrails.rs`](/C:/проекты/RusTok/apps/server/src/services/runtime_guardrails.rs)
- [`rate-limiting.md`](/C:/проекты/RusTok/docs/guides/rate-limiting.md)
