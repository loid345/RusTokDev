# Установка модулей с UI пакетами админок и фронтендов

**Версия:** 1.0  
**Статус:** Production Ready  
**Дата:** 16 февраля 2026

---

## Обзор

Этот документ описывает новый подход к установке модулей RusToK, который включает автоматическую установку UI пакетов для админки и фронтенда. Система основана на манифесте `modules.toml` и механизме пересборки в стиле WordPress/NodeBB.

### Ключевая концепция

**Один модуль → Три артефакта:**
1. **Backend crate** — доменная логика и API (в `crates/`)
2. **Admin UI пакет** — компоненты админки (в `crates/` или `apps/admin/src/modules/`)
3. **Storefront UI пакет** — компоненты витрины (в `crates/` или `apps/storefront/src/modules/`)

При установке модуля через манифест, все три артефакта собираются и деплоятся вместе в рамках одного релиза.

---

## Архитектура системы

### Диаграмма потока установки

```text
┌────────────────────────────────────────────────────────────────────┐
│  1. Админка: Install/Uninstall модуля                              │
└─────────────────────────┬──────────────────────────────────────────┘
                          │
                          ▼
┌────────────────────────────────────────────────────────────────────┐
│  2. Обновление modules.toml                                        │
│     ┌─────────────────────────────────────────────────────────┐   │
│     │ [modules]                                              │   │
│     │ commerce = { crate = "rustok-commerce",                │   │
│     │            source = "path",                            │   │
│     │            path = "crates/rustok-commerce",            │   │
│     │            admin_ui = "leptos-commerce-admin",         │   │  ← UI пакеты
│     │            storefront_ui = "leptos-commerce-storefront" }│   │
│     └─────────────────────────────────────────────────────────┘   │
└─────────────────────────┬──────────────────────────────────────────┘
                          │
                          ▼
┌────────────────────────────────────────────────────────────────────┐
│  3. BuildService: request_build()                                  │
│     - Создаёт запись в builds таблице                             │
│     - Вычисляет manifest_hash                                      │
└─────────────────────────┬──────────────────────────────────────────┘
                          │
                          ▼
┌────────────────────────────────────────────────────────────────────┐
│  4. Build Runner (асинхронный воркер)                             │
│     ┌─────────────────────────────────────────────────────────┐   │
│     │ Стадия 1: Generate Registry                             │   │
│     │   cargo xtask generate-registry                        │   │
│     ├─────────────────────────────────────────────────────────┤   │
│     │ Стадия 2: Build Backend                                │   │
│     │   cargo build -p rustok-server --release               │   │
│     ├─────────────────────────────────────────────────────────┤   │
│     │ Стадия 3: Build Admin UI                               │   │
│     │   cd apps/admin && trunk build --release              │   │
│     ├─────────────────────────────────────────────────────────┤   │
│     │ Стадия 4: Build Storefront UI                          │   │
│     │   cd apps/storefront && trunk build --release          │   │
│     ├─────────────────────────────────────────────────────────┤   │
│     │ Стадия 5: Create Docker Images / Bundles               │   │
│     │   docker build -t rustok-server:latest .               │   │
│     │   docker build -t rustok-admin:latest .                │   │
│     │   docker build -t rustok-storefront:latest .           │   │
│     └─────────────────────────────────────────────────────────┘   │
└─────────────────────────┬──────────────────────────────────────────┘
                          │
                          ▼
┌────────────────────────────────────────────────────────────────────┐
│  5. Create Release                                                 │
│     - Создаёт запись в releases таблице                            │
│     - Связывает build_id с release_id                              │
│     - Сохраняет список всех артефактов                            │
└─────────────────────────┬──────────────────────────────────────────┘
                          │
                          ▼
┌────────────────────────────────────────────────────────────────────┐
│  6. Deploy (monolith или headless)                                │
│     - Обновляет контейнеры/сервисы                                │
│     - Запускает smoke-тесты                                       │
│     - Активирует новый release                                    │
└────────────────────────────────────────────────────────────────────┘
```

---

## Расширенный формат манифеста modules.toml

