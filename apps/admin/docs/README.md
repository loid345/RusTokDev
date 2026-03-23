# Документация Leptos Admin

Локальная документация для `apps/admin`.

## Текущий runtime contract

- UI/state: `leptos`, `leptos_router`, `Resource`, actions.
- GraphQL transport: `crates/leptos-graphql`.
- Build progress: `/modules` использует `buildProgress` через `/api/graphql/ws`, polling остаётся только fallback-механизмом.
- FSD-структура остаётся канонической: `app/`, `pages/`, `widgets/`, `features/`, `entities/`, `shared/`.
- Tailwind/shadcn миграция завершена: новые экраны используют семантические CSS-переменные и общие UI-примитивы.

## Generated module UI wiring

- `apps/admin/build.rs` теперь читает `modules.toml` и модульные `rustok-module.toml`, а затем генерирует manifest-driven wiring в `OUT_DIR`.
- Текущий convention-based contract для publishable Leptos admin UI: `[provides.admin_ui].leptos_crate` плюс экспорт корневого компонента `<PascalSlug>Admin`.
- Host регистрирует три generic surface-а без знания о конкретном модуле: `AdminSlot::DashboardSection`, `AdminSlot::NavItem` и `AdminPageRegistration`.
- Для module-owned admin pages используется единый host route `/modules/:module_slug`: `ModuleAdminPage` резолвит модуль по generated registry и рендерит его root component, если модуль включён у tenant.
- `[provides.admin_ui]` может дополнительно задать `route_segment` и `nav_label`; если они не указаны, host берёт `module.slug` и `module.name`.
- Референсные publishable admin packages в workspace сейчас: `rustok-blog-admin`, `rustok-workflow-admin` и `rustok-pages-admin`.

## Ограничения

- Закрыт только module root page contract. Более богатый nested route/page contract для модульных admin-crate’ов пока не реализован.
- `workflow` уже имеет publishable root page в `crates/rustok-workflow/admin`, но detail/edit flow пока ещё живёт на legacy-маршрутах `/workflows/*`.
- Для внешних crate-ов вне текущего workspace всё ещё нужен более явный entry-point contract, чем текущие naming conventions.

## Связанные документы

- [План реализации](./implementation-plan.md)
- [Контракты UI API](../../../UI/docs/api-contracts.md)
- [Каталог UI-компонентов Rust](../../../docs/UI/rust-ui-component-catalog.md)
- [Карта документации](../../../docs/index.md)
