# План реализации `rustok-customer`

Статус: customer boundary выделен; модуль остаётся owner-ом storefront customer
profile, admin UI ownership уже вынесен в `rustok-customer/admin`, а storefront
transport и checkout orchestration остаются у umbrella `rustok-commerce`.

## Execution checkpoint

- Current phase: ffa_admin_native_adapter_split
- Last checkpoint: Admin customer core теперь владеет list/detail view-model policy (`CustomerListItemViewModel`, `CustomerDetailViewModel`), активным CSS row-state, form snapshot mapping (`CustomerAdminFormSnapshot`) и submit-command validation/preparation; Leptos adapter оставлен render/effect слоем, который применяет core snapshots, переводит host-provided locale в display labels и вызывает transport facade. Native Leptos server functions остаются в `admin/src/transport/native_server_adapter.rs`.
- Next step: Продолжать сужать `admin/src/ui/leptos.rs`: переносить page-level empty/loading/error policy и editor action-state helpers в `core` без изменения native CRUD behavior; при появлении второго transport path добавить explicit adapter рядом с `native_server_adapter.rs`.
- Open blockers: None.
- Hand-off notes for next agent: После каждого инкремента обновлять этот блок и держать central readiness board синхронизированным.
- Last updated at (UTC): 2026-06-12T18:50:39Z

## FFA/FBA status

- FFA status: `in_progress`
- FBA status: `not_started`
- Structural shape: `core_transport_ui`
- Evidence:
  - модуль ведётся в ускоренном FFA migration track; FBA остаётся `not_started` до закрытия FFA phase-gate как часть ecommerce family;
  - любые изменения UI/transport boundary должны фиксироваться с parity/boundary evidence в этом же инкременте;
  - admin FFA slice добавил framework-agnostic `admin/src/core.rs` list request policy, submit-command validation/preparation, form snapshot mapping, list/detail row view-model policy and active row CSS policy; `admin/src/transport/mod.rs` remains the module-owned facade over native-only `admin/src/transport/native_server_adapter.rs` `#[server]` endpoints; explicit Leptos render adapter `admin/src/ui/leptos.rs` now consumes core view-models/snapshots and no longer owns covered list/detail fallback strings or active-row class decisions; legacy `admin/src/api.rs` удалён, `admin/src/lib.rs` только wires modules и re-export `CustomerAdmin`.
- Last verified at (UTC): 2026-06-12T18:50:39Z
- Owner: `rustok-customer` module team

## Область работ

- удерживать `rustok-customer` как отдельный customer domain module;
- синхронизировать customer contract, optional user/profile bridge и local docs;
- не смешивать customer profile с platform/admin user domain.

## Текущее состояние

- `customers` и `CustomerService` уже выделены в отдельный модуль;
- optional linkage на `user_id` и bridge к `profiles` уже существуют как integration contract;
- `rustok-customer` уже публикует собственный module-owned admin UI package `rustok-customer/admin` с `admin/src/core.rs` defaults для request, submit-command policy, form snapshots и list/detail view-model policy, `admin/src/transport/mod.rs` facade поверх `admin/src/transport/native_server_adapter.rs` native Leptos server functions для list/detail/create/update customer records и явным `admin/src/ui/leptos.rs` render adapter;
- transport adapters по-прежнему публикуются фасадом `rustok-commerce`;
- customer read/write contract не превращает customer в canonical public profile surface.

## Этапы

### 1. Contract stability

- [x] зафиксировать отдельный customer profile boundary;
- [x] удерживать optional linkage к `user` и `profiles` как integration-only contract;
- [x] удерживать sync между customer runtime contract, commerce transport и module metadata.

### 2. Domain expansion

- [ ] расширять customer-owned settings/profile flows только внутри модуля;
- [ ] удерживать ownership guard и tenant isolation покрытыми targeted tests;
- [ ] не допускать размывания customer semantics в auth/user domain.

### 3. Operability

- [x] документировать новые customer guarantees одновременно с изменением runtime surface;
- [x] удерживать local docs и `README.md` синхронизированными;
- [ ] добавлять richer diagnostics только при реальном operational pressure.

## Проверка

- `cargo xtask module validate customer`
- `cargo xtask module test customer`
- targeted tests для customer CRUD/lookup, ownership guard и optional profile bridge

## Правила обновления

1. При изменении customer runtime contract сначала обновлять этот файл.
2. При изменении public/runtime surface синхронизировать `README.md` и `docs/README.md`.
3. При изменении module metadata синхронизировать `rustok-module.toml`.
4. При изменении integration с `auth`/`profiles` обновлять связанные module docs.


## Quality backlog

- [ ] Актуализировать покрытие тестами по ключевым сценариям модуля.
- [ ] Проверить полноту и актуальность `README.md` и локальных docs.
- [ ] Зафиксировать/обновить verification gates для текущего состояния модуля.
