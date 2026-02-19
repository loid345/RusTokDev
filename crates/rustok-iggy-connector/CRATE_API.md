# rustok-iggy-connector / CRATE_API

## Публичные модули
- API объявлен в `lib.rs` (без `pub mod` секций).

## Основные публичные типы и сигнатуры
- `pub enum ConnectorMode { Embedded, Remote }`
- `pub struct EmbeddedConnectorConfig`, `RemoteConnectorConfig`, `ConnectorConfig`
- `pub trait IggyConnector`
- `pub trait MessageSubscriber`
- `pub enum ConnectorError`
- Реализации: `RemoteConnector`, `EmbeddedConnector` и subscriber-структуры.

## События
- Публикует/потребляет бинарные сообщения Iggy в рамках коннектора (не `DomainEvent` напрямую).

## Зависимости от других rustok-крейтов
- нет прямых зависимостей на другие `rustok-*`.

## Частые ошибки ИИ
- Путает `Embedded` и `Remote` конфиги при инициализации.
- Считает connector полноценным EventBus (это уровень подключения/IO).
