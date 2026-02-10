# Паритет библиотек для админки (Leptos-first)

Этот документ фиксирует **известные** соответствия библиотек между админками и станет базой для унификации стека.

## Контекст админок

- **Описание:** сравнение библиотек и паттернов между админками, чтобы стек был единым и прогнозируемым.
- **Стек:** Leptos CSR (`apps/admin`), TailwindCSS, shadcn-style компоненты, общие дизайн‑токены для админки и фронтенда.
- **Ссылки:** [UI документы](./) • [UI parity](./ui-parity.md) • [IU библиотеки](../../IU/README.md)

## Известные аналоги (подтверждено в коде/доках)

| Категория | Leptos admin | Frontend parity | Примечание |
| --- | --- | --- | --- |
| CSS/дизайн-токены | TailwindCSS (`tailwind-rs`) | TailwindCSS (storefront) | Один набор токенов и переменных должен использоваться и в админке, и во фронтенде. |
| CSS pipeline | PostCSS + Autoprefixer | PostCSS + Autoprefixer | Одинаковая цепочка сборки стилей. |
| UI контракт | shadcn/ui | shadcn-style components | В документации зафиксирован единый shadcn‑style подход для обеих админок. |
| Каталог аналогов | N/A | N/A | Список библиотек и адаптеров: https://github.com/leptos-rs/awesome-leptos |
| Метаданные (Next.js) | next/metadata | leptos-next-metadata | https://github.com/cloud-shuttle/leptos-next-metadata |
| Data fetching | @tanstack/react-query | leptos-query | https://github.com/cloud-shuttle/leptos-query |
| i18n | next-intl | leptos_i18n | https://github.com/Baptistemontan/leptos_i18n |
| GraphQL client | graphql-request (или fetch) | `leptos-graphql` (тонкий transport слой на `reqwest`) + `Resource`/actions Leptos | На клиенте не используем `async-graphql`; запросы идут по HTTP к `/api/graphql`, typed codegen — через `graphql-client` (опционально). |

## Требуют поиска и подтверждения

- Формы/валидация (Next.js: react-hook-form + zod).
- Таблицы (Next.js: @tanstack/react-table).
- Data fetching (Next.js: @tanstack/react-query).
- State (Next.js: zustand).

## Принципы выбора библиотек

Наш приоритет — **максимальное использование готовых библиотек** для реализации функционала.
При создании нового функционала **нужно сначала найти и предложить** соответствующую библиотеку/интеграцию.
Иначе в конце мы получим неработающий самопис, который сложно поддерживать и масштабировать.

## Матрица заимствований и источники

Чтобы не раздувать документ, отдельная матрица с источниками и ссылками ведётся здесь:
[`docs/UI/admin-reuse-matrix.md`](./admin-reuse-matrix.md).

This is an alpha version and requires clarification. Be careful, there may be errors in the text. So that no one thinks that this is an immutable rule.


## Зафиксированный GraphQL-подход (RusToK)

1. **Transport:** `leptos-graphql` (наш тонкий внутренний слой, без собственного state runtime).
2. **HTTP:** `reqwest` как battle-tested клиент.
3. **Типизация (по необходимости):** `graphql-client` для codegen из `.graphql` файлов.
4. **State lifecycle:** `leptos::Resource`/actions в UI страницах.

Это осознанная альтернатива Apollo/urql-подобным монолитам для Leptos-части.
