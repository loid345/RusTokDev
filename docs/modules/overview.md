# Документация по модулям RusToK

Этот документ фиксирует текущее состояние модульной архитектуры в репозитории:
- какие `RusToKModule` зарегистрированы в `rustok-server`;
- какие из них Core/Optional;
- какие инфраструктурные crate'ы критичны для runtime, но не являются `RusToKModule`.

## 1. Общая картина

RusToK — модульный монолит: модули компилируются в общий бинарник и поднимаются через `ModuleRegistry`.

Ключевой момент: **не каждый критичный компонент платформы реализует `RusToKModule`**.
Например, `rustok-outbox` является core-инфраструктурой событий, но инициализируется через event runtime, а не через `ModuleRegistry`.

**Где смотреть в коде:**
- Runtime-регистрация модулей: `apps/server/src/modules/mod.rs`
- Контракт модуля и виды модулей: `crates/rustok-core/src/module.rs`
- Реестр Core/Optional: `crates/rustok-core/src/registry.rs`
- Манифест модулей: `modules.toml`

## 2. Что реально зарегистрировано в сервере

В текущей сборке в `ModuleRegistry` регистрируются:

### Core (`ModuleKind::Core`)

| Slug | Crate | Назначение |
| --- | --- | --- |
| `index` | `rustok-index` | CQRS/read-model индексатор |
| `tenant` | `rustok-tenant` | Tenant lifecycle и метаданные |
| `rbac` | `rustok-rbac` | RBAC lifecycle и health |

### Optional (`ModuleKind::Optional`)

| Slug | Crate | Назначение |
| --- | --- | --- |
| `content` | `rustok-content` | Базовый CMS-контент |
| `commerce` | `rustok-commerce` | e-commerce домен |
| `blog` | `rustok-blog` | Блоговая надстройка (depends_on: `content`) |
| `forum` | `rustok-forum` | Форумный модуль (depends_on: `content`) |
| `pages` | `rustok-pages` | Страницы и меню |

## 3. Критичная инфраструктура вне ModuleRegistry

Эти crate'ы не являются `RusToKModule`, но относятся к ядру платформы:

| Crate | Статус | Примечание |
| --- | --- | --- |
| `rustok-core` | Core | Контракты, базовые типы и инфраструктура |
| `rustok-outbox` | Core | Транзакционная доставка событий (required в `modules.toml`) |
| `rustok-telemetry` | Core infra | Сквозная observability |

Также есть опциональные/технические инфраструктурные crate'ы (`rustok-iggy`, `rustok-iggy-connector`, `rustok-mcp`, `alloy-scripting`).

## 4. Приложения

- `apps/server` (`rustok-server`) — API-сервер и orchestration модулей.
- `apps/admin` (`rustok-admin`) — админ-панель на Leptos.
- `apps/storefront` (`rustok-storefront`) — storefront на Leptos.
- `crates/rustok-mcp` (bin `rustok-mcp-server`) — MCP сервер/адаптер.

## 5. Связанные документы

- `docs/modules/registry.md` — реестр приложений и crate'ов.
- `docs/modules/manifest.md` — манифест и правила описания модулей.
- `docs/architecture/improvement-recommendations.md` — рекомендации и roadmap архитектуры.

## 6. Что делать при изменениях модульного состава

При добавлении/удалении модульных crate'ов или их регистрации в сервере:
1. Обновить `apps/server/src/modules/mod.rs` (если меняется runtime-регистрация).
2. Обновить `modules.toml` (required/depends_on/default_enabled).
3. Обновить `docs/modules/overview.md`, `docs/modules/registry.md` и при необходимости `docs/index.md`.