### Базовый формат

```toml
schema = 1
app = "rustok-server"

[build]
target = "x86_64-unknown-linux-gnu"
profile = "release"
deployment_profile = "monolith" # monolith | headless

[modules]
# Модуль без UI пакетов
content = { 
    crate = "rustok-content", 
    source = "path", 
    path = "crates/rustok-content" 
}

# Модуль с UI пакетами
commerce = { 
    crate = "rustok-commerce", 
    source = "path", 
    path = "crates/rustok-commerce",
    # UI пакеты (опционально)
    admin_ui = "leptos-commerce-admin",
    storefront_ui = "leptos-commerce-storefront",
    admin_ui_path = "crates/leptos-commerce-admin",
    storefront_ui_path = "crates/leptos-commerce-storefront",
    # Версионирование UI
    admin_ui_version = "0.1.0",
    storefront_ui_version = "0.1.0"
}

[settings]
default_enabled = ["content", "commerce", "pages"]
```

### Описание полей UI пакетов

| Поле | Тип | Обязательное | Описание |
|------|------|--------------|----------|
| `admin_ui` | string | Нет | Имя crate админ-UI пакета |
| `storefront_ui` | string | Нет | Имя crate UI пакета витрины |
| `admin_ui_path` | string | Нет | Путь к админ-UI пакету (если source = "path") |
| `storefront_ui_path` | string | Нет | Путь к UI пакету витрины (если source = "path") |
| `admin_ui_version` | string | Нет | Версия админ-UI пакета (для crates-io или git) |
| `storefront_ui_version` | string | Нет | Версия UI пакета витрины (для crates-io или git) |

---

## Типы UI пакетов

### 1. Admin UI Пакеты

Админ-UI пакеты содержат компоненты для управления модулем через админку:

```
crates/leptos-commerce-admin/
├── src/
│   ├── components/          # UI компоненты
│   │   ├── ProductList.rs
│   │   ├── ProductForm.rs
│   │   └── OrderTable.rs
│   ├── pages/              # Страницы админки
│   │   ├── ProductsPage.rs
│   │   └── OrdersPage.rs
│   ├── graphql/            # GraphQL запросы
│   │   ├── queries.rs
│   │   └── mutations.rs
│   └── lib.rs
├── Cargo.toml
└── README.md
```

**Зависимости типичного admin-UI пакета:**

```toml
[dependencies]
leptos = { workspace = true }
leptos-ui = { workspace = true }
leptos-forms = { workspace = true }
leptos-graphql = { workspace = true }
leptos-router = { workspace = true }
serde = { workspace = true }
```

### 2. Storefront UI Пакеты

UI пакеты витрины содержат компоненты для отображения данных модуля в магазине:

```
crates/leptos-commerce-storefront/
├── src/
│   ├── components/          # UI компоненты витрины
│   │   ├── ProductCard.rs
│   │   ├── ProductGrid.rs
│   │   └── CartWidget.rs
│   ├── pages/              # Страницы витрины
│   │   ├── ProductPage.rs
│   │   └── CartPage.rs
│   ├── graphql/            # GraphQL запросы
│   │   └── queries.rs
│   └── lib.rs
├── Cargo.toml
└── README.md
```

**Зависимости типичного storefront-UI пакета:**

```toml
[dependencies]
leptos = { workspace = true }
leptos-ui = { workspace = true }
leptos-router = { workspace = true }
leptos-graphql = { workspace = true }
serde = { workspace = true }
```

### 3. Интегрированные UI (в приложениях)

Вместо отдельных пакетов, UI компоненты могут быть размещены непосредственно в приложениях:

```
apps/admin/src/modules/
├── commerce/
│   ├── components/
│   ├── pages/
│   └── graphql/
├── blog/
│   ├── components/
│   ├── pages/
│   └── graphql/
```

Этот подход подходит для простых модулей без сложного UI.

---

## Процесс создания модуля с UI пакетами

### Шаг 1: Создание backend модуля

```bash
# Создаём новый crate модуля
cargo new --lib crates/rustok-mymodule
```

