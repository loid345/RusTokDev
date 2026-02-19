# rustok-test-utils / CRATE_API

## Публичные модули
`db`, `events`, `fixtures`, `helpers`.

## Основные публичные типы и сигнатуры
- `pub async fn setup_test_db(...)`
- `pub struct MockEventBus`, `pub struct MockEventTransport`
- `pub fn mock_transactional_event_bus() -> TransactionalEventBus`
- Фикстуры доменных сущностей в `fixtures::*`.

## События
- Публикует: тестовые `DomainEvent` через mock transport.
- Потребляет: записанные event envelope для assertions.

## Зависимости от других rustok-крейтов
- `rustok-core`
- `rustok-outbox`
- (optional) `rustok-content`, `rustok-commerce`

## Частые ошибки ИИ
- Подключает crate в production dependencies (должен быть только dev).
- Ожидает реальную доставку брокером вместо behavior mock transport.
