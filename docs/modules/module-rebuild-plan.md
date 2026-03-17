# План подгрузки и компиляции при включении/отключении модулей

> **Этот документ объединён с `marketplace-plan.md`.**
> Актуальный план: [`docs/modules/module-system-plan.md`](module-system-plan.md)

Разделение на два файла (delivery tracker + marketplace RFC) создавало расхождения.
Единый документ `module-system-plan.md` содержит:
- Текущий статус delivery (что реализовано, что осталось)
- Platform-level install/uninstall + build pipeline
- Tenant-level toggle
- Архитектуру маркетплейс-каталога
- Влияние migration distribution и переноса CategoryService/TagService
