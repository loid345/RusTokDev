# Rate Limiting in RusToK

## Overview

RusToK uses a sliding-window HTTP rate limiter in `apps/server` to protect API paths from brute force, abuse, and short traffic spikes.

This limiter is an HTTP-layer control. It does not replace `rustok-core::security::RateLimiter`, which remains an internal security primitive.

## Current Contract

- client identity starts from IP address;
- IP priority is `X-Forwarded-For` -> `X-Real-IP` -> `ip:unknown`;
- spoofable headers such as `X-User-ID` are ignored;
- rate-limit violations return `429 Too Many Requests`;
- responses expose `Retry-After`, `X-RateLimit-Limit`, `X-RateLimit-Remaining`, and `X-RateLimit-Reset`.

## Trusted Dimensions

When `rustok.rate_limit.trusted_auth_dimensions=true`, the limiter keeps the base IP bucket and extends it only from verified bearer-token claims:

- direct bearer token -> `ip + tenant`
- OAuth bearer token -> `ip + tenant + oauth_app`

Requests without a valid bearer token stay on the plain IP bucket. This is especially important for `/api/oauth/token` and `/api/oauth/revoke`, where client input is not trusted before verification.

## Namespaces

Current wiring in `apps/server/src/app.rs` uses separate limiter namespaces:

- `/api/*` -> `api`
- `/api/auth/login`, `/api/auth/register`, `/api/auth/reset*` -> `auth`
- `/api/oauth/token`, `/api/oauth/revoke`, `/api/oauth/authorize` -> `oauth`

This lets runtime guardrails, metrics, and backend health be tracked separately per limiter policy.

## Configuration

Relevant settings live under `rustok.rate_limit`:

- `enabled`
- `backend`
- `redis_key_prefix`
- `requests_per_minute`
- `burst`
- `auth_requests_per_minute`
- `auth_burst`
- `oauth_requests_per_minute`
- `oauth_burst`
- `trusted_auth_dimensions`

## Response Contract

Successful response:

```http
HTTP/1.1 200 OK
X-RateLimit-Limit: 300
X-RateLimit-Remaining: 299
X-RateLimit-Reset: 60
```

Rate-limited response:

```http
HTTP/1.1 429 Too Many Requests
Retry-After: 42
X-RateLimit-Limit: 20
X-RateLimit-Remaining: 0
X-RateLimit-Reset: 42
Content-Type: text/plain

Rate limit exceeded
```

## Verification

```bash
cargo test -p rustok-server rate_limit
cargo check -p rustok-server --bin rustok-server
```

## Related Docs

- [`runtime-guardrails.md`](/C:/проекты/RusTok/docs/guides/runtime-guardrails.md)
- [`improvement-recommendations.md`](/C:/проекты/RusTok/docs/architecture/improvement-recommendations.md)
