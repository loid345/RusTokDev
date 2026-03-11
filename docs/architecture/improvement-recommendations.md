# RusToK — рекомендации по развитию архитектуры

- Дата: 2026-02-19
- Статус: Живой документ
- Последнее обновление: 2026-03-11
- Основание обновления: учтены изменения по состоянию кода, ADR от 2026-03-07 и code audit от 2026-03-11 по foundation/runtime/frontends
- Автор: Архитектурное ревью платформы

---

## 1. Состояние на 2026-03-11

Этот документ больше не пытается пересказывать всю архитектуру платформы. Его задача — держать в одном месте только открытые архитектурные треки, их приоритеты и зависимости между ними.

### 1.1 Что уже считаем базовым инвариантом

Ниже — вещи, которые больше не ведём как отдельный backlog в этом документе:

- граница между `Core` и `Optional` модулями зафиксирована и отражена в registry;
- `rustok-index`, `rustok-tenant`, `rustok-rbac` работают как обязательные core-модули;
- `rustok-outbox` признан критическим инфраструктурным слоем write-path и не рассматривается как optional capability;
- манифест модулей (`modules.toml`) уже перешёл на composable-модель слоёв `server/admin/storefront`, а `DeploymentProfile` уже вычисляется из `[build.server]`;
- унификация admin UI между Leptos и Next.js принята и реализована: обе админки уже используют библиотечный i18n (`leptos_i18n` / `next-intl`) и синхронизированные locale-файлы.

### 1.2 Что ещё остаётся источником архитектурного риска

- Надёжность event consumers заметно выросла, но transport/runtime guardrails всё ещё не production-hard: `relay_target=iggy` умеет тихо деградировать в `memory`, а backpressure controller в core-слое пока не подключён к server runtime как обязательный бюджет очередей.
- Политика локалей по-прежнему фрагментирована: backend и четыре UI-стека используют разные источники locale, разные default/fallback semantics, а runtime-модель tenant'а не экспонирует `default_locale`, хотя миграции и часть документации уже опираются на него.
- `apps/server/src/app.rs::after_routes()` остаётся перегруженным композиционным корнем с ручной инициализацией event runtime, tenant cache, Alloy scheduler, rate limiting и UI wiring.
- Highload read-path'ы ещё не доведены до предсказуемого бюджета: GraphQL schema собирается на каждый запрос, dashboard/admin read-модель местами строится через серию `count()` и полную загрузку событий в память, а часть pagination/filter logic по-прежнему не опирается на bounded SQL path.
- Typed per-tenant settings не доведены до production-ready runtime-контракта; `tenant.settings` и `tenant_modules.settings` в критичных путях по-прежнему остаются сырым JSON без versioned schema и upgrade hooks.
- Module-owned UI composition остаётся частичной: у optional-модулей нет завершённого parity-контракта для `Next + Leptos` admin/storefront, а bundle-регистрация всё ещё требует ручного знания о модуле в центральных приложениях.

### 1.3 Что изменилось с предыдущего обновления

#### 2026-03-07: composable deployment layers

В кодовой базе уже отражены:

- новый формат `modules.toml` с `[build.server]`, `[build.admin]`, `[[build.storefront]]`;
- вычисляемый `DeploymentProfile` (`Monolith`, `ServerWithAdmin`, `ServerWithStorefront`, `HeadlessApi`);
- документация манифеста и типовых конфигураций.

Но бэклог не закрыт: пока не реализованы обещанные Cargo features `embed-admin` / `embed-storefront`, полная сборочная оркестрация и CLI preset'ы.

#### 2026-03-07: унификация admin UI/i18n

Подзадача расхождения между двумя admin-стеками больше не должна считаться самостоятельной архитектурной проблемой:

- Leptos admin уже сидит на `leptos_i18n`;
- Next.js admin уже сидит на `next-intl`;
- locale-файлы и структура модулей выровнены.

Следствие для бэклога: трек единой locale-policy переводим из чистого `Запланировано` в `В работе`, но фокус смещаем с admin UI на backend/storefront/runtime contract.

#### 2026-03-11: code audit по foundation, runtime, frontends и highload-path'ам

По состоянию кода на 2026-03-11 подтверждены дополнительные точки, которые нужно отразить в backlog как часть следующей фазы взросления платформы:

- GraphQL schema в `apps/server` всё ещё собирается на каждый HTTP-запрос, а не живёт как boot-time singleton/shared factory; это увеличивает лишнюю аллокацию и мешает формализовать request-path budget.
- Admin/read-path'ы пока местами не bounded: `dashboard_stats` читает несколько счётчиков напрямую и затем грузит историю `order.placed` из `sys_events` в память для агрегации; часть pagination/filter logic в forum/admin остаётся application-side, а не строго SQL-side.
- i18n-contract расходится между слоями: backend по умолчанию часто выбирает `en`, Leptos storefront работает через `?lang=`, Next storefront — через locale path и default `ru`, Next admin — через cookie и default `en`; при этом миграции уже содержат `tenants.default_locale` и `tenant_locales`, а runtime entity/`TenantContext` ещё нет.
- Runtime guardrails для horizontal scale не завершены: server использует in-memory rate limiters с hardcoded значениями, `RustokSettings.rate_limit` не подключён к фактическому middleware path, а transport policy для event relay в production всё ещё допускает soft fallback в `memory`.
- Модульная UI-модель пока частичная: `next-admin` и `next-frontend` реально self-register'ят только blog UI packages, тогда как `content/commerce/forum/pages/alloy` ещё не имеют такого же контрактного пути.

---

## 2. Актуальный бэклог

### 2.1 Вынести `DomainEvent` из `rustok-core` в `rustok-events`

**Статус на 2026-03-08:** `В работе`

