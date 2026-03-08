# Health endpoints (`apps/server`)

Документ описывает поведение health endpoints в `apps/server/src/controllers/health.rs`.

## Endpoints

- `GET /health` — базовый статус процесса и версия приложения.
- `GET /health/live` — liveness probe (процесс жив).
- `GET /health/ready` — readiness probe с агрегированным статусом зависимостей и модулей.
- `GET /health/modules` — health только по зарегистрированным модулям.

## Readiness модель

`/health/ready` возвращает общий статус и детальные проверки:

- `status`: `ok | degraded | unhealthy`
- `checks`: инфраструктурные проверки
- `modules`: проверки health для модулей из `ModuleRegistry`
- `degraded_reasons`: список причин деградации

### Актуальные dependency checks

- `database` — критичная проверка доступности БД.
- `cache_backend` — базовая проверка tenant cache path.
- `tenant_cache_invalidation` — не-критичная проверка внешнего Redis pubsub listener для cross-instance invalidation.
- `event_transport` — критичная проверка инициализации event transport.
- `search_backend` — не-критичная проверка search connectivity.

Для `tenant_cache_invalidation` действует следующая семантика:

- `disabled` или `healthy` не деградируют readiness.
- `starting` или `degraded` переводят `/health/ready` в `degraded`, но не в `unhealthy`.
- текущее состояние дополнительно отражается в `/metrics` как `rustok_tenant_invalidation_listener_status` (`0=disabled`, `1=starting`, `2=healthy`, `3=degraded`).

### Поля проверки

Каждая запись в `checks` и `modules` содержит:

- `name`: имя проверки (например, `database`, `search_backend`, `module:content`)
- `kind`: `dependency` или `module`
- `criticality`: `critical` или `non_critical`
- `status`: `ok | degraded | unhealthy`
- `latency_ms`: время выполнения проверки
- `reason`: причина деградации/ошибки (опционально)

## Агрегация статуса

- Если есть `critical` проверка со статусом `unhealthy` → общий `status = unhealthy`.
- Если `unhealthy` для critical нет, но есть не-`ok` проверки → общий `status = degraded`.
- Если все проверки `ok` → общий `status = ok`.

## Надёжность проверок

Для каждой readiness-проверки используются защитные механизмы:

- timeout на выполнение проверки,
- in-process circuit breaker (порог ошибок + cooldown),
- fail-fast поведение при открытом circuit.

Это предотвращает зависание `/health/ready` при проблемной зависимости.
