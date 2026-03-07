# Унификация UI модулей между Next.js и Leptos Admin

- Date: 2026-03-07
- Status: Accepted & Implemented

## Context

Модуль управления модулями (Modules page) был реализован в двух admin-панелях
(Next.js и Leptos), но с расхождениями:

| Аспект | Next.js (до) | Leptos | Расхождение |
|---|---|---|---|
| i18n | Хардкод строк | `translate()` + JSON locales (en/ru) | Нет локализации |
| ModuleCard | Inline-функция в `modules-list.tsx` | Отдельный `module_card.rs` (FSD) | Нарушение FSD |
| Toast-сообщения | Строки с интерполяцией имени | Ключи `modules.toast.*` | Несогласованные тексты |
| i18n-ключи | Не используются | `modules.*` (15+ ключей) | — |

Пользователь ожидает **WordPress-like UX**: одинаковый опыт вне зависимости от стека.

## Decision

### 1. Единые locale-файлы

Оба admin-приложения используют **идентичные** JSON locale-файлы с одинаковыми ключами:

```
apps/admin/locales/en.json        ← Leptos (source of truth)
apps/admin/locales/ru.json
apps/next-admin/messages/en.json  ← Next.js (copy, same keys)
apps/next-admin/messages/ru.json
```

Ключи модулей:
```
modules.title, modules.subtitle, modules.eyebrow
modules.section.core, modules.section.optional
modules.always_active, modules.always_on
modules.badge.core
modules.enabled, modules.disabled
modules.depends_on
modules.toast.enabled, modules.toast.disabled
modules.error.load
```

### 2. i18n-система для Next.js Admin

Лёгковесная реализация без `next-intl` (не нужен URL-routing для admin):

```
src/shared/lib/i18n.ts       — zustand store + t() function
src/shared/hooks/use-i18n.ts — useT() React hook
messages/en.json              — English locale
messages/ru.json              — Russian locale
```

**Pattern**: localStorage `rustok-admin-locale` — идентичен Leptos admin.

### 3. FSD-структура компонентов (единая)

```
# Leptos Admin                    # Next.js Admin
features/modules/                  features/modules/
├── api.rs                         ├── api.ts
├── mod.rs                         └── components/
└── components/                        ├── module-card.tsx    ← extracted
    ├── mod.rs                         └── modules-list.tsx
    ├── module_card.rs
    └── modules_list.rs

entities/module/                   (types inline in api.ts —
├── model.rs                        TS convention)
└── mod.rs
```

### 4. Матрица соответствия строк

| i18n Key | Leptos использует | Next.js использует |
|---|---|---|
| `modules.section.core` | `ModulesList` | `ModulesList` |
| `modules.section.optional` | `ModulesList` | `ModulesList` |
| `modules.always_active` | `ModulesList` badge | `ModulesList` badge |
| `modules.badge.core` | `ModuleCard` | `ModuleCard` |
| `modules.always_on` | `ModuleCard` | `ModuleCard` |
| `modules.enabled` | `ModuleCard` | `ModuleCard` |
| `modules.disabled` | `ModuleCard` | `ModuleCard` |
| `modules.depends_on` | `ModuleCard` | `ModuleCard` |
| `modules.toast.enabled` | `ModulesList` toggle | `ModulesList` toggle |
| `modules.toast.disabled` | `ModulesList` toggle | `ModulesList` toggle |
| `modules.title` | `Modules` page | `page.tsx` PageContainer |
| `modules.subtitle` | `Modules` page | `page.tsx` PageContainer |

## Consequences

### Позитивные

- **Единый UX**: пользователь видит одинаковые тексты в обоих стеках.
- **Единые locales**: добавление нового языка = один JSON-файл, копируется в оба app.
- **FSD consistency**: компонентная структура совпадает между стеками.
- **Лёгкая i18n**: zustand + localStorage, без тяжёлых зависимостей.

### Негативные

- **Дублирование JSON**: locale-файлы копируются в два места.
  Митигация: в будущем можно вынести в shared workspace package.
- **Ручная синхронизация**: при добавлении ключа нужно обновить оба файла.
  Митигация: CI-проверка на совпадение ключей.

### Follow-up

1. Применить i18n к остальным страницам Next.js admin (users, dashboard, profile, etc.)
   — сейчас хук готов, но используется только на странице модулей.
2. CI: добавить проверку на совпадение ключей между `apps/admin/locales/` и `apps/next-admin/messages/`.
3. Рассмотреть `@rustok/admin-locales` workspace package для единого источника.