**Почему задача всё ещё открыта:**
- `rustok-events` существует, но пока остаётся фазой совместимого re-export;
- доменные модули и `apps/server` продолжают массово импортировать `DomainEvent` из `rustok-core`;
- ownership событийного контракта всё ещё размыт между core-слоем и отдельным crate.

**Ближайший scope:**
- завершить phases 2/3 из ADR `2026-02-19-rustok-events-canonical-contract.md`;
- мигрировать импорты доменных модулей и server-кода на `rustok-events`;
- ввести явную deprecation-политику для старых public paths в `rustok-core`.

**Критерии готовности:**
- канонический public import для событий — `rustok-events`;
- новые доменные события больше не добавляются напрямую в `rustok-core` как основной public entry point;
- дата удаления legacy compatibility layer зафиксирована в ADR/документации;
- migration покрыта integration/regression тестами.

### 2.2 Типизированные настройки модулей на уровне tenant

**Статус на 2026-03-08:** `Бэклог`

**Почему задача всё ещё открыта:**
- концепция typed settings уже понятна, но единый runtime-контракт отсутствует;
- миграционный путь с legacy `tenant_modules.settings` пока не стандартизирован;
- нет общей схемы версионирования настроек и safe defaulting для модулей.

**Ближайший scope:**
- определить единый runtime-контракт настроек: валидация, дефолты, версионирование, upgrade hooks;
- зафиксировать формат хранения в БД и правила миграции legacy JSON;
- ввести набор integration tests на backward compatibility.

**Критерии готовности:**
- включение/обновление module settings проходит через единый typed validation path;
- модуль может объявить версию настроек и безопасно мигрировать старые payload'ы;
- backward compatibility и rollback сценарии покрыты тестами.

### 2.3 `rustok-notifications` как optional-модуль

**Статус на 2026-03-08:** `Бэклог`

**Почему задача всё ещё открыта:**
- потребность в capability есть, но отдельного `rustok-notifications` crate/module contract нет;
- нет общего lifecycle, health и telemetry контракта для notification-подсистемы;
- tenant-aware включение и настройка зависят от трека 2.2.

**Ближайший scope:**
- оформить модуль как `RusToKModule` с `ModuleKind::Optional`;
- определить минимальные transport-адаптеры и эксплуатационные метрики;
- встроить per-tenant toggle и зависимость от typed settings.

**Критерии готовности:**
- модуль подключается через registry и lifecycle optional-модулей;
- health/metrics доступны в стандартном формате платформы;
- настройки уведомлений идут через typed settings path, а не через ad-hoc JSON.

### 2.4 Разделение `apps/server` на `core-server` и `module-bundles`

**Статус на 2026-03-08:** `Бэклог`

**Почему задача всё ещё открыта:**
- ADR `2026-02-19-core-server-module-bundles-routing.md` есть, но implementation phase не начата;
- route wiring всё ещё зависит от ручной сборки в центральном server-слое;
- задача логически зависит от упрощения composition root (2.7) и стабилизации deployment/build contract (2.9).

**Ближайший scope:**
- подготовить technical design для `HttpModule`/route-provider контракта;
- описать route parity checks до миграции;
- определить rollback path на один релиз.

**Критерии готовности:**
- optional-модули могут регистрировать HTTP routes без ручной правки центрального glue-кода;
- parity/integration tests подтверждают совпадение маршрутизации до/после миграции;
- `core-server` и bundle-слой имеют ясные ownership boundaries.

### 2.5 Надёжность EventBus consumers

**Статус на 2026-03-11:** `В работе`

**Почему задача всё ещё открыта:**
- в `rustok-core` dispatcher уже обрабатывает `Lagged/Closed`, но это пока только базовый защитный слой;
- supervised restart/resubscribe policy уже введена не для всех transport-bound loops, а transport policy всё ещё допускает тихую деградацию `iggy -> memory` для relay target;
- backpressure controller и queue budget есть в foundation-коде, но server runtime не использует их как обязательный production contract;
- нет минимального операционного набора сигналов и runbook'ов для backlog saturation, DLQ growth и bounded replay/reindex при инцидентах.

**Ближайший scope:**
- инвентаризировать все consumer loops и выровнять реакцию на `Lagged/Closed`;
- отделить dev-friendly fallback от production-safe fail-closed policy для transport/relay target;
- подключить конфигурируемые queue capacities/backpressure budgets к `server_event_forwarder` и dispatcher path;
- расширить операционный baseline: backlog/DLQ saturation thresholds, replay budget и reindex runbook для read-моделей (`partial` vs `full`).

**Критерии готовности:**
- в платформе не остаётся consumer loop'ов с немой деградацией или тихой остановкой;
- production profile не допускает тихий `relay_target=iggy -> memory` без явного opt-in;
- `/metrics` и `/health/ready` показывают не только lag/restart, но и bounded queue/backlog degradation;
- runbook документирует, когда достаточно partial reindex, а когда нужен full rebuild read-моделей и replay backlog.

#### 2.5.1 Фаза A — runtime-contract и observability baseline для consumers

**Статус на 2026-03-08:** `Выполнено`

**Scope**
- вынести общий runtime-helper для long-lived consumer loops в `rustok-core`;
- выровнять реакцию на `Lagged/Closed` в `EventDispatcher`, `server_event_forwarder` и GraphQL build-progress subscription;
- добавить минимальные telemetry-сигналы `rustok_event_consumer_lagged_total`, `rustok_event_consumer_restarted_total`, `rustok_event_dispatch_latency_ms`;
- зафиксировать runbook для выбора `partial` vs `full` reindex после lag-инцидентов.

