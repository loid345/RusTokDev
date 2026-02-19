# rustok-pages / CRATE_API

## Публичные модули
`dto`, `entities`, `error`, `services`.

## Основные публичные типы и сигнатуры
- `pub struct PagesModule`
- `pub struct PageService`, `MenuService`, `BlockService`
- `pub struct Page`, `Menu`, `Block`
- `pub enum PagesError`, `pub type PagesResult<T>`

## События
- Публикует domain events страниц/меню/блоков через `TransactionalEventBus`.
- Потребляет: внешние события напрямую не подписывает.

## Зависимости от других rustok-крейтов
- `rustok-core`
- `rustok-content`
- `rustok-outbox`

## Частые ошибки ИИ
- Путает `Page` (страница) и `Block` (контентный блок) в сигнатурах сервисов.
- Забывает синхронизировать публикацию/снятие с публикации в `PageService`.
- Использует DTO вместо ORM-entity в запросах SeaORM.
