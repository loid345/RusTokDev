# RusToK — рекомендации по развитию архитектуры

- Дата: 2026-02-19
- Статус: Живой документ
- Последнее обновление: 2026-03-08
- Основание обновления: учтены изменения по состоянию кода и ADR от 2026-03-07
- Автор: Архитектурное ревью платформы

---

## 1. Состояние на 2026-03-08

Этот документ больше не пытается пересказывать всю архитектуру платформы. Его задача — держать в одном месте только открытые архитектурные треки, их приоритеты и зависимости между ними.

### 1.1 Что уже считаем базовым инвариантом

Ниже — вещи, которые больше не ведём как отдельный backlog в этом документе:

- граница между `Core` и `Optional` модулями зафиксирована и отражена в registry;
- `rustok-index`, `rustok-tenant`, `rustok-rbac` работают как обязательные core-модули;
- `rustok-outbox` признан критическим инфраструктурным слоем write-path и не рассматривается как optional capability;
- манифест модулей (`modules.toml`) уже перешёл на composable-модель слоёв `server/admin/storefront`, а `DeploymentProfile` уже вычисляется из `[build.server]`;
- унификация admin UI между Leptos и Next.js принята и реализована: обе админки уже используют библиотечный i18n (`leptos_i18n` / `next-intl`) и синхронизированные locale-файлы.

### 1.2 Что ещё остаётся источником архитектурного риска

- Надёжность event consumers не доведена до платформенного стандарта: базовая обработка `Lagged/Closed` есть в dispatcher, но нет единого операционного контура, метрик и runbook'ов для всех потребителей.
- Политика локалей по-прежнему фрагментирована: admin-слой продвинулся, но backend/storefront/request negotiation/fallback ещё не унифицированы.
- `apps/server/src/app.rs::after_routes()` остаётся перегруженным композиционным корнем с ручной инициализацией тяжёлых подсистем.
- Событийный контракт всё ещё живёт в `rustok-core`; `rustok-events` пока является фазой совместимого re-export, а не финальным центром владения контрактом.
- Typed per-tenant settings не доведены до production-ready runtime-контракта; в ряде мест платформа по-прежнему опирается на сырые JSON-настройки.
- Новый composable deployment contract частично вошёл в код и документы, но build pipeline, Cargo features и `rustok rebuild` ещё не доведены до полного production-пути.

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

**Статус на 2026-03-08:** `В работе`

**Почему задача всё ещё открыта:**
- в `rustok-core` dispatcher уже обрабатывает `Lagged/Closed`, но это пока только базовый защитный слой;
- нет платформенного контракта на restart/resubscribe policy для всех consumer loops;
- нет минимального операционного набора метрик и runbook'ов для partial/full reindex после инцидентов.

**Ближайший scope:**
- инвентаризировать все consumer loops и выровнять реакцию на `Lagged/Closed`;
- добавить минимальные метрики: `event_consumer_lagged_total`, `event_consumer_restarted_total`, `event_dispatch_latency_ms`;
- оформить reindex runbook для read-моделей (`partial` vs `full`).

**Критерии готовности:**
- в платформе не остаётся consumer loop'ов с немой деградацией или тихой остановкой;
- `/metrics` показывает минимальный набор сигналов по event delivery degradation;
- runbook документирует, когда достаточно partial reindex, а когда нужен full rebuild read-моделей.

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

**Статус на 2026-03-08:** `В работе`

**Почему задача всё ещё открыта:**
- admin UI уже выровнен, но backend и storefront до сих пор не живут по одному negotiation contract;
- `RequestContext` пока использует упрощённый `Accept-Language` parsing, а не платформенную policy;
- документ `docs/architecture/i18n.md` отстаёт от новой реальности с несколькими UI-стеками и composable deployment model.

**Ближайший scope:**
- закрепить единую policy: `URL locale -> cookie -> Accept-Language -> tenant default`;
- формализовать fallback цепочку контента: `requested -> tenant.fallback -> tenant.default -> en`;
- расширить допустимую длину `locale` как минимум до 16 символов для BCP47-подобных тегов;
- синхронизировать `docs/architecture/i18n.md` с фактическим runtime contract.

**Критерии готовности:**
- backend, admin и storefront используют одинаковую семантику locale negotiation;
- API и UI явно различают `requested_locale` и `effective_locale`, если сработал fallback;
- схему хранения locale можно безопасно использовать для BCP47-подобных тегов без точечных исключений.

