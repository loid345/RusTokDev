# rustok-forum / CRATE_API

## Публичные модули
`constants`, `dto`, `entities`, `error`, `services`.

## Основные публичные типы и сигнатуры
- `pub struct ForumModule`
- `pub struct CategoryService`, `TopicService`, `ReplyService`, `ModerationService`
- Публичные DTO/константы из `dto::*` и `constants::*`
- `pub enum ForumError`, `pub type ForumResult<T>`

## События
- Публикует forum-события через outbox pipeline сервисов.
- Потребляет: явных подписчиков на внешние события нет.

## Зависимости от других rustok-крейтов
- `rustok-content`
- `rustok-core`
- `rustok-outbox`

## Частые ошибки ИИ
- Неправильно использует лимиты/константы модерации из `constants`.
- Путает иерархию category/topic/reply в импортах сущностей.
- Игнорирует tenant-boundary в сервисных фильтрах.
