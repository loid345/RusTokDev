# Rate Limiting в RusToK

## Обзор

RusToK использует sliding-window rate limiter в `apps/server` для защиты HTTP API от brute force, abuse и кратковременных всплесков трафика. Этот limiter является именно HTTP-слоем и не заменяет `rustok-core::security::RateLimiter`, который остаётся внутренним security primitive.

## Актуальный контракт

- идентификация клиента идёт по IP;
- приоритет источников IP: `X-Forwarded-For` -> `X-Real-IP` -> `ip:unknown`;
- `X-User-ID` сознательно игнорируется;
- при превышении лимита сервер возвращает `429 Too Many Requests`;
- в ответе используются `Retry-After`, `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`;
- `X-RateLimit-Reset` и `Retry-After` сейчас имеют одинаковую семантику: число секунд до сброса активного окна.

## Почему `X-User-ID` не используется

HTTP middleware не должен доверять identity headers, которые клиент может подменить. Иначе атакующий сможет:

- тратить чужой bucket;
- обходить собственный лимит;
- ломать соответствие между rate limiting и реально подтверждённой auth-сессией.

Если later понадобится user-aware throttling, его нужно добавлять только после доверенной аутентификации и на основе проверенных claims.

## Конфигурация

```rust
use rustok_server::middleware::rate_limit::RateLimitConfig;

let default_config = RateLimitConfig::default();
let strict_config = RateLimitConfig::new(20, 60);
let disabled_config = RateLimitConfig::disabled();
```

Текущий wiring в `apps/server/src/app.rs`:

- `/api/*` -> 300 запросов в 60 секунд на IP;
- `/api/auth/login`, `/api/auth/register`, `/api/auth/reset*` -> 20 запросов в 60 секунд на IP.

## Подключение middleware

```rust
use axum::middleware as axum_middleware;
use rustok_server::middleware::rate_limit::{
    cleanup_task, rate_limit_for_paths, RateLimitConfig, RateLimiter,
};
use std::sync::Arc;

let limiter = Arc::new(RateLimiter::new(RateLimitConfig::new(300, 60)));
let prefixes = Arc::new(vec!["/api/"]);

let limiter_for_cleanup = limiter.clone();
tokio::spawn(async move {
    cleanup_task(limiter_for_cleanup).await;
});

let app = router.layer(axum_middleware::from_fn_with_state(
    (limiter, prefixes),
    rate_limit_for_paths,
));
```

## Формат ответов

Успешный ответ:

```http
HTTP/1.1 200 OK
X-RateLimit-Limit: 300
X-RateLimit-Remaining: 299
X-RateLimit-Reset: 60
```

Ответ при превышении лимита:

```http
HTTP/1.1 429 Too Many Requests
Retry-After: 42
X-RateLimit-Limit: 20
X-RateLimit-Remaining: 0
X-RateLimit-Reset: 42
Content-Type: text/plain

Rate limit exceeded
```

## Перепроверка

```bash
cargo test -p rustok-server rate_limit
cargo check -p rustok-server --bin rustok-server
```

## Что дальше

Перед переходом на `governor` или `tower-governor` нужно сохранить этот HTTP-контракт без регрессий. Любая библиотечная замена должна сначала подтвердить совместимость по headers, `retry-after`, cleanup semantics и path-aware behavior.