**Результат фазы**
- в `rustok-core` появился `EventConsumerRuntime`, который закрепляет единый contract для long-lived consumers: bootstrap/restart signal, `Lagged -> warn + metric`, `Closed -> explicit info + stop`;
- `EventDispatcher` теперь публикует `rustok_event_dispatch_latency_ms` как платформенный сигнал latency внутри in-memory dispatch path;
- в `apps/server` больше нет немой деградации у `server_event_forwarder` и `graphql_build_progress`: lag и закрытие подписки явно логируются и отражаются в `/metrics`;
- runbook в `docs/architecture/events.md` теперь объясняет, когда достаточно partial reindex, а когда нужен full rebuild read-моделей.

**Что остаётся на следующую подфазу**
- ввести настоящую supervised restart/resubscribe policy для consumers, которые работают поверх внешних transport/runtime, а не только над `tokio::broadcast`;
- довести telemetry/read-model recovery до интеграционных тестов полного event path.

**Перепроверка**
- `cargo test -p rustok-core event_consumer_runtime --lib`
- `cargo test -p rustok-telemetry test_event_bus_metrics --test metrics_test`
- `cargo check -p rustok-core --lib`
- `cargo check -p rustok-server --bin rustok-server`

#### 2.5.2 Фаза B — supervised resubscribe для внешних consumer loops

**Статус на 2026-03-08:** `Выполнено`

**Scope**
- найти consumer loop поверх внешнего runtime, где `Closed`/ошибка подключения всё ещё приводят к тихой остановке;
- ввести supervised restart/resubscribe policy с backoff для `tenant_invalidation_listener`, работающего через Redis pubsub;
- зафиксировать, что библиотечная замена здесь не требуется: listener уже использует стабильный `redis` crate, проблема была не в выборе библиотеки, а в отсутствии restart contract.

**Результат фазы**
- redis-based tenant invalidation listener в `apps/server/src/middleware/tenant.rs` больше не завершается навсегда при ошибке `get_async_pubsub()` или `subscribe()`;
- loop теперь работает как supervised consumer: явный `startup`, повторный `retry` после ошибки, контролируемый backoff `5s`, structured warn-log и метрика `rustok_event_consumer_restarted_total{consumer="tenant_invalidation_listener",...}`;
- payload invalidation вынесен в явный parse-path, malformed сообщения теперь логируются как отдельная деградация, а не теряются совсем без следа;
- runbook дополнен правилом, что для деградации `tenant_invalidation_listener` обычно не нужен reindex read-моделей: сначала надо восстановить invalidation path и, при необходимости, локально сбросить tenant cache.

**Что остаётся на следующую подфазу**
- покрыть полный event-path telemetry integration tests, включая read-model recovery;
- решить, нужен ли отдельный health/readiness signal для consumers с внешним transport, а не только counters в `/metrics`.

**Перепроверка**
- `cargo test -p rustok-server tenant_cache_stampede_test --test tenant_cache_stampede_test`
- `cargo check -p rustok-server --bin rustok-server`
- проверить `/metrics` на наличие `rustok_event_consumer_restarted_total{consumer="tenant_invalidation_listener",...}` после рестарта listener

#### 2.5.3 Фаза C — readiness signal для внешних consumers

**Статус на 2026-03-08:** `Выполнено`

**Scope**
- не вводить новую health-подсистему, а встроить consumer-loop state в уже существующие `/health/ready` и `/metrics`;
- добавить минимальный state snapshot для `tenant_invalidation_listener`: `disabled | starting | healthy | degraded`;
- оставить сигнал не-критичным для readiness aggregation, чтобы падение invalidation path не уроняло весь server в `unhealthy`, но всё же делало его `degraded`.

**Результат фазы**
- `tenant_invalidation_listener` теперь публикует state snapshot, который используется и в readiness check `tenant_cache_invalidation`, и в метрике `rustok_tenant_invalidation_listener_status`;
- `/health/ready` больше не слеп к внешнему cache-invalidation consumer: при деградации listener появляется явная причина в `degraded_reasons`;
- `/metrics` теперь показывает отдельный gauge по listener state, поэтому on-call видит не только факт рестартов, но и текущее steady-state состояние.

**Что остаётся на следующую подфазу**
- довести эти сигналы до полноценного integration/regression coverage после снятия внешнего storefront blocker;
- решить, нужно ли расширять такой же readiness contract на другие внешние consumers/relay loops.

**Перепроверка**
- `cargo check -p rustok-server --bin rustok-server`
- проверить `GET /health/ready` на наличие `tenant_cache_invalidation` в `checks`
- проверить `GET /metrics` на наличие `rustok_tenant_invalidation_listener_status`

### 2.6 Единая политика локалей

**Статус на 2026-03-11:** `В работе`

**Почему задача всё ещё открыта:**
- admin UI уже выровнен, но backend и storefront до сих пор не живут по одному negotiation contract;
- runtime-модель tenant'а не экспонирует `default_locale`, хотя миграции и часть тестов уже опираются на это поле;
- backend, Leptos storefront, Next storefront и Next admin сейчас используют разные default/fallback semantics (`en`, `ru`, cookie, query param, locale path);
- `RequestContext` по-прежнему использует упрощённый `Accept-Language` parsing, а blog/forum/content дублируют hardcoded fallback helpers с `en`;
- документ `docs/architecture/i18n.md` отстаёт от новой реальности с несколькими UI-стеками и composable deployment model.

**Ближайший scope:**
- закрепить единую policy: `URL locale -> cookie -> Accept-Language -> tenant default`;
- формализовать fallback цепочку контента: `requested -> tenant.fallback -> tenant.default -> en`;
- вытянуть `default_locale` и enabled locales в runtime contract (`TenantContext`, entities, tests, API-layer);
- вынести locale negotiation/fallback helpers из модулей в общий policy слой и различать `requested_locale` / `effective_locale`;
- расширить допустимую длину `locale` как минимум до 16 символов для BCP47-подобных тегов;
- синхронизировать `docs/architecture/i18n.md` с фактическим runtime contract.