### 2.7 Тонкий `apps/server` как композиционный корень

**Статус на 2026-03-08:** `Запланировано`

**Почему задача всё ещё открыта:**
- `after_routes()` по-прежнему смешивает routing, lifecycle, background workers, event runtime, rate limiting, Alloy и UI wiring;
- любое изменение старта приложения повышает риск регрессий и мешает следующему шагу — 2.4.

**Ближайший scope:**
- вынести тяжёлые подсистемы в отдельные init-компоненты внутри текущего процесса;
- оставить в composition root только wiring: routing, middleware, registration и orchestration;
- покрыть init/health/stop интеграционными smoke-тестами.

**Критерии готовности:**
- server startup раскладывается на понятные bootstrap-компоненты;
- `app.rs` перестаёт быть местом прямой бизнес-инициализации подсистем;
- lifecycle ключевых подсистем проверяется smoke-тестами, а не только ручным прогоном.

### 2.8 Масштабирование БД только по evidence

**Статус на 2026-03-08:** `Запланировано`

**Почему задача всё ещё открыта:**
- в системе уже есть hot-path'ы (`outbox`, read models, event-related queries), но решения уровня partitioning пока не подтверждены метриками;
- нужен baseline, который не будет подменять инженерные данные предположениями.

**Ближайший scope:**
- провести аудит индексов на hot-path запросах;
- собрать baseline через `pg_stat_statements` и сохранить EXPLAIN-планы для top-N запросов;
- подготовить partition-ready дизайн без немедленного включения в production.

**Критерии готовности:**
- top-N SQL hot paths известны и документированы;
- для целевых запросов есть метрики до/после и план индексации;
- тяжёлые схемные изменения запускаются только после подтверждённого bottleneck.

### 2.9 Компонуемые слои развёртывания и пайплайн сборки

**Статус на 2026-03-08:** `В работе`

**Почему задача всё ещё открыта:**
- manifest contract и `DeploymentProfile` уже вошли в код и документацию;
- но `apps/server/Cargo.toml` пока не содержит обещанные `embed-admin` / `embed-storefront` features;
- build-service и `rustok rebuild` ещё не стали полным production path для всех профилей из ADR `2026-03-07-deployment-profiles-and-ui-stack.md`.

**Ближайший scope:**
- реализовать Cargo features для встраивания admin/storefront артефактов;
- научить build pipeline и build-service собирать команды из `modules.toml`;
- добавить validation для несовместимых комбинаций стека и embedding;
- ввести smoke-check'и для минимум трёх конфигураций: `monolith`, `server+admin`, `headless-api`.

**Критерии готовности:**
- одна и та же manifest-модель реально управляет сборкой артефактов, а не только документирует их;
- build-service и `rustok rebuild` воспроизводимо собирают поддерживаемые deployment profiles;
- invalid configs отсекаются до начала долгой сборки;
- ключевые варианты деплоя покрыты smoke/integration проверками.

---

## 3. Приоритеты и зависимости

### 3.1 Что имеет смысл делать прямо сейчас

- `2.5 EventBus consumers` — это самый дешёвый способ снизить риск `silent desync`;
- `2.6 Единая политика локалей` — уже частично развернута на admin-слое, теперь важно дотянуть платформенный contract;
- `2.8 Evidence-driven DB baseline` — нужен до любых тяжёлых изменений схемы;
- `2.9 Deployment layers и build pipeline` — нужен, чтобы ADR от 2026-03-07 перестал быть только документированным дизайном.

### 3.2 Что логически зависит от предыдущих треков

- `2.4 core-server + module-bundles` зависит от `2.7` и `2.9`;
- `2.3 rustok-notifications` зависит от `2.2`;
- `2.1 DomainEvent extraction` можно вести параллельно, но без требования срочно перепахивать runtime;
- `2.8` не блокирует другие треки, но должен предшествовать крупным БД-решениям.

### 3.3 Что сознательно не форсируем

- переход на новый event broker по умолчанию без подтверждённой эксплуатационной необходимости;
- большой replatforming всего UI за один релиз;
- partitioning и другие дорогие схемные изменения без метрик и EXPLAIN;
- plugin-ready внешние bundle-механизмы до стабилизации внутреннего bundle-слоя.

---

## 4. Приоритизированный план действий

