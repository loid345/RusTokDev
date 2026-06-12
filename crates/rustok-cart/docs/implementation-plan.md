# План реализации `rustok-cart`

Статус: cart boundary выделен; модуль остаётся owner-ом cart state и storefront
context snapshot, а orchestration над checkout живёт в umbrella `rustok-commerce`.

## Execution checkpoint

- Current phase: phase_b_ready
- Last checkpoint: Cart storefront FFA Phase B считается закрытой: cart-owned storefront UI остаётся внутри `rustok-cart/storefront`, имеет `core/transport/ui` split, Leptos adapter не вызывает raw `api::*`, а cart-owned UI не перетаскивается в umbrella `rustok-commerce` или host-приложения.
- Next step: Не продолжать механические FFA-переносы cart UI; следующий work item — parity/evidence hardening для SSR native path, GraphQL fallback, headless cart mutation contracts и DOM evidence, либо новый FFA-срез только при появлении реального cart-owned storefront функционала.
- Open blockers: None.
- Hand-off notes for next agent: После каждого инкремента обновлять этот блок и central readiness board.
- Last updated at (UTC): 2026-06-12T13:20:43Z


## FFA/FBA status

- FFA status: `phase_b_ready`
- FBA status: `not_started`
- Structural shape: `core_transport_ui`
- Evidence:
  - module plan синхронизирован с central FFA/FBA readiness board; UI surface уже опубликован и ведётся в migration/backlog ритме;
  - storefront slice выделяет `core/` helpers для route/input normalization, UUID validation, adjustment metadata mapping, channel-slug normalization, decrement policy, typed fetch/decrement/remove request objects, GraphQL decrement command dispatch, stable serializable transport fallback error evidence, DOM evidence adapter и display/view-model mapping;
  - `ui/leptos::CartView` теперь вызывает thin `transport` facade через core-owned request objects, получает prepared view-model values из `core/` и рендерит error evidence attributes `data-cart-transport-failed-path`, `data-cart-transport-fallback-attempted`, `data-cart-transport-native-error`, `data-cart-transport-graphql-error`; transport facade сохраняет validation errors без GraphQL retry и возвращает `CartTransportError` со stable `failed_path` (`native_server`/`graphql`), `fallback_attempted`, `native_error` и `graphql_error`, а native `#[server]` + GraphQL adapter calls остаются внутри API adapter layer, при этом API layer больше не пересчитывает GraphQL decrement policy;
  - Phase B closure decision: cart storefront FFA больше не расширяется без нового cart-owned UI/transport surface; UI функционал cart подмодуля остаётся в `rustok-cart/storefront`, а umbrella `rustok-commerce` продолжает только checkout orchestration/handoff;
  - дальнейшее повышение до `parity_verified` выполняется только вместе с full parity evidence и обновлением local+central docs.
- Last verified at (UTC): 2026-06-12T13:20:43Z
- Owner: `rustok-cart` module team

## Область работ

- удерживать `rustok-cart` как owner cart lifecycle и line-item state;
- синхронизировать cart snapshot contract, runtime dependencies, storefront UI ownership и local docs;
- не допускать возврата cart domain logic обратно в umbrella или host слой.

## Текущее состояние

- `carts` и `cart_line_items` уже module-owned;
- `cart_adjustments` уже module-owned и фиксируют language-neutral promotion/discount snapshot без display labels;
- tax runtime уже больше не зашит напрямую в cart service: `rustok-cart` вызывает `rustok-tax`,
  а `cart_tax_lines` теперь несут typed `provider_id`;
- cart lifecycle и persisted storefront context snapshot уже встроены в базовый contract;
- cart write-side теперь поддерживает batch repricing line items при смене контекста/количества,
  чтобы unit_price оставался согласован с pricing resolver;
- transport adapters по-прежнему публикуются фасадом `rustok-commerce`, без цикла зависимостей;
- storefront cart inspection, safe decrement/remove write-side и seller-aware delivery-group snapshot уже вынесены в `rustok-cart/storefront`;
- storefront package продолжил FFA-декомпозицию: pure cart UI policy, typed request construction, GraphQL command dispatch, stable transport error evidence, Leptos DOM evidence adapter и display/view-model mapping разложены по `storefront/src/core/{identifiers,policy,request,view_model,error}.rs`, Leptos layer живёт в `storefront/src/ui/leptos.rs` и использует facade в `storefront/src/transport/mod.rs`, native-first/GraphQL fallback orchestration живёт в `storefront/src/transport/`, а adapter calls остаются в `storefront/src/api.rs`;
- channel/context/deliverability orchestration поверх cart по-прежнему выполняется на уровне umbrella-модуля.
- targeted tests теперь явно фиксируют, что cart mutation paths `set_adjustments` и typed promotion apply-path отклоняются при `checking_out`, чтобы во время checkout не было конкурентной мутации pricing snapshot.

## Этапы

### 1. Contract stability

- [x] зафиксировать cart lifecycle и storefront context snapshot;
- [x] удерживать line-item CRUD и totals внутри `rustok-cart`;
- [x] добавить typed cart adjustment snapshot с `subtotal_amount`, `adjustment_total` и net `total_amount`;
- [x] удерживать sync между cart runtime contract, commerce orchestration, storefront route ownership и module metadata.

### 2. Storefront ownership

- [x] вынести storefront cart inspection в `rustok-cart/storefront`;
- [x] использовать native Leptos `#[server]` functions как default internal data layer;
- [x] сохранить GraphQL storefront contract как fallback;
- [x] вынести безопасные cart-owned line-item decrement/remove mutations из aggregate storefront surface;
- [x] начать FFA-разделение storefront package на `core/` policy/request/view-model helpers, `transport` facade и `ui/leptos` render layer;
- [ ] не смешивать cart-owned UI с quantity increase, add-to-cart и checkout orchestration, пока эти write-path требуют cross-domain validation.

### 3. Checkout hardening

- [x] удерживать `checking_out`/recovery semantics совместимыми с payment/order orchestration;
- [x] покрывать stale snapshot, shipping selection и multi-group edge-cases targeted tests;
- [x] развивать cart state только через explicit snapshot/versioning semantics.

### 4. Operability

- [ ] документировать новые cart guarantees одновременно с изменением checkout flows;
- [x] удерживать local docs и `README.md` синхронизированными со storefront contract;
- [ ] расширять diagnostics только при реальном runtime pressure.

## Проверка

- `cargo xtask module validate cart`
- `cargo xtask module test cart`
- targeted tests для cart lifecycle, line items, typed adjustments, snapshot context и checkout-preflight semantics

## Правила обновления

1. При изменении cart runtime contract сначала обновлять этот файл.
2. При изменении public/runtime surface синхронизировать `README.md`, `docs/README.md` и `storefront/README.md`.
3. При изменении module metadata синхронизировать `rustok-module.toml`.
4. При изменении checkout orchestration expectations обновлять umbrella docs в `rustok-commerce`.


## Quality backlog

- [ ] Актуализировать покрытие тестами по ключевым сценариям модуля.
- [ ] Проверить полноту и актуальность `README.md` и локальных docs.
- [ ] Зафиксировать/обновить verification gates для текущего состояния модуля.