**crates/rustok-mymodule/Cargo.toml:**

```toml
[package]
name = "rustok-mymodule"
version.workspace = true
edition.workspace = true

[dependencies]
rustok-core = { workspace = true }
sea-orm = { workspace = true }
serde = { workspace = true }
```

**crates/rustok-mymodule/src/lib.rs:**

```rust
use rustok_core::module::RusToKModule;

pub struct MyModule;

#[async_trait::async_trait]
impl RusToKModule for MyModule {
    fn slug(&self) -> &'static str { "mymodule" }
    fn name(&self) -> &'static str { "My Module" }
    fn description(&self) -> &'static str { "Example module with UI" }
    fn version(&self) -> &'static str { "0.1.0" }
}
```

### Шаг 2: Создание Admin UI пакета

```bash
# Создаём crate для админ-UI
cargo new --lib crates/leptos-mymodule-admin
```

**crates/leptos-mymodule-admin/Cargo.toml:**

```toml
[package]
name = "leptos-mymodule-admin"
version.workspace = true
edition.workspace = true

[dependencies]
leptos = { workspace = true }
leptos-ui = { workspace = true }
leptos-forms = { workspace = true }
leptos-graphql = { workspace = true }
leptos-router = { workspace = true }
serde = { workspace = true }
rustok-mymodule = { path = "../rustok-mymodule" }
```

**crates/leptos-mymodule-admin/src/lib.rs:**

```rust
use leptos::*;
use leptos_router::*;

pub fn MyModuleAdminPage() -> impl IntoView {
    view! {
        <div class="p-6">
            <h1 class="text-2xl font-bold mb-4">"My Module"</h1>
            <p>"Manage your module data here"</p>
        </div>
    }
}
```

### Шаг 3: Создание Storefront UI пакета (опционально)

```bash
# Создаём crate для UI витрины
cargo new --lib crates/leptos-mymodule-storefront
```

**crates/leptos-mymodule-storefront/Cargo.toml:**

```toml
[package]
name = "leptos-mymodule-storefront"
version.workspace = true
edition.workspace = true

[dependencies]
leptos = { workspace = true }
leptos-ui = { workspace = true }
leptos-router = { workspace = true }
leptos-graphql = { workspace = true }
serde = { workspace = true }
```

**crates/leptos-mymodule-storefront/src/lib.rs:**

```rust
use leptos::*;

pub fn MyModuleWidget() -> impl IntoView {
    view! {
        <div class="my-module-widget">
            <h3>"My Module Widget"</h3>
        </div>
    }
}
```

### Шаг 4: Обновление modules.toml

```toml
[modules]
mymodule = { 
    crate = "rustok-mymodule", 
    source = "path", 
    path = "crates/rustok-mymodule",
    admin_ui = "leptos-mymodule-admin",
    storefront_ui = "leptos-mymodule-storefront",
    admin_ui_path = "crates/leptos-mymodule-admin",
    storefront_ui_path = "crates/leptos-mymodule-storefront"
}
```

### Шаг 5: Обновление Cargo.toml приложений

**apps/admin/Cargo.toml:**

```toml
[dependencies]
# ... существующие зависимости
leptos-mymodule-admin = { path = "../../crates/leptos-mymodule-admin", optional = true }

[features]
default = ["mymodule"]
mymodule = ["leptos-mymodule-admin"]
```

**apps/storefront/Cargo.toml:**

```toml
[dependencies]
# ... существующие зависимости
leptos-mymodule-storefront = { path = "../../crates/leptos-mymodule-storefront", optional = true }

[features]
default = ["mymodule"]
mymodule = ["leptos-mymodule-storefront"]
```

### Шаг 6: Интеграция в Admin приложении

**apps/admin/src/app_new.rs:**

```rust
use leptos::*;
use leptos_router::*;

#[cfg(feature = "mymodule")]
use leptos_mymodule_admin::MyModuleAdminPage;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main class="min-h-screen bg-gray-50">
                <Routes>
                    // ... существующие маршруты
                    #[cfg(feature = "mymodule")]
                    <Route path="/mymodule" view=MyModuleAdminPage />
                </Routes>
            </main>
        </Router>
    }
}
```

