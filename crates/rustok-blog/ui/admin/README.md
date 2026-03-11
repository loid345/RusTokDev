# rustok-blog admin UI package

This package provides Next.js admin surfaces for the blog module.

## Responsibilities

- Blog post CRUD screens and forms.
- Rich-text editing with `rt_json_v1` via Tiptap.
- Legacy markdown to `rt_json_v1` migration helpers in post form.
- Reusable `PageBuilder` and `ForumReplyEditor` components that share the same `rt_json_v1` UX contract.

## Entry points

- `index.ts` — module registration and public exports.
- `components/post-form.tsx` — primary post editor UI.
- `components/rt-json-editor.tsx` — reusable Tiptap wrapper.

## Interactions

- Uses GraphQL transport via `@/lib/graphql` wrappers in `api/*.ts`.
- Integrates with server GraphQL contracts for blog/pages/forum.

## Docs map

See central docs index: `docs/index.md`.
