# Final cutover gate для перехода RBAC в `casbin_only`

- Date: 2026-03-05
- Status: Accepted

## Context

Relation graph уже принят как канонический источник RBAC данных. Перед переключением runtime в `casbin_only` платформа обязана пройти контролируемое parity-окно и зафиксировать формальный Go/No-Go пакет, чтобы cutover не зависел от неявных договорённостей.

## Decision

1. **Переход в `casbin_only` разрешён только после полного pre-cutover gate.**
   - Staging rehearsal завершён и invariant artifacts приложены.
   - Baseline helper `scripts/rbac_cutover_baseline.sh` показывает:
     - `rustok_rbac_engine_mismatch_total` delta = 0
     - `rustok_rbac_shadow_compare_failures_total` delta = 0
     - decision volume >= `min-decision-delta`
   - Auth release gate закрыт и приложен к release bundle.

2. **Rollback gate обязателен и определяется до switch.**
   - Немедленный rollback: возврат в `RUSTOK_RBAC_AUTHZ_MODE=casbin_shadow`.
   - Trigger'ы rollback:
     - рост 401/403 сверх baseline;
     - аномальный deny-rate без ожидаемого бизнес-контекста;
     - деградация authorization latency выше согласованного SLO;
     - рост `rustok_rbac_shadow_compare_failures_total` или других authz-path incident indicators после switch.

3. **Release evidence обязателен.**
   - `artifacts/rbac-staging/*`
   - `artifacts/rbac-cutover/*`
   - auth release-gate report
   - gate decision summary (`Go`/`No-Go`) с ответственными ролями

## Consequences

### Положительные

- Cutover в `casbin_only` становится воспроизводимым и аудируемым.
- Решение опирается на parity evidence, а не на интуитивную оценку готовности.

### Компромиссы

- Добавляется формальный release overhead.
- Требуется дисциплина по сбору и хранению operational artifacts.

### Follow-up actions

1. Приложить реальные cutover artifacts к ближайшему release окну.
2. После стабильного `casbin_only` закрыть cleanup хвосты и обновить steady-state runbooks.
