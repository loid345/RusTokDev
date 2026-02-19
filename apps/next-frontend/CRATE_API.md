# apps/next-frontend / CRATE_API

## Публичные модули
- Next.js storefront (App Router): публичные страницы каталога/контента/поиска.

## Основные структуры/контракты
- Публичные маршруты storefront.
- Контракты чтения данных из backend API/GraphQL.

## События
- Публикует: клиентские query-запросы и пользовательские действия в backend.
- Потребляет: ответы API и кешированные состояния клиента.

## Зависимости от других крейтов/пакетов
- `packages/leptos-graphql` и смежные frontend packages, backend `apps/server`.

## Частые ошибки ИИ
- Переносит admin-specific контракты/компоненты в frontend storefront.
- Ошибки импорта между server/client компонентами Next.js.
