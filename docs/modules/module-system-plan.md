# RusTok — Система модулей: маркетплейс и жизненный цикл

> **Статус**: Действующий план
> **Дата**: 2026-03-17
> **Заменяет**: `marketplace-plan.md` (RFC) + `module-rebuild-plan.md` (delivery)
>
> Связанные документы:
> - `docs/modules/manifest.md` — структура `modules.toml`
> - `docs/concepts/plan-oauth2-app-connections.md` — OAuth2 для marketplace auth
> - `docs/architecture/modules.md` — общая архитектура модульной системы

---

## Содержание

1. [Философия](#1-философия)
2. [Ключевые изменения в архитектуре](#2-ключевые-изменения-в-архитектуре)
3. [Стандарт модуля](#3-стандарт-модуля)
4. [Источники модулей](#4-источники-модулей)
5. [Два уровня операций](#5-два-уровня-операций)
6. [Tenant-level toggle](#6-tenant-level-toggle)
7. [Platform-level install/uninstall](#7-platform-level-installuninstall)
8. [Marketplace каталог](#8-marketplace-каталог)
9. [GraphQL API](#9-graphql-api)
10. [Admin UI](#10-admin-ui)
11. [Внешний реестр и публикация](#11-внешний-реестр-и-публикация)
12. [Статус реализации](#12-статус-реализации)

---

## 1. Философия

Для администратора управление модулями выглядит **как в WordPress**: одна страница,
кнопка "Включить/Отключить", кнопка "Установить/Удалить".

```
┌─────────────────────────────────────────────────────┐
│  WordPress        │  RusTok                         │
├───────────────────┼─────────────────────────────────┤
│  Включить         │  Toggle switch → мгновенно      │
│  Отключить        │  Toggle switch → мгновенно      │
│  Установить       │  Install → пересборка (2-5 мин) │
│  Удалить          │  Uninstall → пересборка (2-5 мин)│
│  Маркетплейс      │  Каталог → provider chain        │
└───────────────────┴─────────────────────────────────┘
```

**Ключевое отличие от WordPress**: модуль — это не интерпретируемый PHP/JS, а
компилируемый Rust crate, линкуемый в бинарник. Установка = пересборка платформы.
Это даёт compile-time безопасность и нативную производительность в обмен на
2-5 минут ожидания при изменении состава модулей.

**Два уровня операций — один UX**:

| Уровень | Действие | Время | UX |
|---|---|---|---|
| **Tenant-level** | Включить/отключить для тенанта | Мгновенно | Toggle switch |
| **Platform-level** | Установить/удалить из бинарника | 2-5 мин (сборка) | Кнопка + прогресс-бар |

---

## 2. Ключевые изменения в архитектуре

### 2.1. Миграции распределены по crate-модулям

> Коммит: `9b187ae refactor(migrations): distribute module migrations to their respective crates`

Ранее все миграции лежали в `apps/server/migration/`. Теперь каждый модуль
несёт свои миграции **внутри собственного crate**:

```
crates/rustok-blog/src/migrations/
├── mod.rs                        # impl MigrationSource
├── m20250101_000001_create_posts.rs
└── m20250201_000002_add_tags_to_posts.rs
```

**Влияние на install/uninstall**:
- При добавлении нового модуля в бинарник его `migrations()` регистрируются
  в `ModuleRegistry::migrations()` и прогоняются автоматически при старте.
- При удалении модуля схема **не дропается автоматически** — требуется явный
  `down()` через `uninstall_module` с флагом `drop_schema: true`.
- Третьесторонние модули из маркетплейса получают ту же механику:
  publish → `.crate` → cargo dep → `MigrationSource` → автопрогон.

### 2.2. CategoryService/TagService перенесены в rustok-content

> Коммит: `838dffd feat(taxonomy): move CategoryService and TagService to rustok-content`

Таксономическая инфраструктура (категории, теги) теперь живёт в модуле `content`.
Любой модуль, которому нужны категории или теги, объявляет `depends_on = ["content"]`
и получает `CategoryService`/`TagService` из контекста.

**Влияние на маркетплейс**:
- Модуль `blog` → `depends_on = ["content"]` → переиспользует категории контента.
- Сторонний модуль `podcast` → то же самое, без переопределения taxonomy-слоя.
- При проверке совместимости registry обязан валидировать граф зависимостей:
  если `blog@1.2` требует `content >= 1.0`, а установлен `content@0.9` — конфликт.

```
ModuleRegistry::dependencies_satisfied(slug, enabled_modules) → bool
```

---

## 3. Стандарт модуля

### 3.1. Манифест `rustok-module.toml`

Каждый модуль (встроенный, маркетплейс или приватный) **обязан** содержать
манифест `rustok-module.toml` в корне crate.

```toml
# ── Идентификация ───────────────────────────────────
[module]
slug = "blog"
name = "Blog"
version = "1.2.0"
description = "Blogging engine with categories, tags, and SEO"
authors = ["RusTok Team <team@rustok.dev>"]
license = "MIT"
repository = "https://github.com/RustokCMS/rustok-blog"

# ── Каталогизация (маркетплейс) ──────────────────────
[marketplace]
icon = "assets/icon.svg"
banner = "assets/banner.png"
screenshots = ["assets/screenshot-editor.png"]
category = "content"
tags = ["blog", "cms", "seo", "markdown"]

# ── Совместимость ────────────────────────────────────
[compatibility]
rustok_min = "0.5.0"
rustok_max = "1.x"

# ── Зависимости ──────────────────────────────────────
[dependencies]
content = ">= 1.0.0"    # Переиспользует CategoryService/TagService

[conflicts]
legacy-blog = "*"

# ── Rust crate ───────────────────────────────────────
[crate]
name = "rustok-blog"
entry_type = "BlogModule"

# ── Что предоставляет ────────────────────────────────
[provides]
migrations = true       # Миграции живут в crate (src/migrations/)
permissions = ["blog:read", "blog:write", "blog:publish"]
events_emitted = ["blog.post.created", "blog.post.published"]
events_consumed = ["content.item.updated"]

[[provides.admin_nav]]
label_key = "blog.nav.posts"
href = "/posts"
icon = "pencil"
section = "content"

[[provides.storefront_slots]]
id = "blog-latest-posts"
slot = "HomeAfterHero"
order = 20

[provides.graphql]
types = ["Post"]
queries = ["posts", "post"]
mutations = ["createPost", "updatePost", "publishPost"]

# ── Настройки ────────────────────────────────────────
[settings.posts_per_page]
type = "integer"
default = 10
min = 1
max = 100

[settings.enable_comments]
type = "boolean"
default = true

# ── Локализация ──────────────────────────────────────
[locales]
supported = ["en", "ru"]
default = "en"
```

### 3.2. Структура файлов модуля

```
rustok-blog/
├── rustok-module.toml          # Обязательно
├── Cargo.toml
├── src/
│   ├── lib.rs                  # pub struct BlogModule; impl RusToKModule
│   ├── entities/
│   ├── migrations/             # ← Теперь здесь (migration distribution)
│   │   ├── mod.rs              # impl MigrationSource
│   │   ├── m20250101_create_posts.rs
│   │   └── m20250201_add_slug_index.rs
│   ├── graphql/
│   ├── services/
│   └── events/
├── admin/                      # Leptos-компоненты для FSD слоёв
├── storefront/                 # Виджеты для slot-системы
├── locales/
│   ├── en.json
│   └── ru.json
└── assets/                     # Иконка, скриншоты (для маркетплейса)
```

### 3.3. Минимальный контракт

| Требование | Проверяется |
|---|---|
| `rustok-module.toml` присутствует | Валидатором при publish |
| `impl RusToKModule` | Компилятором |
| `impl MigrationSource` | Компилятором |
| `slug` уникален в реестре | Реестром маркетплейса |
| `version` — semver | Валидатором |
| `locales/en.json` | Валидатором |
| Зависимости удовлетворены | `ModuleRegistry::dependencies_satisfied()` |

---

## 4. Источники модулей

```
┌─────────────────────────────────────────────────────────┐
│                     Module Sources                       │
│                                                          │
│  ┌──────────┐   ┌──────────────┐   ┌─────────────────┐  │
│  │ Built-in │   │  Marketplace │   │ Private / Git   │  │
│  │  (path)  │   │  (registry)  │   │ (git / path)    │  │
│  └─────┬────┘   └──────┬───────┘   └───────┬─────────┘  │
│        │               │                    │             │
│        └───────────────┴────────────────────┘             │
│                        ▼                                  │
│            ┌───────────────────────┐                     │
│            │      modules.toml     │                     │
│            └───────────────────────┘                     │
└─────────────────────────────────────────────────────────┘
```

### 4.1. Built-in (path) — Встроенные модули

Модули из монорепы. Версия = версия платформы:

```toml
content = { crate = "rustok-content", source = "path", path = "crates/rustok-content" }
blog    = { crate = "rustok-blog",    source = "path", path = "crates/rustok-blog",
            depends_on = ["content"] }
```

### 4.2. Marketplace (registry) — Публичный реестр

```toml
seo-tools = { crate = "rustok-seo", source = "registry", version = "^1.2.0" }
analytics = { crate = "rustok-analytics", source = "registry", version = "~2.0" }
```

Скачиваются с `modules.rustok.dev` (внешний реестр, см. [секцию 11](#11-внешний-реестр-и-публикация)).

### 4.3. Private / Git — Приватные модули

```toml
internal-crm = { crate = "our-crm", source = "git",
                 git = "git@github.com:our-org/crm.git", rev = "abc123" }
```

---

## 5. Два уровня операций

```
┌──────────────────────────────────────────────────────────────────┐
│                                                                  │
│  PLATFORM-LEVEL (операции с бинарником)                         │
│  ─────────────────────────────────────────────────              │
│  installModule   → modules.toml + cargo dep → rebuild → deploy  │
│  uninstallModule → modules.toml - cargo dep → rebuild → deploy  │
│  upgradeModule   → modules.toml ver bump   → rebuild → deploy   │
│  rollbackBuild   → предыдущий release image → deploy            │
│                                                                  │
│  TENANT-LEVEL (операции с runtime-состоянием)                   │
│  ─────────────────────────────────────────────────              │
│  toggleModule(enabled=true)  → tenant_modules → on_enable()     │
│  toggleModule(enabled=false) → tenant_modules → on_disable()    │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

**Инвариант**: Tenant-level toggle работает только над **установленными** модулями.
Установленность = модуль скомпилирован в бинарник (присутствует в `ModuleRegistry`).

---

## 6. Tenant-level toggle

### 6.1. Архитектура (реализовано ✅)

**Файлы**:
- `apps/server/src/services/module_lifecycle.rs` — `ModuleLifecycleService`
- `crates/rustok-tenant/src/entities/tenant_module.rs` — SeaORM entity
- `apps/server/migration/src/m20250101_000003_create_tenant_modules.rs`

**Flow**:

```
Admin UI
  └─ GraphQL toggleModule(moduleSlug, enabled)
       └─ ModuleLifecycleService::toggle_module(db, registry, tenant_id, slug, enabled)
            ├─ Проверить: slug ∈ ModuleRegistry        (UnknownModule)
            ├─ Проверить: не Core модуль               (CoreModuleCannotBeDisabled)
            ├─ enabled=true:  все depends_on включены   (MissingDependencies)
            ├─ enabled=false: нет зависящих от него     (HasDependents)
            ├─ BEGIN TRANSACTION
            │    UPDATE tenant_modules SET enabled=?
            │    CALL module.on_enable() / on_disable()
            │    COMMIT  ─── или ─── ROLLBACK при HookFailed
            └─ Return TenantModule { module_slug, enabled, settings }
```

### 6.2. Что нужно доработать (Tenant-level) ⬜

**[ ] `EnabledModulesProvider` контекст в Leptos**

```rust
// apps/admin-leptos/src/providers/enabled_modules.rs
#[component]
pub fn EnabledModulesProvider(children: Children) -> impl IntoView {
    let enabled = create_resource(|| (), |_| async { fetch_enabled_modules().await });
    provide_context(enabled);
    children()
}

pub fn use_enabled_modules() -> Resource<(), Vec<String>> {
    use_context::<Resource<(), Vec<String>>>().expect("EnabledModulesProvider missing")
}
```

- Загружает `enabledModules` query при старте приложения.
- Sidebar фильтрует nav items по включённым модулям.
- Используется во всех feature-флагах UI.

**[ ] `<ModuleGuard>` — обёртка для маршрутов**

```rust
// apps/admin-leptos/src/components/module_guard.rs
#[component]
pub fn ModuleGuard(slug: &'static str, children: Children) -> impl IntoView {
    let enabled_modules = use_enabled_modules();
    move || {
        if enabled_modules.get().map(|m| m.contains(&slug.to_string())).unwrap_or(false) {
            children()
        } else {
            view! { <ModuleDisabledPlaceholder slug=slug /> }
        }
    }
}
```

**[ ] Фильтрация slot-компонентов**

`components_for_slot(slot_id, enabled_modules)` — фильтрует виджеты витрины
по `enabled_modules` тенанта перед рендером.

---

## 7. Platform-level install/uninstall

### 7.1. Архитектура (реализовано ✅)

**Файлы**:
- `apps/server/src/modules/manifest.rs` — `ManifestManager`
- `apps/server/src/services/build_service.rs` — `BuildService`
- `apps/server/src/services/build_executor.rs` — `BuildExecutor`
- `apps/server/src/models/build.rs` — `Build`, `BuildStatus`, `BuildStage`
- `apps/server/src/models/release.rs` — `Release`, `ReleaseStatus`
- `apps/server/migration/src/m20250212_000001_create_builds_and_releases.rs`

**UX flow** (WordPress-аналогия):

```
Admin UI ──────────────────────────────────────────────────────────────┐
                                                                        │
┌─ Каталог модулей ──────────────────────────────────────────────────┐ │
│  Blog Module v1.2.0                           [Установить]         │ │
│  Блоговый движок с категориями и тегами                            │ │
└────────────────────────────────────────────────────────────────────┘ │
                      ↓ клик [Установить]                              │
┌─ Прогресс ─────────────────────────────────────────────────────────┐ │
│  Blog Module v1.2.0                                                │ │
│  ████████████░░░░░░░░  52%  Компиляция...                          │ │
└────────────────────────────────────────────────────────────────────┘ │
                      ↓ готово                                         │
┌─ Установлен ───────────────────────────────────────────────────────┐ │
│  Blog Module v1.2.0              [●] Вкл    [Обновить] [Удалить]   │ │
│  Блоговый движок с категориями и тегами                            │ │
└────────────────────────────────────────────────────────────────────┘ │
```

**Backend pipeline**:

```
installModule(slug, version)
  └─ ManifestManager::install_builtin_module(manifest, slug, version)
       ├─ Добавить в modules.toml
       ├─ validate(manifest)          ← проверить граф зависимостей
       ├─ save(manifest)
       └─ BuildService::request_build(BuildRequest { ... })
            ├─ Хешировать modules_delta (SHA-256)
            ├─ Дедублировать: если уже есть build с таким hash — вернуть его
            ├─ INSERT INTO builds (status=queued, ...)
            └─ Publish BuildRequested event

BuildExecutor::execute_next_queued_build()
  ├─ SELECT build WHERE status=queued ORDER BY created_at LIMIT 1
  ├─ UPDATE status=running, stage=checkout
  ├─ cargo build --features=[installed_modules]
  │    ← BuildExecutor строит feature flags из ModuleRegistry
  ├─ UPDATE stage=test, progress=70%
  ├─ cargo test
  ├─ UPDATE stage=deploy, progress=90%
  ├─ docker build + push (или rolling restart для monolith)
  └─ UPDATE status=success, stage=complete, progress=100%
       └─ INSERT INTO releases (status=active, ...)
```

**Миграции при deploy** (после migration distribution):

Модули регистрируют свои миграции через `MigrationSource`. При старте нового
бинарника:

```rust
// apps/server/src/main.rs (или app.rs)
let registry = build_registry();
let migrations = registry.migrations(); // собирает из всех модулей
run_migrations(&db, migrations).await?;
```

Новые таблицы создаются автоматически — ничего вручную добавлять в
`apps/server/migration/` не нужно.

### 7.2. ManifestManager — API

```rust
impl ManifestManager {
    // Загрузка и сохранение
    pub fn load() -> Result<ModulesManifest>;
    pub fn save(manifest: &ModulesManifest) -> Result<()>;
    pub fn validate(manifest: &ModulesManifest) -> Result<()>;
    pub fn validate_with_registry(manifest: &ModulesManifest, registry: &ModuleRegistry) -> Result<()>;

    // Platform-level операции
    pub fn install_builtin_module(manifest: &mut ModulesManifest, slug: &str, version: Option<&str>) -> Result<()>;
    pub fn uninstall_module(manifest: &mut ModulesManifest, slug: &str) -> Result<()>;
    pub fn upgrade_module(manifest: &mut ModulesManifest, slug: &str, version: &str) -> Result<()>;

    // Для BuildService
    pub fn build_modules(manifest: &ModulesManifest) -> Vec<BuildModuleSpec>;
    pub fn deployment_profile(manifest: &ModulesManifest) -> DeploymentProfile;
    pub fn build_execution_plan(manifest: &ModulesManifest) -> BuildExecutionPlan;

    // Для MarketplaceCatalogService
    pub fn catalog_modules(manifest: &ModulesManifest) -> Result<Vec<CatalogManifestModule>>;
    pub fn installed_modules(manifest: &ModulesManifest) -> Vec<InstalledModuleInfo>;
}
```

### 7.3. BuildService — модели

```rust
// BuildStatus
enum BuildStatus { Queued, Running, Success, Failed, Cancelled }

// BuildStage
enum BuildStage { Pending, Checkout, Build, Test, Deploy, Complete }

// DeploymentProfile
enum DeploymentProfile { Monolith, ServerWithAdmin, ServerWithStorefront, HeadlessApi }

// ReleaseStatus
enum ReleaseStatus { Pending, Deploying, Active, RolledBack, Failed }
```

### 7.4. Что нужно доработать (Platform-level) ⬜

**[ ] `upgradeModule` мутация** — обёртка над `ManifestManager::upgrade_module()`:

```graphql
mutation {
  upgradeModule(slug: "blog", version: "1.3.0") { id status }
}
```

**[ ] `rollbackBuild` мутация** — откат к предыдущему `Release`:

```rust
// Цепочка откатов через releases.previous_release_id
pub async fn rollback_build(build_id: Uuid) -> Result<BuildJob>;
```

**[ ] Реальный deployment pipeline**:
- `docker build` + `docker push` по завершении cargo build.
- Rolling restart для монолитного режима.
- `releases` таблица: заполнение `container_image`, `server_artifact_url`.

**[ ] Build progress subscriptions в UI**:

```graphql
subscription {
  buildProgress(buildId: "...") {
    status stage progress logsUrl
  }
}
```

Leptos WebSocket / SSE клиент, обновляющий прогресс-бар в реальном времени.

---

## 8. Marketplace каталог

### 8.1. Provider chain (реализовано ✅)

**Файл**: `apps/server/src/services/marketplace_catalog.rs`

```
MarketplaceCatalogService
  ├─ LocalManifestMarketplaceProvider   (local-manifest)
  │    └─ ManifestManager::catalog_modules() → встроенные path-модули
  └─ RegistryMarketplaceProvider        (registry)
       └─ GET $RUSTOK_MARKETPLACE_REGISTRY_URL/v1/catalog
            └─ moka cache (TTL: $RUSTOK_MARKETPLACE_REGISTRY_CACHE_TTL_SECS, default 60s)
```

**Env vars**:
- `RUSTOK_MARKETPLACE_REGISTRY_URL` — URL внешнего реестра (опционально)
- `RUSTOK_MARKETPLACE_REGISTRY_TIMEOUT_MS` — таймаут (default: 3000)
- `RUSTOK_MARKETPLACE_REGISTRY_CACHE_TTL_SECS` — TTL кеша (default: 60)

**Режимы**:
- `local_only()` — только встроенные (dev/тесты)
- `evolutionary_defaults()` — local + registry с fallback на local при ошибке

**Дедупликация**: если модуль есть в нескольких провайдерах — побеждает первый.

### 8.2. Категории каталога

Фиксированный набор (расширяется только core-командой):

| Категория | Slug | Описание |
|---|---|---|
| Контент | `content` | CMS, блоги, страницы, медиа |
| E-commerce | `commerce` | Магазины, оплата, каталог |
| Аналитика | `analytics` | Метрики, дашборды, отчёты |
| Социальные | `social` | Комментарии, форумы |
| SEO | `seo` | Поисковая оптимизация |
| Интеграции | `integrations` | Внешние API, webhooks |
| Инструменты | `dev-tools` | Утилиты, миграции |
| Безопасность | `security` | 2FA, аудит |
| Локализация | `localization` | i18n, мультиязычность |
| Темы/UI | `themes` | Оформление, компоненты |

### 8.3. Фильтрация каталога (реализовано ✅)

`marketplace(...)` поддерживает фильтрацию по:
- `search` — полнотекстовый поиск по имени/описанию
- `category` — категория модуля
- `source` — `"local"` / `"registry"`
- `installed` — только установленные / только нет
- `trust_level` — `first_party` / `third_party` / `community`
- `compatible_only` — только совместимые с текущей версией платформы

---

## 9. GraphQL API

### 9.1. Реализовано ✅

```graphql
# Запросы
type Query {
  marketplace(
    search: String
    category: String
    source: String
    installed: Boolean
    trust_level: String
    compatible_only: Boolean
  ): [MarketplaceModule!]!

  marketplaceModule(slug: String!): MarketplaceModule

  modules: InstalledModules!
  moduleRegistry: [ModuleRegistryItem!]!

  activeBuild: BuildJob
}

# Мутации
type Mutation {
  toggleModule(moduleSlug: String!, enabled: Boolean!): TenantModule!
  installModule(slug: String!, version: String): BuildJob!
  uninstallModule(slug: String!): BuildJob!
}

# Типы
type TenantModule {
  moduleSlug: String!
  enabled: Boolean!
  settings: JSON!
}

type BuildJob {
  id: ID!
  status: BuildStatus!
  stage: String
  progress: Int
  logsUrl: String
  modulesDelta: String!
  startedAt: DateTime
  finishedAt: DateTime
}

type MarketplaceModule {
  slug: String!
  name: String!
  description: String!
  version: String!
  source: String!
  installed: Boolean!
  enabled: Boolean
  trustLevel: String!
  category: String!
  tags: [String!]!
  compatibility: CompatibilityInfo!
  adminSurfaces: [String!]!
  versions: [MarketplaceModuleVersion!]!
}
```

### 9.2. Нужно добавить ⬜

```graphql
type Mutation {
  # Обновить версию модуля
  upgradeModule(slug: String!, version: String!): BuildJob!

  # Откат к предыдущему release
  rollbackBuild(buildId: ID!): BuildJob!
}

type Query {
  buildHistory(limit: Int, offset: Int): [BuildJob!]!
}

type Subscription {
  # Реальтайм прогресс сборки
  buildProgress(buildId: ID!): BuildJob!
}
```

---

## 10. Admin UI

### 10.1. Страница `/modules` — состояние реализации

**Реализовано ✅**:
- Список установленных модулей (`modules` query)
- Каталог маркетплейса (`marketplace` query)
- Фильтры: поиск, категория, trust level, compatibility
- Детальная панель модуля (`marketplaceModule` query) с deep-link (`?module=slug`)
- Кнопка Install / Uninstall → `installModule` / `uninstallModule` мутации
- Toggle switch → `toggleModule` мутация
- Секции Installed / Marketplace / Updates

**Нужно доработать ⬜**:

**[ ] Build progress в UI**:
```rust
// Подписка на прогресс сборки (Leptos)
let build_progress = use_build_subscription(build_id);
view! {
  <ProgressBar value=build_progress.progress max=100 />
  <span>{build_progress.stage}</span>
}
```

**[ ] `upgradeModule` кнопка в карточке модуля**:
- Показывать, если `installed && latest_version != current_version`
- Badge "Update available" в карточке

**[ ] `EnabledModulesProvider` + `<ModuleGuard>`** — см. [секцию 6.2](#62-что-нужно-доработать-tenant-level-)

**[ ] Slot-фильтрация** — компоненты storefront скрываются если модуль отключён

---

## 11. Внешний реестр и публикация

> Этот раздел — долгосрочный roadmap. Не блокирует текущую delivery.

### 11.1. Архитектура реестра

```
modules.rustok.dev
  ├─ GraphQL API
  │    ├─ marketplace(search, category, compatible) → [Module]
  │    ├─ marketplaceModule(slug) → Module
  │    ├─ publishModule(crate, manifest) → PublishResult    # автор
  │    └─ yankVersion(slug, version) → Result              # автор
  ├─ Crate Storage (S3-совместимое)
  │    └─ rustok-blog-1.2.0.crate (tar.gz)
  └─ Validation Pipeline (CI/CD для публикации)
       ├─ Stage 1: static checks (manifest, slug unique, semver)
       ├─ Stage 2: security audit (cargo-audit, unsafe check)
       ├─ Stage 3: compilation (с rustok_min..rustok_max)
       ├─ Stage 4: runtime tests (cargo test, migrations up/down)
       └─ Stage 5: metadata quality (icon, description length)
```

### 11.2. Аутентификация (см. также OAuth2 plan)

Три auth-потока:

```
Admin UI → Server (OAuth PKCE прокси) → Marketplace (Platform API Key)
CLI автора → Marketplace (свой auth или federated)
Build Worker → Marketplace (Platform API Key, скачивание .crate)
```

Подробности: `docs/concepts/plan-oauth2-app-connections.md` Приложение A.

### 11.3. rustok CLI для авторов модулей

```bash
rustok mod init          # Создать шаблон модуля с rustok-module.toml
rustok mod validate      # Локальная проверка манифеста
rustok mod test          # Запустить validation pipeline локально
rustok mod publish       # Опубликовать в реестр (Stage 1-5 валидация)
rustok mod yank 1.2.0    # Отозвать версию (не удаляет, помечает yanked)
```

### 11.4. Схема БД реестра

```sql
-- Основные таблицы
CREATE TABLE marketplace_modules (
    slug          VARCHAR(64) PRIMARY KEY,
    name          VARCHAR(128) NOT NULL,
    description   TEXT NOT NULL,
    authors       JSONB NOT NULL,
    license       VARCHAR(32) NOT NULL,
    category      VARCHAR(32) NOT NULL,
    tags          JSONB NOT NULL DEFAULT '[]',
    icon_url      VARCHAR(512),
    featured      BOOLEAN NOT NULL DEFAULT false,
    owner_id      UUID NOT NULL,
    total_downloads BIGINT NOT NULL DEFAULT 0,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE marketplace_versions (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    module_slug   VARCHAR(64) NOT NULL REFERENCES marketplace_modules(slug),
    version       VARCHAR(32) NOT NULL,
    rustok_min    VARCHAR(32) NOT NULL,
    rustok_max    VARCHAR(32),
    changelog     TEXT,
    crate_url     VARCHAR(512) NOT NULL,
    checksum      VARCHAR(64) NOT NULL,    -- SHA-256
    manifest      JSONB NOT NULL,          -- Parsed rustok-module.toml
    yanked        BOOLEAN NOT NULL DEFAULT false,
    published_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(module_slug, version)
);

CREATE TABLE marketplace_dependencies (
    version_id    UUID NOT NULL REFERENCES marketplace_versions(id),
    depends_on    VARCHAR(64) NOT NULL,
    version_req   VARCHAR(32) NOT NULL,
    PRIMARY KEY(version_id, depends_on)
);
```

---

## 12. Статус реализации

### Platform-level (install/uninstall/build pipeline)

| Компонент | Файл | Статус |
|---|---|---|
| `modules.toml` парсер | `src/modules/manifest.rs` | ✅ |
| `ManifestManager` (install/uninstall/upgrade) | `src/modules/manifest.rs` | ✅ |
| `BuildService` (request, dedup, events) | `src/services/build_service.rs` | ✅ |
| `BuildExecutor` (cargo build, feature flags) | `src/services/build_executor.rs` | ✅ |
| `builds` + `releases` таблицы | `migration/m20250212...` | ✅ |
| `installModule` / `uninstallModule` мутации | `src/graphql/mutations.rs` | ✅ |
| Migration distribution (crate-level migrations) | `crates/*/src/migrations/` | ✅ |
| `upgradeModule` мутация | — | ⬜ |
| `rollbackBuild` мутация | — | ⬜ |
| Docker build + push в pipeline | — | ⬜ |
| `buildProgress` GraphQL subscription | — | ⬜ |
| Build progress UI (прогресс-бар) | — | ⬜ |

### Tenant-level (toggle)

| Компонент | Файл | Статус |
|---|---|---|
| `tenant_modules` таблица | `migration/m20250101_000003...` | ✅ |
| `ModuleLifecycleService::toggle_module` | `src/services/module_lifecycle.rs` | ✅ |
| `toggleModule` GraphQL мутация | `src/graphql/mutations.rs` | ✅ |
| Dependency/dependent validation | `src/services/module_lifecycle.rs` | ✅ |
| Hook rollback на ошибке | `src/services/module_lifecycle.rs` | ✅ |
| `EnabledModulesProvider` (Leptos) | — | ⬜ |
| `<ModuleGuard>` компонент | — | ⬜ |
| Slot-фильтрация по enabled modules | — | ⬜ |
| Sidebar фильтрация по enabled modules | — | ⬜ |

### Marketplace каталог

| Компонент | Файл | Статус |
|---|---|---|
| `MarketplaceCatalogService` (provider chain) | `src/services/marketplace_catalog.rs` | ✅ |
| `LocalManifestMarketplaceProvider` | `src/services/marketplace_catalog.rs` | ✅ |
| `RegistryMarketplaceProvider` (skeleton) | `src/services/marketplace_catalog.rs` | ✅ |
| `marketplace` / `marketplaceModule` queries | `src/graphql/queries.rs` | ✅ |
| Фильтрация по source/trust/category/installed | `src/graphql/queries.rs` | ✅ |
| Deep-link `?module=slug` | admin UI | ✅ |
| Внешний реестр `modules.rustok.dev` | — | ⬜ |
| Validation pipeline для publish | — | ⬜ |
| `rustok mod publish` CLI | — | ⬜ |
| OAuth2 auth для marketplace | — | ⬜ |

### Shared taxonomy (CategoryService/TagService)

| Компонент | Файл | Статус |
|---|---|---|
| `CategoryService` в rustok-content | `crates/rustok-content/src/services/category.rs` | ✅ |
| `TagService` в rustok-content | `crates/rustok-content/src/services/tag.rs` | ✅ |
| Модули используют через `depends_on = ["content"]` | `modules.toml` | ✅ |
| Dependency validation в `ModuleRegistry` | `crates/rustok-core/src/registry.rs` | ✅ |