### Шаг 7: Интеграция в Storefront приложении (опционально)

**apps/storefront/src/app.rs:**

```rust
use leptos::*;

#[cfg(feature = "mymodule")]
use leptos_mymodule_storefront::MyModuleWidget;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>
            // ... существующий UI
            #[cfg(feature = "mymodule")]
            <MyModuleWidget />
        </div>
    }
}
```

### Шаг 8: Генерация регистрации модуля

```bash
# Генерируем код регистрации из манифеста
cargo xtask generate-registry
```

Эта команда создаст или обновит `apps/server/src/modules/generated.rs` с регистрацией нового модуля.

### Шаг 9: Сборка и тестирование

```bash
# Сборка с новым модулем
cargo build --release --features mymodule

# Запуск сервера
cargo run -p rustok-server --release

# Запуск админки
cd apps/admin && trunk serve --open

# Запуск витрины (опционально)
cd apps/storefront && trunk serve --open
```

---

## API для управления установкой модулей

### POST /admin/builds — Создать сборку

Запускает процесс установки/удаления модулей.

```json
{
  "manifest_ref": "main",
  "requested_by": "admin@rustok",
  "reason": "install module: commerce with UI packages",
  "modules": {
    "commerce": {
      "source": "path",
      "crate": "rustok-commerce",
      "path": "crates/rustok-commerce",
      "admin_ui": "leptos-commerce-admin",
      "storefront_ui": "leptos-commerce-storefront",
      "admin_ui_path": "crates/leptos-commerce-admin",
      "storefront_ui_path": "crates/leptos-commerce-storefront"
    }
  }
}
```

**Ответ:**

```json
{
  "build_id": "bld_01HABC123DEF456",
  "status": "queued",
  "stage": "pending",
  "progress": 0,
  "manifest_hash": "a1b2c3d4...",
  "created_at": "2026-02-16T12:00:00Z"
}
```

### GET /admin/builds/{build_id} — Статус сборки

```json
{
  "build_id": "bld_01HABC123DEF456",
  "status": "running",
  "stage": "build_admin_ui",
  "progress": 65,
  "steps": [
    { "name": "generate_registry", "status": "completed", "duration_ms": 1200 },
    { "name": "build_backend", "status": "completed", "duration_ms": 45000 },
    { "name": "build_admin_ui", "status": "running", "progress": 65 },
    { "name": "build_storefront_ui", "status": "pending", "progress": 0 },
    { "name": "create_docker_images", "status": "pending", "progress": 0 }
  ],
  "logs_url": "https://builds.rustok.com/bld_01HABC123DEF456/logs"
}
```

### POST /admin/builds/{build_id}/deploy — Деплой релиза

```json
{
  "environment": "production"
}
```

**Ответ:**

```json
{
  "release_id": "rel_20260216_120000",
  "build_id": "bld_01HABC123DEF456",
  "status": "deploying",
  "artifacts": [
    {
      "name": "rustok-server",
      "type": "docker_image",
      "url": "rustok-server:rel_20260216_120000"
    },
    {
      "name": "rustok-admin",
      "type": "static_bundle",
      "url": "s3://rustok-releases/admin/rel_20260216_120000.tar.gz"
    },
    {
      "name": "rustok-storefront",
      "type": "docker_image",
      "url": "rustok-storefront:rel_20260216_120000"
    }
  ]
}
```

### POST /admin/builds/{build_id}/rollback — Откат

```json
{
  "target_release": "rel_20260215_180000",
  "reason": "Commerce module UI causing issues"
}
```

---

## Режимы деплоя

### Monolith режим (по умолчанию)

Все три артефакта (server, admin, storefront) деплоятся как единый релиз.

**Преимущества:**
- Простота эксплуатации
- Гарантированная совместимость версий
- Единый процесс отката

**Процесс:**
1. Build Runner собирает все артефакты
2. Создаётся один release-id
3. Все артефакты деплоятся атомарно
4. Откат возвращает все компоненты к предыдущему release

