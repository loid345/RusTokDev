# rustok-index / CRATE_API

## Публичные модули
`content`, `error`, `product`, `search`, `traits`.

## Основные публичные типы и сигнатуры
- `pub struct IndexModule`
- `pub trait Indexer`, `pub trait LocaleIndexer`
- `pub struct IndexerContext`
- `pub enum IndexError`, `pub type IndexResult<T>`

## События
- Публикует: обычно не является источником бизнес-событий.
- Потребляет: индексируемые события контента/коммерции через интеграционный слой приложения.

## Зависимости от других rustok-крейтов
- `rustok-core`

## Частые ошибки ИИ
- Путает domain-level индексер (`traits`) и конкретные адаптеры поиска.
- Делает индексирование без учёта locale.
