# Документация `rustok-ai-product`

`rustok-ai-product` — domain-owned support crate для AI-вертикалей продуктового домена.

## Назначение

- вынести product AI vertical ownership из `rustok-ai` core runtime;
- держать product-scoped AI contracts (`product_copy`, `product_attributes`) рядом с продуктовым доменом;
- подготовить модуль к поэтапному переносу direct handler wiring.

## Зона ответственности

- registration seam для product AI verticals;
- typed contracts и policy hooks для product AI задач;
- координация с `rustok-product`/`rustok-commerce` по read/write контрактам.

## Интеграция

- execution host: `rustok-ai`;
- domain services: `rustok-product`, `rustok-commerce`;
- operator surface: `rustok-ai` admin packages.

## Проверка

- `cargo check -p rustok-ai-product`

## Связанные документы

- [README crate](../README.md)
- [План реализации](./implementation-plan.md)
