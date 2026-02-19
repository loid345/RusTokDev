# rustok-tenant / CRATE_API

## Публичные модули
`dto`, `entities`, `error`, `services`.

## Основные публичные типы и сигнатуры
- `pub struct TenantModule`
- Публичные tenant DTO/сервисы из `services`.
- `TenantModule` реализует `RusToKModule` с `ModuleKind::Core`.

## События
- Публикует: N/A (базовый core-модуль tenancy).
- Потребляет: N/A.

## Зависимости от других rustok-крейтов
- `rustok-core`

## Частые ошибки ИИ
- Смешивает `tenant slug` и внутренний `tenant_id`.
- Не добавляет tenant isolation в запросы и проверки доступа.
