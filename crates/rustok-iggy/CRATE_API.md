# rustok-iggy / CRATE_API

## Публичные модули
`config`, `consumer`, `dlq`, `health`, `partitioning`, `producer`, `replay`, `serialization`, `topology`, `transport`.

## Основные публичные типы и сигнатуры
- `pub struct IggyTransport` (реализация `EventTransport`)
- `pub trait EventSerializer` + `JsonSerializer`, `BincodeSerializer`
- `pub struct TopologyManager`, `ConsumerGroupManager`, `DlqManager`, `ReplayManager`
- `pub fn health_check(...) -> HealthCheckResult`

## События
- Публикует: сериализованные `EventEnvelope` в Iggy stream/topics.
- Потребляет: сообщения из Iggy consumer groups, включая replay/DLQ pipeline.

## Зависимости от других rustok-крейтов
- `rustok-core`
- `rustok-iggy-connector`

## Частые ошибки ИИ
- Пропускает partition key и ломает порядок обработки.
- Использует не тот сериализатор между producer/consumer.
