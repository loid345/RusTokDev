# rustok-commerce / CRATE_API

## Публичные модули
`dto`, `entities`, `error`, `services`, `state_machine`.

## Основные публичные типы и сигнатуры
- `pub struct CommerceModule`
- `pub struct CatalogService`, `pub struct InventoryService`, `pub struct PricingService`
- `pub struct Order<S>` + состояния `Pending`, `Confirmed`, `Paid`, `Shipped`, `Delivered`, `Cancelled`
- `pub enum CommerceError`, `pub type CommerceResult<T>`

## События
- Публикует: `DomainEvent::ProductCreated|Updated|Published|Deleted`, `DomainEvent::PriceUpdated`, события остатков/склада из `services/*`.
- Потребляет: внешние события не подписывает напрямую (сервисный вызов).

## Зависимости от других rustok-крейтов
- `rustok-core`
- `rustok-outbox`
- (dev) `rustok-test-utils`

## Частые ошибки ИИ
- Путает доменные ошибки валидации заказа и инфраструктурные `rustok_core::Error`.
- Меняет статус заказа мимо state-machine.
- Забивает на `ValidateEvent` перед публикацией событий.