**Критерии готовности:**
- backend, admin и storefront используют одинаковую семантику locale negotiation;
- в кодовой базе не остаётся локальных hardcoded fallback chain'ов, противоречащих общей policy;
- API и UI явно различают `requested_locale` и `effective_locale`, если сработал fallback;
- `tenant.default_locale` и `tenant_locales` реально участвуют в runtime resolution, а не живут только в миграциях/документации;
- схему хранения locale можно безопасно использовать для BCP47-подобных тегов без точечных исключений.

### 2.7 Тонкий `apps/server` как композиционный корень

**Статус на 2026-03-11:** `Запланировано`

**Почему задача всё ещё открыта:**
- `after_routes()` по-прежнему смешивает routing, lifecycle, background workers, event runtime, rate limiting, Alloy и UI wiring;
- GraphQL schema по-прежнему собирается внутри request handler, а не инициализируется как boot-time dependency;
- limiter'ы и часть long-lived runtime'ов создаются inline в `app.rs`, а не через отдельные bootstrap/initializer компоненты;
- любое изменение старта приложения повышает риск регрессий и мешает следующему шагу — 2.4.

**Ближайший scope:**
- вынести тяжёлые подсистемы в отдельные init-компоненты внутри текущего процесса;
- вынести schema/runtime factories в shared bootstrappers и переиспользуемые handles;
- перевести создание limiter'ов и runtime guardrails на config-driven initializers;
- оставить в composition root только wiring: routing, middleware, registration и orchestration;
- покрыть init/health/stop интеграционными smoke-тестами.

**Критерии готовности:**
- server startup раскладывается на понятные bootstrap-компоненты;
- GraphQL/schema/runtime объекты переиспользуются между запросами и не создаются на hot-path'е;
- `app.rs` перестаёт быть местом прямой бизнес-инициализации подсистем;
- lifecycle ключевых подсистем проверяется smoke-тестами, а не только ручным прогоном.

### 2.8 Масштабирование БД только по evidence

**Статус на 2026-03-11:** `В работе`

**Почему задача всё ещё открыта:**
- code audit уже подтвердил несколько конкретных hot-path'ов (`dashboard_stats`, `sys_events`, product search, users list, index rebuild loops), но решения уровня partitioning и materialized read models пока не подтверждены измерениями;
- нужен baseline, который не будет подменять инженерные данные предположениями и позволит отделить must-fix от nice-to-have.

**Ближайший scope:**
- провести аудит индексов и SQL plan'ов на подтверждённых hot-path запросах;
- собрать baseline через `pg_stat_statements` и сохранить EXPLAIN-планы для top-N запросов;
- отдельно замерить admin/read-model path'ы, которые сейчас делают in-memory aggregation или pagination;
- подготовить partition-ready дизайн без немедленного включения в production.

**Критерии готовности:**
- top-N SQL hot paths известны и документированы;
- для целевых запросов есть метрики до/после, план индексации и решение: `rewrite / cache / read model / оставить как есть`;
- тяжёлые схемные изменения запускаются только после подтверждённого bottleneck.

### 2.9 Компонуемые слои развёртывания и пайплайн сборки

**Статус на 2026-03-11:** `В работе`

**Почему задача всё ещё открыта:**
- manifest contract и `DeploymentProfile` уже вошли в код и документацию;
- но `apps/server/Cargo.toml` пока не содержит обещанные `embed-admin` / `embed-storefront` features;
- build-service и `rustok rebuild` ещё не стали полным production path для всех профилей из ADR `2026-03-07-deployment-profiles-and-ui-stack.md`;
- Next/Leptos module UI bundles и manual side-effect imports пока не встроены в тот же manifest/build contract.

**Ближайший scope:**
- реализовать Cargo features для встраивания admin/storefront артефактов;
- научить build pipeline и build-service собирать команды из `modules.toml`;
- добавить validation для несовместимых комбинаций стека, embedding и module UI bundle surface;
- ввести smoke-check'и для минимум трёх конфигураций: `monolith`, `server+admin`, `headless-api`.

**Критерии готовности:**
- одна и та же manifest-модель реально управляет сборкой артефактов, а не только документирует их;
- build-service и `rustok rebuild` воспроизводимо собирают поддерживаемые deployment profiles;
- invalid configs отсекаются до начала долгой сборки;
- ключевые варианты деплоя покрыты smoke/integration проверками.

### 2.10 Bounded read-path'ы и агрегированные read models для highload

**Статус на 2026-03-11:** `Запланировано`

**Почему задача открыта:**
- часть hot admin/API path'ов остаётся небюджетной: `dashboard_stats` делает серию `count()` и читает историю `order.placed` из `sys_events` в память для агрегации;
- request-layer пока не формализует budget по query count / data volume и допускает in-memory pagination/aggregation на read-path'е;
- reindex/rebuild loops для read-моделей остаются последовательными и без явных tenant budgets.

**Ближайший scope:**
- вынести dashboard KPI, recent activity и похожие admin-срезы в агрегированные read models или bounded SQL/materialized queries;
- убрать in-memory pagination/aggregation с hot-path endpoints и закрепить DB-side pagination как обязательный contract;
- ввести bounded parallelism, tenant quotas и cancellation points для reindex/rebuild workers.

**Критерии готовности:**
- hot admin/read endpoints не выполняют unbounded full scan или полную загрузку event history в память;
- у ключевых read-path'ов есть целевой query budget и telemetry по latency/rows/read volume;
- reindex throughput предсказуем, не съедает весь DB budget одного tenant'а и может быть ограничен оператором.

### 2.11 Distributed rate limiting и строгие runtime guardrails

**Статус на 2026-03-11:** `В работе`

**Почему задача открыта:**
- server использует per-process in-memory limiter'ы с hardcoded значениями, а `RustokSettings.rate_limit` не является реальным source of truth для runtime;
- при горизонтальном масштабировании текущий limiter становится неравномерным и легко обходится распределением трафика по инстансам;
- runtime guardrails для abuse/saturation пока не объединяют rate limit, transport degradation и queue budgets в одну production policy.

