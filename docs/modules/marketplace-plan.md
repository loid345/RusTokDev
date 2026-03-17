# RusTok Module Marketplace — Архитектура и план

> **Этот документ объединён с `module-rebuild-plan.md`.**
> Актуальный план: [`docs/modules/module-system-plan.md`](module-system-plan.md)

Разделение на два файла (marketplace RFC + delivery tracker) создавало расхождения.
Единый документ `module-system-plan.md` содержит:
- Архитектуру маркетплейса (бывший RFC)
- Delivery-статус по каждому компоненту
- Tenant-level toggle и platform-level install/uninstall
- Влияние migration distribution и переноса CategoryService/TagService
