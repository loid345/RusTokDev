# rustok-content / CRATE_API

## Публичные модули
`dto`, `entities`, `error`, `services`, `state_machine`.

## Основные публичные типы и сигнатуры
- `pub struct ContentModule`
- `pub struct NodeService`
- `pub struct Node`, `pub struct NodeTranslation`, `pub struct Body`
- `pub struct ContentNode<S>` + состояния `Draft`, `Published`, `Archived`
- `pub type ContentResult<T>`, `pub enum ContentError`

## События
- Публикует доменные события контента через `TransactionalEventBus` (создание/обновление/публикация/архивация node).
- Потребляет: внешние доменные события явно не подписывает (бизнес-операции вызываются сервисами).

## Зависимости от других rustok-крейтов
- `rustok-core`
- `rustok-outbox`
- (dev) `rustok-test-utils`

## Частые ошибки ИИ
- Нарушает state-machine (`Draft -> Published -> Archived`) прямым изменением статуса.
- Путает `entities::Model` SeaORM и DTO ответа API.
- Пропускает `tenant_id` в фильтрах запросов.