**Ближайший scope:**
- перевести HTTP limiter на единый settings-driven contract и добавить distributed backend (Redis/edge) для multi-instance профилей;
- ввести безопасные limiter dimensions (`tenant`, `client`, `oauth app`) только после trusted-auth extraction, без reliance на spoofable headers;
- синхронизировать limiter saturation, queue/backpressure и transport degradation с `/metrics`, `/health/ready` и alert thresholds.

**Критерии готовности:**
- rate limiting работает предсказуемо в multi-node deployment и не зависит от одного процесса;
- settings становятся единственным источником truth для limiter policy;
- abuse/saturation/guardrail degradation наблюдаемы на уровне runtime и эксплуатационных сигналов.

### 2.12 Module-owned UI bundles и parity между frontend-стеками

**Статус на 2026-03-11:** `Запланировано`

**Почему задача открыта:**
- `next-admin` и `next-frontend` пока регистрируют side-effect import'ами только blog UI packages; остальные optional-модули не имеют такого же contract path;
- optional modules ещё не владеют симметрично своими admin/storefront surface'ами в `Next + Leptos`;
- включение нового модуля по-прежнему местами требует знания о нём в центральном frontend glue-коде.

**Ближайший scope:**
- зафиксировать минимальный bundle-contract для optional-модулей: admin/storefront surfaces, locale behavior, health/telemetry expectations;
- убрать ручной central import там, где это возможно, и привязать bundle discovery к manifest/registry metadata;
- ввести parity-matrix по module UI support между `next-admin`, `next-frontend`, `apps/admin` и `apps/storefront`.

**Критерии готовности:**
- optional module может подключать UI surface без ручной правки central nav/import glue-кода;
- parity между Next и Leptos фиксируется и видна по каждому модулю;
- module ownership распространяется на UI/i18n contract, а не только на backend crate.

---

## 3. Приоритеты и зависимости

### 3.1 Что имеет смысл делать прямо сейчас

- `2.5 EventBus consumers` — это самый дешёвый способ снизить риск `silent desync`;
- `2.6 Единая политика локалей` — уже частично развернута на admin-слое, теперь важно дотянуть platform contract и устранить drift между миграциями, runtime и фронтендами;
- `2.8 Evidence-driven DB baseline` — нужен до любых тяжёлых изменений схемы и до агрессивных highload-решений;
- `2.10 Bounded read-path'ы` — самый прямой путь убрать unbounded admin/read нагрузки до реального highload;
- `2.11 Distributed rate limiting` — нужен до multi-node/multi-tenant highload, иначе защита и predictability останутся локальными на один процесс;
- `2.9 Deployment layers и build pipeline` — нужен, чтобы ADR от 2026-03-07 перестал быть только документированным дизайном.

### 3.2 Что логически зависит от предыдущих треков

- `2.4 core-server + module-bundles` зависит от `2.7` и `2.9`;
- `2.3 rustok-notifications` зависит от `2.2`;
- `2.1 DomainEvent extraction` можно вести параллельно, но без требования срочно перепахивать runtime;
- `2.10` зависит от `2.8`: сначала baseline и plan, затем выбор между query rewrite, cache и read model;
- `2.11` логически продолжает groundwork из `2.5`, но не требует ждать полного закрытия всего event backlog;
- `2.12` зависит от `2.9` и стратегически поддерживает `2.4`;
- `2.8` не блокирует другие треки, но должен предшествовать крупным БД-решениям.

### 3.3 Что сознательно не форсируем

- переход на новый event broker по умолчанию без подтверждённой эксплуатационной необходимости;
- большой replatforming всего UI за один релиз;
- partitioning и другие дорогие схемные изменения без метрик и EXPLAIN;
- plugin-ready внешние bundle-механизмы до стабилизации внутреннего bundle-слоя;
- тотальную CQRS-агрегацию для каждого модуля до подтверждения реального bottleneck на конкретном read-path'е.

---

## 4. Приоритизированный план действий

| ID | Трек | Приоритет | Статус | Зависимости | Ценность | Зона ответственности |
|---|---|---|---|---|---|---|
| 2.6 | Единая политика локалей | 🔴 Критично | В работе | — | UX / SEO-консистентность | Platform foundation + frontends + content |
| 2.5 | Надёжность EventBus consumers | 🔴 Критично | В работе | — | Надёжность / консистентность | Platform foundation + index |
| 2.8 | Evidence-driven DB baseline | 🔴 Критично | В работе | — | Предсказуемая производительность | Platform foundation |
| 2.10 | Bounded read-path'ы и агрегированные read models | 🔴 Критично | Запланировано | 2.8 | P99 / предсказуемая нагрузка | Platform foundation + admin/read models |
| 2.11 | Distributed rate limiting и строгие runtime guardrails | 🔴 Критично | В работе | 2.5 | Abuse protection / multi-node predictability | Platform foundation + edge/runtime |
| 2.9 | Компонуемые слои развёртывания и пайплайн сборки | 🔵 Стратегически | В работе | — | Гибкость деплоя / корректность сборки | Platform foundation + build/deploy |
| 2.7 | Тонкий `apps/server` как композиционный корень | 🔵 Стратегически | Запланировано | — | DX / стабильность | Platform foundation |
| 2.1 | Вынести `DomainEvent` в `rustok-events` | 🔵 Стратегически | В работе | — | Расширяемость / владение контрактом | Platform foundation |
| 2.12 | Module-owned UI bundles и frontend parity | 🔵 Стратегически | Запланировано | 2.9 | Масштабируемость модулей / dual-stack parity | Domain modules + frontends |
| 2.2 | Типизированные настройки модулей на уровне tenant | 🟢 Улучшение | Бэклог | — | Консистентность / безопасность | Platform foundation + domain modules |
| 2.3 | `rustok-notifications` как optional-модуль | 🟢 Улучшение | Бэклог | 2.2 | Новая capability | Domain modules |
| 2.4 | `core-server` + `module-bundles` | 🔵 Стратегически | Бэклог | 2.7, 2.9 | DX / масштабируемость | Platform foundation |

