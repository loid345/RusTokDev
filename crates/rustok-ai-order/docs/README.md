# Документация `rustok-ai-order`

`rustok-ai-order` — domain-owned support crate для order AI verticals.

## Назначение

- вынести order AI ownership из `rustok-ai` core runtime;
- держать order-scoped contracts (`order_analytics`, `order_ops_assistant`) в отдельном bounded контексте.

## Зона ответственности

- registration seam для order AI verticals;
- typed contracts/policies для recommendation и operator-assist flows.

## Проверка

- `cargo check -p rustok-ai-order`

## Связанные документы

- [README crate](../README.md)
- [План реализации](./implementation-plan.md)
