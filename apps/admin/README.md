# RusToK Admin (Leptos)

`apps/admin` — Leptos CSR админка RusToK, развиваемая параллельно с `apps/next-admin` для функционального паритета.

## Роль в платформе

- управление контентом, пользователями, безопасностью и настройками;
- Rust/Leptos реализация admin-панели;
- эталон для внутреннего UI и crate-контрактов в Rust фронтенде.

## FSD структура

Приложение следует FSD-слоям:

- `app/` — app shell, роутер, провайдеры;
- `pages/` — страницы;
- `widgets/` — агрегированные UI-блоки;
- `features/` — сценарные модули;
- `entities/` — сущности домена UI-уровня;
- `shared/` — инфраструктурные утилиты, API, UI primitives.

## Библиотеки и контракты

### Ядро

- `leptos`, `leptos_router` — UI и маршрутизация (CSR/WASM);
- `tailwindcss` + shadcn token model;

### i18n

- `leptos_i18n` 0.6 (feature `csr`) — compile-time многоязычность через `t_string!()` / `t!()` макросы;
- `leptos_i18n_build` — кодогенерация i18n-модуля из `locales/*.json` через `build.rs`;
- файлы локалей: `locales/en.json`, `locales/ru.json` (вложенный JSON, ~260 ключей).

### Данные и API

- `leptos-graphql` — GraphQL transport/контракты;
- `leptos-auth` — auth/session контракты;
- `leptos_query` — кэширование/prefetch запросов.

### Формы и состояние

- `leptos-hook-form`, `leptos-zod` — формы/валидация;
- `leptos-zustand` — управление состоянием;
- `leptos-struct-table`, `leptos-shadcn-pagination` — таблицы/пагинация;
- `leptos-chartistry` — графики на дашборде.

## Взаимодействие

- `apps/server` (HTTP/GraphQL API)
- `crates/rustok-rbac` и другие доменные модули через backend
- общий UI контракт с `apps/next-admin` и storefront приложениями

## Документация

- Локальная: `apps/admin/docs/README.md`
- Платформенная: `docs/UI/fsd-restructuring-plan.md`, `docs/UI/rust-ui-component-catalog.md`, `docs/index.md`