---

## 5. План по итерациям

### 5.0 Итерация 0 — runtime contract, locale policy и production guardrails

**Треки:** `2.5`, `2.6`, `2.8`, `2.11`

**Scope**
- довести event consumer reliability до platform-wide стандарта;
- закрепить locale negotiation/fallback policy в backend + storefront;
- собрать базовый performance evidence по БД и read-path'ам;
- перевести limiter/transport/guardrail contract в production-готовую, settings-driven модель.

**DoD**
- нет немых деградаций event consumers;
- locale contract документирован и применён минимум в двух пользовательских путях;
- rate limiting и transport degradation имеют явный production contract без hardcoded/soft fallback semantics;
- top-N SQL hot paths и EXPLAIN baseline готовы до следующих БД-изменений.

### 5.1 Итерация 1 — bounded request-path'ы и highload read models

**Треки:** `2.10`

**Scope**
- убрать unbounded dashboard/read path'ы с hot admin/API endpoints;
- перевести тяжелые KPI/activity path'ы на bounded SQL или агрегированные read models;
- ввести bounded reindex/rebuild budgets для read-моделей.

**DoD**
- hot admin/read endpoints не делают полную загрузку event history в память;
- у ключевых read-path'ов есть query/load budget и telemetry;
- reindex/rebuild workers ограничены по concurrency и tenant budget.

### 5.2 Итерация 2 — сборочный контракт и облегчение композиционного корня

**Треки:** `2.9`, `2.7`

**Scope**
- превратить composable deployment model из документа в рабочий build pipeline;
- уменьшить связность `app.rs` и вынести bootstrap тяжёлых подсистем.

**DoD**
- `modules.toml` реально управляет сборкой поддерживаемых deployment profiles;
- invalid build combinations валидируются заранее;
- композиционный корень стал тоньше и покрыт smoke-тестами на init/health/stop.

### 5.3 Итерация 3 — окончательное разведение событийного контракта

**Треки:** `2.1`

**Scope**
- завершить migration к `rustok-events`;
- включить deprecation-path для legacy import paths.

**DoD**
- доменные модули и server-код используют `rustok-events` как канонический вход;
- совместимый слой в `rustok-core` либо явно deprecated, либо ограничен строго переходным окном.

### 5.4 Итерация 4 — типизированные настройки и notification capability

**Треки:** `2.2`, `2.3`

**Scope**
- довести typed module settings до production-ready;
- поверх этого включить notification capability как optional module.

**DoD**
- runtime validation и migration path для settings работают стабильно;
- notifications lifecycle, health и tenant-toggle встроены в общую модель платформы.

### 5.5 Итерация 5 — module UI bundles и внутренние `module-bundles`

**Треки:** `2.12`, `2.4`

**Scope**
- довести module-owned UI bundles до parity минимум для `content`, `commerce`, `forum`, `pages`;
- привязать UI bundle discovery к manifest/registry metadata;
- после стабилизации `2.7` и `2.9` перейти к автоматизации route wiring;
- сначала сделать внутренний bundle-слой без plugin runtime.

**DoD**
- optional modules могут подключать UI surface без ручной правки central nav/import glue-кода;
- parity между `Next` и `Leptos` явно видна и проверяема хотя бы для core optional-модулей;
- route registration optional-модулей не требует ручного glue-кода в центральном server-слое;
- migration risks и rollback path проверены parity/integration тестами.

### 5.6 Фазовый план — замена самописного кода стабильными библиотеками

Этот трек теперь не просто backlog-заметка, а отдельный фазовый план с явными решениями: что реально стоит заменить уже сейчас, а что лучше оставить самописным до стабилизации контракта вокруг подсистемы.

#### 5.6.0 Результат аудита на 2026-03-08

| Область | Решение | Что делаем | Почему |
|---|---|---|---|
| Security validation (`rustok-core`) | `Выполняем частичную замену` | HTML escaping → `v_htmlescape`, email validation → `email_address`, UUID validation → `uuid::Uuid::parse_str`; blacklist-regex для SQL/XSS/command/path пока оставляем | Это low-risk замена primitives без смены platform policy |
| Email helper (`alloy-scripting`) | `Выполняем замену` | Убираем локальную email-проверку в пользу `email_address` | Убираем дублирование и выравниваем семантику с core-слоем |
| SSRF/URL validation (`rustok-core`) | `Пока не заменяем` | Сохраняем кастомную policy поверх `url` | Поведение жёстко связано с allowlist и блокировкой private/localhost адресов; drop-in замена без contract tests слишком рискованна |
| Rate limiting (`rustok-core` + `apps/server`) | `Пока не заменяем` | Не трогаем до унификации контракта rate limit | Сейчас в коде две разные реализации; миграция на `governor`/`tower-governor` без предварительного выравнивания только увеличит связность |
| Retry/circuit-breaker (`rustok-core`) | `Пока не заменяем` | Оставляем текущую реализацию | Подсистема уже встроена в cache/resilience flow и даёт мало выигрыша от срочной замены |
| Frontend debounce hooks (`next-admin`) | `Откладываем` | Возвращаемся после платформенных фаз | Выигрыш косметический, а текущие хуки маленькие и понятные |

Принятые ограничения для выбора библиотек:
- `validator 0.20` не берём в эту фазу, потому что crate требует Rust 1.81, а workspace зафиксирован на `rust-version = 1.80`;
- `ammonia` не берём для `sanitize_html()`, потому что текущий контракт функции ближе к полному escaping текста, а не к allowlist-sanitization HTML.

