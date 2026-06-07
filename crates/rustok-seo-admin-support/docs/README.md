# Документация `rustok-seo-admin-support`

`rustok-seo-admin-support` — support crate для owner-module admin UI, который даёт переиспользуемые SEO widgets и transport helper-ы без переноса ownership экрана в `rustok-seo-admin`.

## Назначение

- давать content-модулям общий SEO UI/tooling слой для entity-owned editor surfaces;
- держать shared GraphQL helper-ы для `seoMeta`, `upsertSeoMeta`, `publishSeoRevision`;
- публиковать единый `SeoEntityPanel` и lightweight capability notices для owner-side admin screens.

## Зона ответственности

- reusable Leptos panel для explicit SEO metadata authoring;
- simple completeness scoring и form/view-model helper-ы;
- canonical host-locale consumption без package-local locale input: panel берёт effective locale из owner-module context,
  canonicalizes BCP47-like tags и не inventит свой fallback chain;
- reusable diagnostics/widgets layer: snippet preview, recommendations card, delivery/remediation cards и state notice
  можно переиспользовать в owner-module layouts без возврата к central SEO hub;
- shared control-plane widget state contract (`loading/ready/empty/permission_denied/error`) для единых
  loading/error/permission/empty состояний SEO control-plane виджетов;
- owner-module integration seam между `rustok-seo` runtime и `pages/product/blog/forum` admin packages.

## Интеграция

- используется `rustok-pages/admin`, `rustok-product/admin`, `rustok-blog/admin`, `rustok-forum/admin`;
- owner-side panel chrome локализуется от host locale и больше не держит editable locale field внутри SEO panel;
- не владеет собственным runtime, tenant settings, RBAC policy или central SEO route.

## Phase D alignment

Support crate синхронизируется с SEO Phase D по направлениям:

- reusable observability/remediation widgets для owner-module SEO panels;
- transport fallback parity (GraphQL/REST) для Leptos и Next admin hosts;
- verification/UX consistency matrix для shared panel surface.

## Связанные документы

- [README crate](../README.md)
- [План реализации](./implementation-plan.md)
- [Документация `rustok-seo`](../../rustok-seo/docs/README.md)
