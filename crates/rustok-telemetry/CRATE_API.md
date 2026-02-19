# rustok-telemetry / CRATE_API

## Публичные модули
`metrics`, `otel`.

## Основные публичные типы и сигнатуры
- `pub struct TelemetryConfig`, `pub struct TelemetryHandles`
- `pub enum LogFormat`, `pub enum TelemetryError`
- `pub fn init(config: TelemetryConfig) -> Result<TelemetryHandles, TelemetryError>`
- `pub fn render_metrics() -> Result<String, prometheus::Error>`
- `pub fn current_trace_id() -> Option<String>`

## События
- Публикует: метрики/трейсы observability.
- Потребляет: сигналы и spans из `tracing`/OTel.

## Зависимости от других rustok-крейтов
- нет прямых зависимостей на другие `rustok-*`.

## Частые ошибки ИИ
- Повторно вызывает `init` и получает `SubscriberAlreadySet`.
- Путает application metrics registry и глобальный prometheus registry.
