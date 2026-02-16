# Быстрый старт: UI пакеты модулей

Краткое руководство по созданию и установке модулей с UI пакетами для админки и фронтенда.

## Что это такое?

Каждый модуль RusToK теперь может включать три компонента:
1. **Backend** — доменная логика и API
2. **Admin UI** — компоненты для управления в админке
3. **Storefront UI** — компоненты для отображения в витрине

## Структура модуля с UI

```
rustok-mymodule/              # Backend crate
├── src/lib.rs                # Реализация RusToKModule
├── Cargo.toml

leptos-mymodule-admin/        # Admin UI пакет
├── src/components/           # Компоненты админки
├── src/pages/                # Страницы админки
├── src/graphql/              # GraphQL запросы
├── Cargo.toml

leptos-mymodule-storefront/   # Storefront UI пакет (опционально)
├── src/components/           # Компоненты витрины
├── src/pages/                # Страницы витрины
├── src/graphql/              # GraphQL запросы
├── Cargo.toml
```

## Пошаговое создание модуля с UI

### 1. Создайте backend модуль

```bash
cargo new --lib crates/rustok-mymodule
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

### 2. Создайте Admin UI пакет

```bash
cargo new --lib crates/leptos-mymodule-admin
```

**crates/leptos-mymodule-admin/src/lib.rs:**
```rust
use leptos::*;

pub fn MyModuleAdminPage() -> impl IntoView {
    view! {
        <div class="p-6">
            <h1 class="text-2xl font-bold mb-4">"My Module"</h1>
            <p>"Manage your module data here"</p>
        </div>
    }
}
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
```

### 3. Обновите modules.toml

```toml
[modules]
mymodule = { 
    crate = "rustok-mymodule", 
    source = "path", 
    path = "crates/rustok-mymodule",
    admin_ui = "leptos-mymodule-admin",
    admin_ui_path = "crates/leptos-mymodule-admin"
}
```

### 4. Добавьте в приложения

**apps/admin/Cargo.toml:**
```toml
[dependencies]
leptos-mymodule-admin = { path = "../../crates/leptos-mymodule-admin", optional = true }

[features]
default = ["mymodule"]
mymodule = ["leptos-mymodule-admin"]
```

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
            <Routes>
                // ... другие маршруты
                #[cfg(feature = "mymodule")]
                <Route path="/mymodule" view=MyModuleAdminPage />
            </Routes>
        </Router>
    }
}
```

### 5. Сгенерируйте регистрацию модулей

```bash
cargo xtask generate-registry
```

### 6. Соберите и запустите

```bash
# Сборка
cargo build --release --features mymodule

# Сервер
cargo run -p rustok-server --release

# Админка (другой терминал)
cd apps/admin && trunk serve --open
```

## Использование готовых UI компонентов

### Используйте leptos-ui компоненты

```rust
use leptos::*;
use leptos_ui::{Button, Card, Input};

pub fn MyModuleAdminPage() -> impl IntoView {
    view! {
        <Card>
            <div class="space-y-4">
                <Input 
                    label="Название"
                    placeholder="Введите название"
                />
                <Button variant="primary">
                    "Сохранить"
                </Button>
            </div>
        </Card>
    }
}
```

### Используйте leptos-forms для форм

```rust
use leptos::*;
use leptos_forms::{Form, TextField, TextAreaField};

#[derive(serde::Deserialize, Clone)]
pub struct MyFormData {
    pub title: String,
    pub description: String,
}

pub fn MyModuleForm() -> impl IntoView {
    view! {
        <Form<MyFormData>
            action="/api/mymodule/create"
            method="post"
        >
            <TextField<MyFormData>
                name="title"
                label="Название"
                required=true
            />
            <TextAreaField<MyFormData>
                name="description"
                label="Описание"
                rows=5
            />
        </Form>
    }
}
```

### Используйте leptos-graphql для API

```rust
use leptos::*;
use leptos_query::*;
use leptos_graphql::{use_query, GraphQLClient};

#[derive(serde::Deserialize)]
pub struct MyModuleData {
    pub id: String,
    pub title: String,
}

pub fn MyModuleList() -> impl IntoView {
    let client = GraphQLClient::new("/graphql");

    let query = use_query(
        move || {
            client
                .query::<Vec<MyModuleData>>("
                    query GetMyModuleItems {
                        myModuleItems {
                            id
                            title
                        }
                    }
                ")
                .execute()
        },
        ()
    );

    view! {
        <div class="space-y-2">
            <Transition fallback=move || view! { "Загрузка..." }>
                <For
                    each=move || query.data.get().unwrap_or(&vec![]).clone()
                    key=|item| item.id.clone()
                    children=move |item| {
                        view! {
                            <div class="p-4 border rounded">
                                {&item.title}
                            </div>
                        }
                    }
                />
            </Transition>
        </div>
    }
}
```

## Установка через админку

1. Откройте админку
2. Перейдите в раздел "Модули"
3. Нажмите "Установить"
4. Выберите модуль из списка
5. Нажмите "Установить"
6. Дождитесь завершения сборки
7. UI компоненты появятся в админке

## API для сборки

```bash
# Создать сборку
curl -X POST http://localhost:3000/admin/builds \
  -H "Content-Type: application/json" \
  -d '{
    "requested_by": "admin",
    "reason": "install module: mymodule",
    "modules": {
      "mymodule": {
        "crate": "rustok-mymodule",
        "source": "path",
        "path": "crates/rustok-mymodule",
        "admin_ui": "leptos-mymodule-admin"
      }
    }
  }'

# Проверить статус
curl http://localhost:3000/admin/builds/{build_id}

# Деплой
curl -X POST http://localhost:3000/admin/builds/{build_id}/deploy \
  -H "Content-Type: application/json" \
  -d '{ "environment": "production" }'
```

## Полезные команды

```bash
# Валидация манифеста
cargo xtask validate-manifest

# Генерация регистрации
cargo xtask generate-registry

# Список модулей
cargo xtask list-modules

# Проверка зависимостей
cd crates/leptos-mymodule-admin
cargo tree

# Тестирование UI компонентов
cargo test

# Сборка админки
cd apps/admin
trunk build --release
```

## Примеры готовых модулей

### Commerce Module
- Backend: `crates/rustok-commerce/`
- Admin UI: `crates/leptos-commerce-admin/`
- Storefront UI: `crates/leptos-commerce-storefront/`

### Blog Module
- Backend: `crates/rustok-blog/`
- Admin UI: `crates/leptos-blog-admin/`
- Storefront UI: `crates/leptos-blog-storefront/`

## Дополнительная документация

- [Полная документация по установке модулей с UI](MODULE_UI_PACKAGES_INSTALLATION.md)
- [Манифест модулей](module-manifest.md)
- [Реализация системы установки](INSTALLATION_IMPLEMENTATION.md)
- [UI компоненты](../../crates/leptos-ui/README.md)
- [Формы и валидация](../../crates/leptos-forms/README.md)
- [GraphQL интеграция](../../crates/leptos-graphql/README.md)

## Следующие шаги

1. ✅ Создайте backend модуль
2. ✅ Создайте Admin UI пакет
3. ✅ Обновите modules.toml
4. ✅ Интегрируйте в приложения
5. ✅ Соберите и протестируйте
6. ✅ Опубликуйте в реестр модулей

---

**Версия:** 1.0  
**Последнее обновление:** 16 февраля 2026