### Headless режим

Backend и UI деплоятся раздельно.

**Преимущества:**
- Независимые релизы backend и UI
- Более частые обновления UI без пересборки backend
- Масштабирование по отдельности

**Требования:**
- Контракт совместимости версий
- Preflight проверка перед деплоем UI
- Отдельные rollback политики

**Процесс:**
1. Backend деплоится первым
2. UI деплоятся после успешного backend
3. Preflight проверяет совместимость
4. При несовместимости UI остаётся на старой версии

---

## Жизненный цикл UI пакетов

### 1. Разработка

```bash
# Создаём структуру UI пакетов
crates/leptos-mymodule-admin/
crates/leptos-mymodule-storefront/

# Разрабатываем компоненты
# Добавляем зависимости в манифест

cargo xtask validate-manifest
```

### 2. Установка через админку

```bash
# Админка → Модули → Установить
# Выбираем модуль → Нажимаем Install
# Статус: queued → running → success
```

### 3. Сборка и деплой

```bash
# Автоматически запускается Build Runner
# Стадии: generate-registry → build-backend → build-admin → build-storefront → deploy

# Проверка статуса через API
curl GET /admin/builds/{build_id}
```

### 4. Активация

```bash
# После успешного деплоя модуль активируется
# UI компоненты появляются в админке и витрине
# GraphQL схема обновляется
```

### 5. Удаление (опционально)

```bash
# Админка → Модули → Удалить
# Build Runner собирает без модуля
# UI компоненты удаляются из админки и витрины
# Откат на предыдущий релиз
```

---

## Локальная разработка с UI пакетами

### Быстрый старт

```bash
# 1. Клонируем репозиторий
git clone https://github.com/RustokCMS/RusToK.git
cd RusToK

# 2. Добавляем модуль в modules.toml
# (см. раздел "Обновление modules.toml")

# 3. Генерируем регистрацию модулей
cargo xtask generate-registry

# 4. Собираем проект
cargo build --release --features mymodule

# 5. Запускаем сервер
cargo run -p rustok-server --release

# 6. Запускаем админку (в другом терминале)
cd apps/admin
trunk serve --open

# 7. Запускаем витрину (в другом терминале)
cd ../storefront
trunk serve --port 3000
```

### Горячая перезагрузка (Hot Reload)

Для разработки без полной пересборки:

```bash
# Сервер (watch mode)
cargo watch -x 'run -p rustok-server'

# Админка (Trunk с hot reload)
cd apps/admin
trunk serve

# Витрина (Trunk с hot reload)
cd apps/storefront
trunk serve --port 3000
```

### Тестирование UI компонентов

```bash
# Unit тесты для UI пакетов
cd crates/leptos-mymodule-admin
cargo test

# E2E тесты с Playwright
cd apps/admin
cargo test --test e2e -- --features mymodule
```

---

## Best Practices

### 1. Структура UI пакетов

✅ **Рекомендуется:**
- Разделять admin и storefront UI в отдельные crates
- Использовать общие компоненты из `leptos-ui`
- Следовать DSD (Design System Driven) подходу
- Добавлять документацию в README.md

❌ **Не рекомендуется:**
- Смешивать admin и storefront UI в одном пакете
- Дублировать код между UI пакетами
- Игнорировать TypeScript/JSDoc для документации

### 2. Версионирование

- Backend модуль и UI пакеты должны иметь совместимые версии
- Используйте SemVer: `MAJOR.MINOR.PATCH`
- При breaking changes увеличивайте MAJOR
- Документируйте изменения в CHANGELOG.md

### 3. GraphQL контракты

- Определяйте GraphQL схему в backend модуле
- Генерируйте типы для UI пакетов
- Используйте `graphql_client` для типизированных запросов

### 4. Зависимости

- Минимизируйте transitive зависимости UI пакетов
- Используйте workspace зависимости
- Избегайте прямых зависимостей от специфичных библиотек

### 5. Тестирование

- Пишите unit тесты для UI компонентов
- Используйте snapshot тесты для визуальной регрессии
- Добавляйте интеграционные тесты для API

