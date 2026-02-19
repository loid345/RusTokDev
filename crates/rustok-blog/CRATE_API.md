# rustok-blog / CRATE_API

## Публичные модули
`dto`, `entities`, `error`, `services`, `state_machine`.

## Основные публичные типы и сигнатуры
- `pub struct BlogModule`
- `pub struct PostService`
- `pub struct BlogPost<S>`, `pub enum BlogPostStatus`, `pub enum CommentStatus`
- Состояния: `Draft`, `Published`, `Archived`
- `pub enum BlogError`, `pub type BlogResult<T>`

## События
- Публикует: blog-производные domain events через `rustok-content`/`rustok-outbox` pipeline.
- Потребляет: внешние события напрямую не подписывает.

## Зависимости от других rustok-крейтов
- `rustok-content`
- `rustok-core`
- `rustok-outbox`

## Частые ошибки ИИ
- Пытается добавлять отдельные миграции для blog (модуль использует таблицы content).
- Путает blog state-machine и content state-machine.
- Пропускает проверку permissions (`Resource::Posts`, `Resource::Comments`).
