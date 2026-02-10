# server

## Назначение
`apps/server` — модуль/приложение RusToK. Здесь находится его код и корневая документация.

## Взаимодействие
- crates/rustok-core
- доменные модули crates/rustok-*
- apps/admin и apps/next-frontend как клиенты API

## Документация
- Локальная документация: `./docs/`
- Общая документация платформы: `/docs`

## Паспорт компонента
- **Роль в системе:** Главный backend RusToK: API, модули, миграции, orchestration runtime.
- **Основные данные/ответственность:** бизнес-логика и API данного компонента; структура кода и документации в корне компонента.
- **Взаимодействует с:**
  - crates/rustok-core
  - все доменные crates/rustok-*
  - apps/admin/apps/storefront как клиенты API
- **Точки входа:**
  - `apps/server/src/main.rs`
  - `apps/server/src/controllers/*`
- **Локальная документация:** `./docs/`
- **Глобальная документация платформы:** `/docs/`

