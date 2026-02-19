# rustok-rbac / CRATE_API

## Публичные модули
`dto`, `entities`, `error`, `services`.

## Основные публичные типы и сигнатуры
- `pub struct RbacModule`
- Публичные DTO/сервисы RBAC из `services`.
- Контракты авторизации переиспользуются из `rustok_core::permissions` и `rustok_core::rbac`.

## События
- Публикует: как правило не публикует бизнес-события по умолчанию.
- Потребляет: N/A (вызов сервисов напрямую).

## Зависимости от других rustok-крейтов
- `rustok-core`

## Частые ошибки ИИ
- Путает `Resource/Action/Permission` из core с локальными DTO.
- Добавляет проверку прав в неправильном слое (вместо application/service boundary).
