# apps/server / CRATE_API

## Публичные модули
- Точка входа API-сервера Loco (`src/main.rs`, `src/app.rs`, HTTP/GraphQL handlers).
- Интеграция `ModuleRegistry` и доменных `rustok-*` модулей.

## Основные структуры/контракты
- `AppContext` из `rustok-core` как основной runtime context.
- Публичный HTTP/GraphQL контракт сервера (эндпоинты, schema, auth middleware).
- Инициализация event runtime и outbox relay.

## События
- Публикует: интеграционные domain events из подключённых модулей.
- Потребляет: broker/outbox поток для фоновой обработки и индексации.

## Зависимости от других крейтов
- `rustok-core`, `rustok-events`, `rustok-outbox`, доменные `rustok-*` модули.

## Частые ошибки ИИ
- Путает `AppContext` сервера с локальными контекстами модулей.
- Регистрирует модуль без объявления зависимостей в `ModuleRegistry`.
