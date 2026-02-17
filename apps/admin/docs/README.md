# admin docs

В этой папке хранится документация модуля `apps/admin`.

## Зафиксированный стек интеграции
- UI/state: `leptos`, `leptos_router`, `Resource`/actions.
- GraphQL transport: внутренний crate `crates/leptos-graphql` (тонкий слой).
- HTTP: `reqwest`.
- Typed GraphQL (опционально): `graphql-client` на уровне приложений.

Цель: использовать battle-tested библиотеки и минимальный внутренний glue-код.


## Текущее состояние improvement plan

В рамках `docs/admin-review-improvement-plan.md` уже реализованы ключевые элементы **Phase 1-2**:

- Auth session теперь хранит `refresh_token` и `expires_at`, а `AuthProvider` выполняет периодическую проверку срока жизни токена и обновление сессии.
- Dashboard получает `dashboardStats` через GraphQL `Resource` + `Suspense` fallback вместо хардкода.
- Users использует серверную фильтрацию с debounce-поиском и общие query-константы из `apps/admin/src/api/queries.rs`.

Открытые этапы (Phase 3+) остаются в плане как backlog для итеративной доработки.
