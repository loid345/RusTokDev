# Error Handling Guide

Полное руководство по обработке ошибок в RusToK находится в [`docs/standards/errors.md`](../standards/errors.md).

## Краткое резюме

В текущем коде RusToK использует два совместимых уровня обработки ошибок:

1. Базовый `rustok_core::error::Error` для обратной совместимости в domain/service коде.
2. Расширенный `RichError` + `ErrorKind` + `ErrorResponse` для стандартизированных API-ответов и richer context.

Актуальные категории ошибок в коде:

| HTTP | Категория | Когда использовать |
|------|-----------|-------------------|
| 400 | `Validation` | Ошибки валидации входных данных |
| 401 | `Unauthenticated` / `Auth` | Требуется аутентификация |
| 403 | `Forbidden` | Нет прав доступа |
| 404 | `NotFound` | Ресурс не найден |
| 409 | `Conflict` | Конфликт состояния или дублирование |
| 429 | `RateLimited` | Превышен лимит запросов |
| 500 | `Internal` / `Database` / `Serialization` / `Cache` / `Scripting` | Внутренние ошибки приложения |
| 503 | `ExternalService` | Ошибка внешнего сервиса |
| 504 | `Timeout` | Таймаут запроса |

## Правила

1. Библиотечный и сервисный код возвращает типизированные `Result<T, E>`; в `rustok-core` backward-compatible alias — `Result<T, rustok_core::error::Error>`.
2. Для richer API/HTTP semantics используется `RichError` и `ErrorResponse`.
3. Использование `.unwrap()` / `.expect()` запрещено в production-коде, кроме явно безопасных или тестовых сценариев.
4. Внутренние ошибки не должны раскрываться клиенту без необходимости; для client-facing ответов используется `user_message` либо стандартизированный `ErrorResponse`.
5. Для трассировки поддерживаются `trace_id` и структурированный error context.

## Полная документация

→ [`docs/standards/errors.md`](../standards/errors.md)
