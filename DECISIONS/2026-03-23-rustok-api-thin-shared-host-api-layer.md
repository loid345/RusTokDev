# `rustok-api` как тонкий и единый shared host/API layer

- Date: 2026-03-23
- Status: Accepted

## Context

RusToK движется к модели платформы, где optional-модули могут устанавливаться и обновляться как самостоятельные пакеты, а `apps/server` не должен знать о них модульно-специфичные детали. Для этого нужен стабильный shared contract между runtime host и модульными web-адаптерами:

- request/auth/tenant context;
- GraphQL helper-ы и error contract;
- request-level locale/tenant resolution;
- host-facing transport helper-ы, которые не относятся к domain logic.

Такой слой уже появился в виде `crates/rustok-api` и реально используется как сервером, так и рядом модульных crate-ов. При этом остаётся риск архитектурного дрейфа:

1. вернуть общие transport/helper-типы обратно в `apps/server`;
2. начать создавать параллельные helper-layer crate-ы рядом с отдельными модулями;
3. превратить `rustok-api` в «ещё один сервер», затянув в него module-specific resolvers, controllers и domain behavior.

Нужно зафиксировать границу явно, чтобы сохранить модель сторонних модулей и не получить несколько несовместимых реализаций одного и того же host contract.

Это решение совместимо с уже принятыми границами:

- `apps/server` остаётся composition root и server-infra/runtime host;
- module-specific transport code живёт в crate-ах модулей;
- infrastructure-level capabilities не выносятся без отдельного ADR в новые platform modules.

## Decision

1. `rustok-api` закрепляется как **тонкий и единственный shared host/API layer** для RusToK.
2. В `rustok-api` разрешено держать только общий host-level контракт, который реально переиспользуется между `apps/server` и модульными crate-ами:
   - `AuthContext`, `TenantContext`, `RequestContext`;
   - GraphQL helper-ы, pagination/error contract, module-enabled guard;
   - locale/tenant/request extraction primitives;
   - минимальные transport/runtime helper-ы без domain logic.
3. В `rustok-api` **нельзя** переносить:
   - module-specific resolvers и controllers;
   - module-specific business logic;
   - module manifests, module settings schema, UI policy, registry-специфику конкретного модуля;
   - composition-root wiring уровня `apps/server`.
4. `apps/server` может подключать и реэкспортировать `rustok-api`, но не должен развивать вторую параллельную реализацию того же shared host/API слоя.
5. Module crates могут зависеть от `rustok-api` для shared host contract, но их собственный transport/domain code остаётся локально в модуле.
6. Новый helper попадает в `rustok-api` только если он:
   - действительно shared минимум между несколькими модулями или между модулем и сервером;
   - относится к host/API boundary, а не к domain behavior конкретного модуля.
7. Любая попытка:
   - расширить `rustok-api` до platform business layer;
   - ввести второй shared API/helper layer;
   - переопределить эти границы,
   требует отдельного ADR.

## Consequences

### Плюсы

- Сохраняется единый contract surface для сторонних модулей.
- `apps/server` не деградирует обратно в место, где размазаны общие transport/helper-типы.
- Снижается риск нескольких несовместимых shared-layer реализаций.
- Проще двигаться к WordPress-подобной модели install/uninstall из админки, где сервер знает generic host contract, а не детали модулей.

### Компромиссы

- Нужна дисциплина при code review: не каждый helper должен попадать в `rustok-api`.
- Некоторые временные shim/re-export слои в `apps/server` ещё остаются до следующего этапа codegen/composition cleanup.
- Это решение само по себе не убирает ручную сборку composition root; оно фиксирует только boundary shared host/API layer.

### Follow-up

1. Использовать `rustok-api` как каноническую точку входа для новых shared host/API helper-ов.
2. Не добавлять новые parallel helper crates с той же ролью.
3. Продолжить вынос module-specific GraphQL/REST адаптеров в crate-ы модулей.
4. Отдельно довести codegen/composition-root automation, чтобы `apps/server` перестал вручную знать optional-модули.