#### 5.6.1 Фаза A — библиотечные primitives для security/input validation

**Статус на 2026-03-08:** `Выполнено`

**Scope**
- заменить ручную HTML-экранизацию в `rustok-core` на `v_htmlescape`;
- заменить regex/email helper на `email_address` в `rustok-core` и `alloy-scripting`;
- заменить regex UUID-проверку в `rustok-core` на `uuid::Uuid::parse_str`;
- обновить тесты и зафиксировать решение в архитектурной документации.

**DoD**
- больше нет ручной HTML-экранизации и regex UUID/email в целевых точках;
- тесты подтверждают сохранение ожидаемого security baseline;
- документ явно фиксирует, что заменили, а что сознательно оставили самописным.

**Перепроверка**
- `cargo test -p rustok-core test_input_validator_sanitizes_html --test security_audit_test`
- `cargo test -p rustok-core test_uuid_validation --lib`
- `cargo test -p rustok-core test_email_validation --lib`
- `cargo test -p alloy-scripting test_validate_email --lib`

#### 5.6.2 Фаза B — contract-first усиление SSRF policy

**Статус на 2026-03-08:** `Выполнено`

**Scope**
- зафиксировать contract tests на allowlist, localhost, literal IPv4/IPv6, private/link-local/unspecified ranges и redirect chain;
- усилить текущую policy без внешней библиотеки: нормализовать host (`case` + trailing dot), валидировать redirect chain, корректно разбирать literal IPv6 через типизированный host API `url`.

**DoD**
- SSRF policy покрыта тестами как платформенный контракт;
- любое библиотечное усиление не меняет production semantics без явного ADR/документации.

**Результат фазы**
- библиотечную замену (`ipnet`/CIDR-слой) по итогам фазы не делаем;
- текущую policy оставляем самописной, но уже с зафиксированным baseline для IPv4, IPv6 и redirect hops;
- следующая переоценка нужна только если появятся redirect-following HTTP clients или требования к CIDR allow/deny policy.

**Перепроверка**
- `cargo test -p rustok-core test_ssrf_protection_blocks_private_ips --test security_audit_test`
- `cargo test -p rustok-core test_ssrf_protection_allows_safe_urls --test security_audit_test`
- `cargo test -p rustok-core test_ssrf_redirect_chain --test security_audit_test`

#### 5.6.3 Фаза C — унификация rate limiting и resilience stack

**Статус на 2026-03-08:** `Выполнено`

**Scope**
- инвентаризировать различия между `rustok-core` limiter и `apps/server` middleware;
- определить единый HTTP-контракт для server-side limiter и убрать дублирующую реализацию auth rate limit в `app.rs`;
- после этого уже переоценивать `governor`, `tower-governor`, `tower`-middleware и готовые retry/backoff primitives.

**DoD**
- есть одна целевая модель rate limiting для платформы;
- замена библиотекой рассматривается только после фиксации headers, retry semantics и cleanup strategy.

**Результат фазы**
- `rustok-core` limiter остаётся внутренним security primitive и не используется как HTTP middleware;
- `apps/server` сохраняет sliding-window реализацию, но теперь auth и global API rate limiting идут через один и тот же middleware path;
- контракт ответа на limit-exceeded унифицирован: `429`, `Retry-After`, `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`;
- библиотечную замену (`governor` / `tower-governor`) пока не делаем, потому что сначала требовалось стабилизировать поведение, а не менять алгоритм.

**Перепроверка**
- `cargo test -p rustok-server rate_limit`
- `cargo check -p rustok-server --bin rustok-server`

#### 5.6.4 Фаза D — cleanup frontend utilities

**Статус на 2026-03-08:** `Закрыто без изменений`

**Scope**
- оценить, стоит ли менять `useDebounce`/`useDebouncedCallback` на `use-debounce` после завершения платформенных фаз.

**DoD**
- решение принимается по итогам фактической боли сопровождения, а не из желания убрать маленький локальный helper.

**Результат фазы**
- замена на `use-debounce` или `lodash.debounce` признана нецелесообразной;
- текущие hooks малы, прозрачны и используются локально в `next-admin`, в основном через `use-data-table`;
- добавление внешней зависимости сейчас дало бы больше surface area, чем реальной пользы.

**Перепроверка**
- `git grep -n "useDebounce\\|useDebouncedCallback" -- apps/next-admin/src`
- проверить отсутствие новых debounce-зависимостей в `apps/next-admin/package.json`

---

### 5.7 Исполнительный план по фазам на 2026-03-11

**Легенда статусов**
- `[x]` Выполнено
- `[-]` В работе
- `[ ]` Не начато

#### Фаза 0 — уже закрытый foundation groundwork

**Статус:** `Частично выполнено / базовый фундамент собран`

- [x] `2.5.1` Введён runtime-contract и observability baseline для consumer loops.
- [x] `2.5.2` Введён supervised resubscribe для внешних consumer loops.
- [x] `2.5.3` Добавлен readiness signal для внешних consumers.
- [x] Зафиксирован composable deployment contract и `DeploymentProfile`.
- [x] Выровнен admin UI/i18n между `Leptos` и `Next`.
- [x] Выполнены фазы `5.6.1`, `5.6.2`, `5.6.3`; `5.6.4` осознанно закрыта без изменений.

**Что это дало**
- у платформы уже есть базовый operational/runtime фундамент, на который можно безопасно наслаивать highload-улучшения;
- дальнейшие фазы не стартуют с нуля, а продолжают уже оформленный contract.

#### Фаза 1 — production guardrails, locale/runtime contract и performance baseline

**Статус:** `В работе`

