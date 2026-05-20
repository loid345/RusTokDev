# Документация `rustok-ai-content`

`rustok-ai-content` — domain-owned support crate для AI content-модерации.

## Назначение

- изолировать content moderation vertical от core-runtime `rustok-ai`;
- подготовить единый policy seam для blog/forum/comment moderation сценариев.

## Зона ответственности

- registration seam для `content_moderation`;
- typed moderation contracts и approval integration hooks.

## Проверка

- `cargo check -p rustok-ai-content`

## Связанные документы

- [README crate](../README.md)
- [План реализации](./implementation-plan.md)