| ID | Трек | Приоритет | Статус | Зависимости | Ценность | Зона ответственности |
|---|---|---|---|---|---|---|
| 2.5 | Надёжность EventBus consumers | 🔴 Критично | В работе | — | Надёжность / консистентность | Platform foundation + index |
| 2.6 | Единая политика локалей | 🔴 Критично | В работе | — | UX / SEO-консистентность | Platform foundation + frontends + content |
| 2.8 | Evidence-driven DB baseline | 🔴 Критично | Запланировано | — | Предсказуемая производительность | Platform foundation |
| 2.9 | Компонуемые слои развёртывания и пайплайн сборки | 🔵 Стратегически | В работе | — | Гибкость деплоя / корректность сборки | Platform foundation + build/deploy |
| 2.7 | Тонкий `apps/server` как композиционный корень | 🔵 Стратегически | Запланировано | — | DX / стабильность | Platform foundation |
| 2.1 | Вынести `DomainEvent` в `rustok-events` | 🔵 Стратегически | В работе | — | Расширяемость / владение контрактом | Platform foundation |
| 2.2 | Типизированные настройки модулей на уровне tenant | 🟢 Улучшение | Бэклог | — | Консистентность / безопасность | Platform foundation + domain modules |
| 2.3 | `rustok-notifications` как optional-модуль | 🟢 Улучшение | Бэклог | 2.2 | Новая capability | Domain modules |
| 2.4 | `core-server` + `module-bundles` | 🔵 Стратегически | Бэклог | 2.7, 2.9 | DX / масштабируемость | Platform foundation |

---

## 5. План по итерациям

### 5.0 Итерация 0 — защитное усиление и единый runtime contract

**Треки:** `2.5`, `2.6`, `2.8`

**Scope**
- довести event consumer reliability до platform-wide стандарта;
- закрепить locale negotiation/fallback policy в backend + storefront;
- собрать базовый performance evidence по БД и read-path'ам.

**DoD**
- нет немых деградаций event consumers;
- locale contract документирован и применён минимум в двух пользовательских путях;
- top-N SQL hot paths и EXPLAIN baseline готовы до следующих БД-изменений.

### 5.1 Итерация 1 — сборочный контракт и облегчение композиционного корня

**Треки:** `2.9`, `2.7`

**Scope**
- превратить composable deployment model из документа в рабочий build pipeline;
- уменьшить связность `app.rs` и вынести bootstrap тяжёлых подсистем.

**DoD**
- `modules.toml` реально управляет сборкой поддерживаемых deployment profiles;
- invalid build combinations валидируются заранее;
- композиционный корень стал тоньше и покрыт smoke-тестами на init/health/stop.

### 5.2 Итерация 2 — окончательное разведение событийного контракта

**Треки:** `2.1`

**Scope**
- завершить migration к `rustok-events`;
- включить deprecation-path для legacy import paths.

**DoD**
- доменные модули и server-код используют `rustok-events` как канонический вход;
- совместимый слой в `rustok-core` либо явно deprecated, либо ограничен строго переходным окном.

### 5.3 Итерация 3 — типизированные настройки и notification capability

**Треки:** `2.2`, `2.3`

**Scope**
- довести typed module settings до production-ready;
- поверх этого включить notification capability как optional module.

**DoD**
- runtime validation и migration path для settings работают стабильно;
- notifications lifecycle, health и tenant-toggle встроены в общую модель платформы.

### 5.4 Итерация 4 — `core-server` и внутренние `module-bundles`

**Треки:** `2.4`

**Scope**
- после стабилизации `2.7` и `2.9` перейти к автоматизации route wiring;
- сначала сделать внутренний bundle-слой без plugin runtime.

**DoD**
- route registration optional-модулей не требует ручного glue-кода в центральном server-слое;
- migration risks и rollback path проверены parity/integration тестами.

### 5.5 Фазовый план — замена самописного кода стабильными библиотеками

Этот трек теперь не просто backlog-заметка, а отдельный фазовый план с явными решениями: что реально стоит заменить уже сейчас, а что лучше оставить самописным до стабилизации контракта вокруг подсистемы.

#### 5.5.0 Результат аудита на 2026-03-08

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

#### 5.5.1 Фаза A — библиотечные primitives для security/input validation

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

#### 5.5.2 Фаза B — contract-first усиление SSRF policy

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

#### 5.5.3 Фаза C — унификация rate limiting и resilience stack

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

#### 5.5.4 Фаза D — cleanup frontend utilities

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
