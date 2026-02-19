# rustok-events / CRATE_API

## Публичные модули
- Отдельные модули не экспонирует; crate делает re-export событий.

## Основные публичные типы и сигнатуры
- `pub use rustok_core::events::{DomainEvent, EventEnvelope}`
- `pub use rustok_core::{DomainEvent as RootDomainEvent, EventEnvelope as RootEventEnvelope}`

## События
- Публикует: N/A (только контракты событий).
- Потребляет: N/A.

## Зависимости от других rustok-крейтов
- `rustok-core`

## Частые ошибки ИИ
- Добавляет новые типы событий прямо в `rustok-events`; source-of-truth остаётся в `rustok-core`.
- Использует одновременно alias и base-export без необходимости.
