# rustok-forum

## Назначение
`rustok-forum` — модуль форума: категории, темы, ответы, модерация и форумные workflow.

## Что делает
- Управляет категориями, темами и ответами.
- Использует `rustok-content` как слой хранения.
- Обеспечивает локализацию контента через `locale` с полной fallback-цепочкой.
- Публикует форумные доменные события через `TransactionalEventBus` из `rustok-outbox`.
- Проверяет статус топика перед созданием ответа (нельзя отвечать в закрытый/архивный топик).

## Как работает (простыми словами)
1. API обращается к сервису форума.
2. Сервис валидирует входные данные (включая статус топика для ответов).
3. Данные сохраняются через `NodeService` из `rustok-content`.
4. Форумное событие публикуется через `TransactionalEventBus` из `rustok-outbox` для надёжной доставки.

## Ключевые компоненты
- `constants.rs` — типы узлов и статусы форума.
- `locale.rs` — хелперы locale fallback: `resolve_translation`, `resolve_body`, `available_locales`.
- `dto/` — структуры запросов и ответов (с `effective_locale`, `available_locales`, `author_id`, `slug`).
- `services/` — категории, темы, ответы, модерация.
- `entities/` — модели SeaORM (пока пустые — фаза 3).

## i18n / Многоязычность
Fallback-цепочка при разрешении перевода: `запрошенный → "en" → первый доступный`.

Все response DTO возвращают:
- `locale` — запрошенный locale
- `effective_locale` — фактически использованный locale
- `available_locales` — список всех доступных локалей (для TopicResponse, CategoryResponse)

## Форумные события (DomainEvent)
Все события определены в `rustok-core::events::DomainEvent`:

| Событие | Когда |
|---------|-------|
| `ForumTopicCreated` | Создание темы |
| `ForumTopicReplied` | Добавление ответа |
| `ForumTopicStatusChanged` | close / archive темы |
| `ForumTopicPinned` | Закрепление / открепление |
| `ForumReplyStatusChanged` | approve / reject / hide ответа |

## Кому нужен
Форуму, комьюнити-разделам, поддержке и контентным обсуждениям.

## Взаимодействие
- crates/rustok-core
- crates/rustok-content
- crates/rustok-outbox (TransactionalEventBus)
- crates/rustok-rbac
- crates/rustok-index

## Документация
- Локальная документация: `./docs/`
- Общая документация платформы: `/docs`

## Паспорт компонента
- **Роль в системе:** Доменный модуль форума: темы, ответы, модерация и события обсуждений.
- **Основные данные/ответственность:** бизнес-логика и API данного компонента; структура кода и документации в корне компонента.
- **Взаимодействует с:**
  - crates/rustok-core
  - crates/rustok-content
  - crates/rustok-outbox (TransactionalEventBus)
  - crates/rustok-rbac
  - crates/rustok-index
- **Точки входа:**
  - `crates/rustok-forum/src/lib.rs`
- **Локальная документация:** `./docs/`
- **Глобальная документация платформы:** `/docs/`