---

## Troubleshooting

### Проблема: UI пакет не компилируется

**Решение:**
```bash
# Проверьте зависимости
cd crates/leptos-mymodule-admin
cargo check

# Проверьте совместимость версий
cargo tree -d
```

### Проблема: Модуль установлен, но UI не появляется

**Решение:**
1. Проверьте включён ли feature в приложении
2. Проверьте регистрацию маршрутов в Router
3. Очистите кэш Trunk: `trunk clean && trunk serve`

### Проблема: Сбой при сборке

**Решение:**
```bash
# Проверьте статус сборки
curl GET /admin/builds/{build_id}

# Посмотрите логи
curl GET /admin/builds/{build_id}/logs

# Запустите повторно
curl POST /admin/builds/{build_id}/retry
```

### Проблема: Несовместимость версий backend и UI

**Решение:**
1. Проверьте версии в Cargo.toml
2. Убедитесь в совместимости GraphQL контрактов
3. При необходимости откатитесь на предыдущий релиз

---

## Примеры готовых модулей с UI

### 1. Commerce Module

**Backend:** `crates/rustok-commerce/`  
**Admin UI:** `crates/leptos-commerce-admin/`  
**Storefront UI:** `crates/leptos-commerce-storefront/`

**Функционал:**
- Управление товарами, заказами, ценами
- Админка: CRUD операции, фильтрация, экспорт
- Витрина: карточки товаров, корзина, чекаут

### 2. Blog Module

**Backend:** `crates/rustok-blog/`  
**Admin UI:** `crates/leptos-blog-admin/`  
**Storefront UI:** `crates/leptos-blog-storefront/`

**Функционал:**
- Посты, страницы, комментарии
- Админка: редактор Markdown, медиа-менеджер
- Витрина: лента постов, комментарии, категории

### 3. Forum Module

**Backend:** `crates/rustok-forum/`  
**Admin UI:** Интегрирован в `apps/admin/src/modules/forum/`  
**Storefront UI:** `crates/leptos-forum-storefront/`

**Функционал:**
- Форумы, темы, ответы
- Админка: модерация, управление категориями
- Витрина: просмотр тем, ответы, поиск

---

## Дополнительные ресурсы

### Основная документация
- [module-manifest.md](module-manifest.md) — Спецификация манифеста модулей
- [INSTALLATION_IMPLEMENTATION.md](INSTALLATION_IMPLEMENTATION.md) — Реализация системы установки
- [module-registry.md](module-registry.md) — Реестр модулей и lifecycle

### UI документация
- [../../crates/leptos-ui/README.md](../../crates/leptos-ui/README.md) — UI компоненты
- [../../crates/leptos-forms/README.md](../../crates/leptos-forms/README.md) — Формы и валидация
- [../../crates/leptos-graphql/README.md](../../crates/leptos-graphql/README.md) — GraphQL интеграция
- [../../docs/UI/README.md](../../docs/UI/README.md) — Админка UI документация

### Архитектура
- [ARCHITECTURE_GUIDE.md](../ARCHITECTURE_GUIDE.md) — Общая архитектура
- [MODULE_MATRIX.md](MODULE_MATRIX.md) — Матрица модулей
- [RUSTOK_MANIFEST.md](../../RUSTOK_MANIFEST.md) — Манифест платформы

---

## Заключение

Новый способ установки модулей с UI пакетами обеспечивает:

✅ **Единый процесс установки** — backend + admin UI + storefront UI  
✅ **Воспроизводимость** — манифест фиксирует состав модулей  
✅ **Безопасность** — отсутствие runtime-подгрузки нативного кода  
✅ **Гибкость** — поддержка monolith и headless режимов  
✅ **Developer Experience** — hot reload, type-safety, единый toolchain  

Эта система позволяет разработчикам сосредоточиться на создании модулей с богатым UI, не беспокоясь о сложностях деплоя и управления зависимостями.

---

**Версия документа:** 1.0  
**Последнее обновление:** 16 февраля 2026  
**Статус:** Production Ready
