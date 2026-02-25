# Unified design system: shadcn/ui class port for all Leptos apps

- Date: 2026-02-25
- Status: Accepted

## Context

RusToK имеет четыре UI-приложения на двух технологических стеках:

| Приложение | Стек | Текущая дизайн-система |
|-----------|------|----------------------|
| `apps/admin` | Leptos CSR | shadcn CSS vars + Tailwind (migrated ✅) |
| `apps/next-admin` | Next.js / React | shadcn/ui |
| `apps/storefront` | Leptos SSR | DaisyUI (`btn`, `card`, `badge-*` утилиты) |
| `apps/next-frontend` | Next.js / React | кастомный Button с `bg-sky-600`, `text-slate-*` |

Вопросы, поставленные при ревью:

1. **Правильно ли мы унифицировали компоненты в админках?** Должны ли они быть похожи визуально?
2. **Применим ли тот же подход на фронтендах** (`apps/storefront` и `apps/next-frontend`)?
3. **Можно ли перенести весь shadcn/ui под Rust/Leptos?**

### Анализ текущего состояния

**Админки** (`apps/admin` ↔ `apps/next-admin`):
Унификация через shadcn — правильное решение. Обе реализации — интерфейсы управления платформой. Консистентность критична: одна команда работает в обоих, одни сценарии (таблицы, формы, модальные). shadcn-переменные дают pixel-perfect паритет без лишних зависимостей.

**Фронтенды** (`apps/storefront` ↔ `apps/next-frontend`):
Ситуация другая:
- `apps/storefront` использует **DaisyUI** (`btn`, `card`, `badge`, `stats`, `navbar`, `hero`) — это consumer-facing витрина, где DaisyUI даёт богатый набор готовых UI-паттернов для e-commerce (карточки товаров, навигация, hero-секции)
- `apps/next-frontend` использует кастомный Button с хардкодированными цветами (`bg-sky-600`, `border-slate-200`) — это скелет, который не следует ни shadcn ни DaisyUI последовательно
- Семантика использования различается: витрина — это маркетинговые страницы, каталог, продуктовые страницы — не admin-интерфейс

**Полный shadcn → Leptos порт**:
shadcn/ui содержит ~50+ компонентов. Часть реализуема как чистые CSS-классы (Button, Badge, Input, Card — уже сделано). Другая часть требует сложного JS-поведения: Dialog (focus trap, scroll lock), Combobox (virtual scroll, keyboard nav), DatePicker, Toast — это существенная работа и не оправдана если Leptos-экосистема уже имеет рабочие аналоги (Leptonic, Thaw).

## Decision

### 1. Админки — текущее решение верно и остаётся

`apps/admin` ↔ `apps/next-admin` — одинаковые shadcn CSS-переменные, одинаковый `tailwind.config`, компоненты портируются из shadcn/ui исходников. Это правильно. Продолжать этот подход.

### 2. Фронтенды — единый токен-слой, но DaisyUI остаётся для Leptos storefront

**`apps/next-frontend`** переводится на shadcn CSS-переменные и удаляет хардкодированные цвета (`bg-sky-600`, `text-slate-*`). Это скелет, который следует привести к тому же стандарту что и `apps/next-admin` (shadcn/ui компоненты, shadcn CSS vars).

**`apps/storefront`** (Leptos SSR) — DaisyUI **остаётся** как основная система для витрины:
- DaisyUI оптимален для e-commerce UI (navbar, hero, card, stats, carousel) — богатая семантика из коробки
- Это SSR-рендеринг без WASM, компоненты — HTML-строки, не нужны реактивные состояния
- Паритет с Next.js витриной по функциям важнее чем паритет по классам

**Однако**: цветовые значения в DaisyUI-теме `rustok` синхронизируются с shadcn-переменными так, чтобы `primary`, `secondary`, `accent` совпадали визуально. Одна палитра бренда — две системы классов.

### 3. Полный shadcn → Leptos порт — не делать как монолит, портировать по требованию

Полный порт 50+ компонентов — избыточная работа. Стратегия:

| Категория | Подход |
|-----------|--------|
| Презентационные (Button, Badge, Input, Card, Label, Separator, Spinner) | ✅ Уже портированы. Продолжать порт по shadcn-исходникам. |
| Простые интерактивные (Switch, Checkbox, Select, Textarea) | ✅ Уже портированы. |
| Overlay-компоненты (Dialog, Sheet, Popover, Tooltip) | Leptonic/Thaw как временное решение → нативный порт по мере реальной потребности |
| Сложные данные (Table, DataTable, Pagination, DatePicker) | Leptonic/Thaw или `leptos-struct-table`. Порт нативный — только если существующие решения неприемлемы |
| Навигация (Breadcrumb, Tabs, Accordion) | Портировать нативно — CSS-only, несложно |

Приоритет следующих портов для `apps/admin`:
1. `Alert` / `AlertDialog` — нужны для UX критических действий
2. `Tabs` — для страниц настроек
3. `Breadcrumb` — уже в header нативно, оформить как компонент

## Consequences

### Положительные
- Единая палитра бренда работает во всех четырёх приложениях
- `apps/next-frontend` становится консистентным с `apps/next-admin`
- `apps/storefront` сохраняет DaisyUI-производительность для SSR витрины
- Минимальный внешний Leptos-UI риск — не тянем крупные нестабильные крейты
- Новые shadcn-компоненты портируются в Leptos за часы, не дни

### Отрицательные / Trade-offs
- `apps/storefront` и `apps/admin` используют разные CSS-классы для визуально похожих элементов (DaisyUI `btn` vs shadcn `bg-primary text-primary-foreground`) — нельзя переиспользовать Leptos-компоненты между витриной и админкой напрямую
- При обновлении shadcn в Next.js-проектах — нужно вручную синхронизировать классы в Leptos-версиях
- Overlay-компоненты (Dialog и пр.) временно визуально расходятся

### Follow-up tasks

**`apps/next-frontend`** — привести к shadcn стандарту:
- [ ] Добавить shadcn CSS-переменные в `src/styles/globals.css`
- [ ] Обновить `tailwind.config.ts` — shadcn color tokens
- [ ] Переписать `src/components/ui/button.tsx` — shadcn variants вместо `bg-sky-600`
- [ ] Удалить хардкодированные цвета из `page.tsx` (`text-slate-*`, `border-slate-*`, `bg-sky-*`)
- [ ] Добавить shadcn компоненты по мере потребности

**`apps/storefront`** — синхронизировать палитру бренда:
- [ ] Обновить цвета DaisyUI-темы `rustok` в `tailwind.config.cjs` — привести `primary`, `secondary`, `accent` к значениям из shadcn CSS vars платформы

**`UI/leptos/src/`** — следующие компоненты на порт:
- [ ] `Alert` — `src/alert.rs`
- [ ] `Tabs` — `src/tabs.rs`
- [ ] `Breadcrumb` — `src/breadcrumb.rs`
- [ ] `Skeleton` — `src/skeleton.rs`
