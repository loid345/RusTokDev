# RBAC source of truth и staged runtime rollout

- Date: 2026-02-26
- Status: Accepted

## Context

Платформа уже переведена на relation-derived RBAC graph как единственный источник permission данных. Следующий шаг миграции касается не источника данных, а runtime decision path: relation-resolver должен пройти контролируемый путь к `casbin_only` без потери наблюдаемости и без возврата legacy decision layers.

Ключевая архитектурная цель: `crates/rustok-rbac` остаётся policy-host, а `apps/server` ограничивается adapter/wiring обязанностями.

## Decision

1. **Source of truth для RBAC данных — только relation-модель.**
   - Канонические таблицы: `roles`, `permissions`, `user_roles`, `role_permissions`.
   - legacy column-based role path отсутствует в authorization decision.

2. **Runtime rollout фиксируется как `relation_only -> casbin_shadow -> casbin_only`.**
   - `relation_only`: relation-resolver авторитативен.
   - `casbin_shadow`: relation-resolver авторитативен, Casbin path работает как parity check.
   - `casbin_only`: authorization decision принимает Casbin runtime.

3. **Dual-read и legacy-role paths не являются допустимой частью целевой архитектуры.**
   - Они не развиваются дальше.
   - Их presence в runtime/docs/scripts считается cleanup debt и подлежит удалению.

4. **Module-first boundary обязателен.**
   - `rustok-rbac` владеет evaluator/runtime/shadow semantics.
   - `apps/server` владеет DB/cache/logging/metrics adapters.

## Consequences

### Положительные

- Данные и runtime разделены чисто и предсказуемо.
- Cutover наблюдаем через relation-vs-casbin parity, а не через legacy fallback.
- Упрощается долгосрочная поддержка RBAC и release gates.

### Компромиссы

- Нужен отдельный parity window перед `casbin_only`.
- Временное сосуществование relation decision и Casbin shadow увеличивает объём observability logic до завершения cutover.

### Follow-up actions

1. Держать migration plan синхронизированным с relation/casbin-only моделью.
2. Удалять любые новые legacy compatibility layers при первом безопасном окне.
3. Закрыть `casbin_only` gate отдельным release decision.
