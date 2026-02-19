# rustok-outbox / CRATE_API

## Публичные модули
`entity`, `migration`, `relay`, `transactional`, `transport`.

## Основные публичные типы и сигнатуры
- `pub struct TransactionalEventBus`
- `pub struct OutboxRelay`, `pub struct RelayConfig`, `pub struct RelayMetricsSnapshot`
- `pub struct OutboxTransport`
- `pub struct SysEventsMigration`
- `pub use entity::{Entity as SysEvents, Model as SysEvent}`

## События
- Публикует: `EventEnvelope` в транспорт после фиксации транзакции.
- Потребляет: записи outbox (`sys_events`) для relay/disptach.

## Зависимости от других rustok-крейтов
- `rustok-core`

## Частые ошибки ИИ
- Публикует event напрямую в transport вместо `TransactionalEventBus::publish` внутри tx.
- Путает `OutboxTransport` и реальный L2 transport (`rustok-iggy`).