- [-] `2.5` Довести transport policy до fail-closed в production и подключить queue/backpressure budgets к runtime.
- [-] `2.6` Подтянуть `tenant.default_locale` и `tenant_locales` в runtime contract.
- [-] `2.6` Убрать расхождение между `backend`, `apps/storefront`, `next-frontend`, `next-admin` по locale negotiation.
- [-] `2.8` Собрать baseline через `pg_stat_statements` и EXPLAIN для подтверждённых hot-path'ов.
- [-] `2.11` Settings-driven policy и Redis-backed distributed backend уже подключены в runtime; остались rollout/observability и guardrail-политика.

**Текущий прогресс в коде**
- `relay_target=iggy` больше не деградирует в `memory` без явного opt-in через settings;
- event bus получил settings-driven channel capacity вместо жёстко зашитого runtime budget;
- `rate_limit` из server settings теперь реально управляет API/auth limiter'ами;
- HTTP limiter умеет работать в `memory|redis` режиме и использует общий Redis runtime модуль вместо локального ad-hoc wiring;
- `tenant.default_locale` протянут в runtime tenant model и используется как fallback в `RequestContext`.

**Критерий завершения фазы**
- production runtime не имеет тихих fallback/degradation path'ов;
- locale-policy одинакова для backend и всех UI-стеков;
- top-N hot paths измерены и задокументированы до начала тяжёлых highload-изменений.

#### Фаза 2 — bounded read-path'ы и highload read models

**Статус:** `Запланировано`

- [ ] `2.10` Убрать полную загрузку `sys_events` и аналогичные unbounded read-path'ы из hot admin/API endpoints.
- [ ] `2.10` Вынести dashboard KPI, recent activity и похожие срезы в bounded SQL или агрегированные read models.
- [ ] `2.10` Перевести pagination/filter logic forum/admin на обязательный DB-side contract.
- [ ] `2.10` Ввести bounded parallelism и tenant budgets для reindex/rebuild workers.

**Критерий завершения фазы**
- hot read endpoints имеют измеримый budget по rows/query/latency;
- heavy read models не строятся через full scan и in-memory aggregation на пользовательском path'е.

#### Фаза 3 — тонкий composition root и реальный build contract

**Статус:** `Запланировано`

- [ ] `2.7` Вынести GraphQL schema/runtime factories из request path в boot-time/shared handles.
- [ ] `2.7` Разгрузить `apps/server/src/app.rs` до уровня wiring + orchestration.
- [ ] `2.9` Реализовать реальные `embed-admin` / `embed-storefront` feature flags и сборку из `modules.toml`.
- [ ] `2.9` Встроить smoke-validation для `monolith`, `server+admin`, `headless-api`.

**Критерий завершения фазы**
- server startup стал предсказуемым и тестируемым;
- deployment profiles управляются не только документацией, но и фактическим build pipeline.

#### Фаза 4 — завершение событийного и settings-контракта

**Статус:** `Запланировано`

- [ ] `2.1` Завершить перенос канонического event-контракта в `rustok-events`.
- [ ] `2.2` Ввести typed settings с versioned schema, defaults и upgrade hooks.
- [ ] `2.3` Поверх typed settings довести notifications до полноценного optional capability.

**Критерий завершения фазы**
- event ownership больше не размазан по `rustok-core`;
- tenant/module settings безопасно эволюционируют без сырого JSON как runtime-контракта.

#### Фаза 5 — module-owned UI bundles и platform modularity

**Статус:** `Запланировано`

- [ ] `2.12` Довести `content`, `commerce`, `forum`, `pages` до parity по UI surface между `Next` и `Leptos`.
- [ ] `2.12` Привязать bundle discovery к registry/manifest metadata.
- [ ] `2.4` После стабилизации `2.7` и `2.9` перейти к внутренним `module-bundles` без ручного glue-кода.

**Критерий завершения фазы**
- optional module подключает backend + UI surface без ручной правки центральных приложений;
- масштабирование платформы идёт через модульные контракты, а не через рост центрального glue-кода.

#### Фаза 6 — highload hardening после baseline

**Статус:** `Запланировано`

- [ ] На основе результатов `2.8` принять решения по `rewrite / cache / read model / partitioning` для конкретных bottleneck'ов.
- [ ] Ввести operator-facing budgets для reindex, replay и background rebuild jobs.
- [ ] Согласовать SLO/SLA для `P95/P99`, backlog saturation и multi-tenant abuse protection.

**Критерий завершения фазы**
- highload-архитектура растёт по измерениям, а не по предположениям;
- у платформы есть понятный путь от текущего monolith/runtime к multi-tenant highload без резкого replatforming.

---

## 6. Связанные документы

- [`docs/architecture/overview.md`](./overview.md) — архитектурный обзор
- [`docs/architecture/events.md`](./events.md) — событийная модель и outbox
- [`docs/architecture/i18n.md`](./i18n.md) — текущая i18n-архитектура, требующая синхронизации с новым platform contract
- [`docs/modules/manifest.md`](../modules/manifest.md) — актуальный формат `modules.toml`
- [`docs/modules/registry.md`](../modules/registry.md) — реестр модулей и границы ответственности
- [`DECISIONS/2026-02-19-rustok-events-canonical-contract.md`](../../DECISIONS/2026-02-19-rustok-events-canonical-contract.md) — ADR по событийному контракту
- [`DECISIONS/2026-02-19-core-server-module-bundles-routing.md`](../../DECISIONS/2026-02-19-core-server-module-bundles-routing.md) — ADR по `core-server` и bundles
- [`DECISIONS/2026-03-07-deployment-profiles-and-ui-stack.md`](../../DECISIONS/2026-03-07-deployment-profiles-and-ui-stack.md) — ADR по composable deployment layers
- [`DECISIONS/2026-03-07-admin-module-ui-unification.md`](../../DECISIONS/2026-03-07-admin-module-ui-unification.md) — ADR по унификации admin UI и i18n
