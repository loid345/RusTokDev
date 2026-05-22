# Аудит документации RusTok

## Executive summary

Репозиторий уже выглядит как проект, который **пытается держать документацию рядом с кодом**, а не сваливать всё в один README: есть центральная карта `docs/index.md`, локальные docs у приложений и crate-ов, отдельный `xtask` для валидации модульного контракта и CI, который уже гоняет `cargo doc`, `typos`, `xtask validate-manifest` и `cargo xtask module validate`. Это сильная база, и она лучше среднего по open-source Rust-репозиториям. fileciteturn22file0L1-L3 fileciteturn39file0L1-L3 fileciteturn69file0L1-L3

Но качество документации как **источника истины для правильного написания кода** сейчас не дотягивает до “идеального” состояния. Основная проблема не в отсутствии файлов, а в том, что доки местами **дрейфуют относительно кода**, дублируют друг друга, содержат старые ссылки, старые версии, расползающуюся терминологию и вручную поддерживаемые API-описания, которые уже не соответствуют реальным сигнатурам. Самые критичные точки: корневые README/CONTRIBUTING, центральная навигация `docs/index.md`, ручные `CRATE_API.md`, CHANGELOG, а также несогласованность реестров модулей с `modules.toml`. fileciteturn23file0L1-L3 fileciteturn30file0L1-L3 fileciteturn47file0L1-L3 fileciteturn62file0L1-L3 fileciteturn37file0L1-L3 fileciteturn40file0L1-L3

Если резюмировать прагматично: **исправлять надо не “ещё больше писать”, а сначала “свести систему к одному источнику правды”**. Я бы начал с P0-пакета из четырёх работ: санировать корневые документы, вычистить дрейфующие ссылки и версии, убрать ручной API-drift, починить CHANGELOG и запустить автоматические проверки Markdown/ссылок/примеров. Это даст самый большой выигрыш именно для разработчика, который приходит в репозиторий и хочет понять, как писать код без ловушек. fileciteturn23file0L1-L3 fileciteturn30file0L1-L3 fileciteturn47file0L1-L3 fileciteturn69file0L1-L3

Ниже — конкретный разбор, готовые правки, CI-конфиги, шаблоны и дорожная карта.

## Что было проверено

Через GitHub connector я прошёл по документированному контуру репозитория: корневые `README.md` и `README.ru.md`, `docs/index.md`, гайды и стандарты, `docs/modules/*`, `xtask/README.md`, `CONTRIBUTING.md`, `AGENTS.md`, `CHANGELOG.md`, README/docs приложений, выборочно `CRATE_API.md`, ключевые shell-скрипты и CI. Для сопоставления с кодом я также проверил `Cargo.toml`, `modules.toml`, `.github/workflows/ci.yml`, отдельные исходники и бенчмарки, а также искал inline docstrings и scattered examples. fileciteturn23file0L1-L3 fileciteturn59file0L1-L3 fileciteturn22file0L1-L3 fileciteturn38file0L1-L3 fileciteturn39file0L1-L3 fileciteturn69file0L1-L3

Картина по структуре такая: у проекта есть сильная центральная договорённость, что `docs/index.md` — каноническая точка входа, `docs/` — платформа целиком, локальные docs живут рядом с компонентами, а `xtask` валидирует модульный контракт и синхронизацию docs с кодом. Это архитектурно правильно. Проблема в том, что часть более старых или периферийных документов живёт уже по другим правилам и не всегда успевает за этой системой. fileciteturn22file0L1-L3 fileciteturn29file0L1-L3 fileciteturn39file0L1-L3

Ниже — краткая карта того, что сейчас есть и где слабые места.

| Зона | Что есть | Что хорошо | Что ломает доверие |
|---|---|---|---|
| Корневые документы | `README.md`, `README.ru.md`, `CONTRIBUTING.md`, `CHANGELOG.md`, `AGENTS.md` | Есть явные entrypoints и contributor policy. fileciteturn23file0L1-L3 fileciteturn29file0L1-L3 fileciteturn30file0L1-L3 | Старые org/repo URLs, старые версии Rust, битая кодировка, мёртвые ссылки и устаревшие инструкции. fileciteturn23file0L1-L3 fileciteturn59file0L1-L3 fileciteturn62file0L1-L3 |
| Центральная навигация | `docs/index.md`, `docs/modules/registry.md`, `docs/modules/manifest.md`, `docs/modules/UI_PACKAGES_INDEX.md` | Есть реальный documentation map и модульный реестр. fileciteturn22file0L1-L3 fileciteturn40file0L1-L3 fileciteturn38file0L1-L3 | `docs/index.md` перегружен volatile-статусом и плохо работает как карта; в реестрах есть drift относительно `modules.toml`. fileciteturn22file0L1-L3 fileciteturn40file0L1-L3 fileciteturn37file0L1-L3 |
| Локальные docs приложений | `apps/*/docs/README.md` | Например, `apps/admin/docs/README.md` уже очень контрактно и подробно описывает runtime split. fileciteturn41file0L1-L3 | Эта детализация не всегда протянута в корневые документы и contributor docs. fileciteturn30file0L1-L3 fileciteturn41file0L1-L3 |
| API/reference | ручные `CRATE_API.md`, rustdoc через `cargo doc`, OpenAPI/GraphQL surfaces в коде | Есть попытка документировать public API специально. fileciteturn47file0L1-L3 fileciteturn69file0L1-L3 | Ручные `CRATE_API.md` дрейфуют от кода. fileciteturn47file0L1-L3 fileciteturn48file0L1-L3 fileciteturn49file0L1-L3 fileciteturn51file0L1-L3 fileciteturn53file0L1-L3 |
| CI для доков | `cargo doc`, `typos`, freshness checks для snapshots, `xtask validate-manifest/module validate` | Уже есть ядро quality gates. fileciteturn69file0L1-L3 | Нет markdownlint, linkcheck, проверки примеров, проверки code blocks, отдельной сборки docs site. fileciteturn69file0L1-L3 |
| Примеры | scattered: README, quickstart, shell scripts, отдельные docs `instrumentation-examples.md` | Примеры вообще есть. fileciteturn27file0L1-L3 fileciteturn28file0L1-L3 fileciteturn21file0L1-L3 | Централизованного каталога examples не видно; discoverability слабая. fileciteturn21file0L1-L3 |

Наблюдение по локализации: в политике проекта прямо сказано, что центральная документация в `docs/` должна быть на русском и “один файл — один язык”, но внутри `docs/` есть документы целиком на английском, например `docs/standards/performance.md` и `docs/guides/testing.md`. С точки зрения пользователя языка это не катастрофа, но это ломает консистентность, полнотекстовый поиск и предсказуемость навигации. fileciteturn29file0L1-L3 fileciteturn22file0L1-L3 fileciteturn24file0L1-L3 fileciteturn46file0L1-L3

## Главные несоответствия и косяки

### Корневые документы и onboarding

Самый заметный класс проблем — **старые entrypoints, которые разработчик видит первыми**. И в `README.md`, и в `README.ru.md` badge CI и ссылки still указывают на старый org/repo `RustokCMS/RusToK`, а badge Rust продолжает обещать `1.80+`. При этом сам workspace фиксирует `rust-version = "1.85"`, а CI отдельно проверяет MSRV как `1.85.0`. Для разработчика это прямой источник ложных ожиданий про совместимость toolchain. fileciteturn23file0L1-L3 fileciteturn59file0L1-L3 fileciteturn25file0L1-L3 fileciteturn69file0L1-L3

`CONTRIBUTING.md` отстаёт ещё сильнее. Там до сих пор clone URL на старый репозиторий, prerequisites “Rust 1.80 or higher”, описание `apps/admin` как “Leptos CSR admin panel”, вручной setup через старые `cargo loco db install/migrate`, ссылки на `docs/testing-guidelines.md`, `DOCS_MAP.md` и `QUICKSTART.md`, которые в проверенном контуре не обнаруживаются, а также предложение “`mdbook build`”, для которого в репозитории не видно book-конфигурации. Это уже не косметика, а вводящая в заблуждение документация для разработчика, который реально хочет поднять проект локально. fileciteturn30file0L1-L3 fileciteturn41file0L1-L3 fileciteturn31file0L1-L3 fileciteturn32file0L1-L3 fileciteturn34file0L1-L3

Отдельно критично, что русский корневой README частично повреждён по кодировке: в нём есть видимый mojibake в заголовках и ссылках вроде “РљСЂР°С‚РєР°СЏ...” и секциях “РџСЂРёР»РѕР¶РµРЅРёСЏ”, “Р‘С‹СЃС‚СЂС‹Р№ СЃС‚Р°СЂС‚”. Это не просто эстетика: такой документ тяжело искать, читать, цитировать и редактировать без риска дальнейших ошибок. Аналогичный мусор есть и в `CHANGELOG.md`. fileciteturn59file0L1-L3 fileciteturn62file0L1-L3

### Центральная карта документации и навигация

`docs/index.md` декларируется как “каноническая точка входа”, но фактически совмещает сразу три роли: navigation map, architecture digest и rolling status-board по множеству фаз и модулей. Внутри карты документации сидят гигантские volatile-абзацы про pricing, SEO, cart promotions, fulfillment, tax snapshot, multivendor и так далее. Из-за этого карта перестаёт быть картой: она слишком длинная, быстро стареет и сложно diff-ится. Для новой команды это не entrypoint, а перегруженный поток статусов. fileciteturn22file0L1-L3

В `docs/modules/registry.md` тоже виден drift относительно `modules.toml`: в архитектурной диаграмме и optional-модульной карте фигурирует `tax`, а в корневом `modules.toml` такого платформенного модуля нет; при этом crate `rustok-tax` в workspace есть, то есть сейчас документация смешивает “существует crate” и “входит в платформенный manifest”. Это ломает главный контракт, который сам `xtask` пытается защищать: модуль считается платформенным не по наличию crate, а по присутствию в `modules.toml`. fileciteturn40file0L1-L3 fileciteturn37file0L1-L3 fileciteturn39file0L1-L3 fileciteturn60file0L1-L3

Есть и менее явный, но показательнейший пример дрейфа терминологии: корневой README перечисляет capability/runtime layers и упоминает `alloy-scripting`, но репозиторный поиск через connector даёт срабатывания только в самих README, без отдельного документационного или кодового источника истины для такого элемента. Для внешнего читателя это выглядит как часть платформы, хотя по документированному контуру неясно, что это именно — crate, legacy label или stale термин. fileciteturn23file0L1-L3 fileciteturn61file0L1-L3

Наконец, есть чисто formatting/hygiene-дефекты: например, в `.github/` присутствуют оба файла шаблона PR — `PULL_REQUEST_TEMPLATE.md` и `pull_request_template.md`, а в `docs/modules/UI_PACKAGES_INDEX.md` секция `### Next.js admin showcase` стоит уже после блока “Связанные документы”, что делает иерархию Markdown менее предсказуемой. Это не главные баги, но именно такие вещи создают ощущение “документация расползается”. fileciteturn21file29L1-L3 fileciteturn21file30L1-L3 fileciteturn68file0L1-L3

### Drift между документацией, кодом и тестами

Самый опасный для разработки класс проблем — **ручная API-документация, ушедшая от реальных сигнатур**. Очень показателен `crates/rustok-workflow/CRATE_API.md`. Документ утверждает, что `WorkflowTriggerHandler::new` принимает `db` и `Arc<WorkflowEngine>`, но текущий код имеет `pub fn new(db: DatabaseConnection) -> Self` и сам создаёт engine/service внутри. Документ также описывает `WorkflowEngine::with_step(...)` и `execute(&self, workflow_id, trigger_event_id, context)`, тогда как текущий engine даёт `register_step(...)` и `execute(&self, workflow_id, tenant_id, trigger_event_id, steps, initial_context)`. Ошибочно описан и `WorkflowError`: в коде есть `StepNotFound`, `ExecutionNotFound`, `NotActive`, `UnknownStepType`, `InvalidTriggerConfig`, `InvalidStepConfig`, а manual API file этого не отражает. Это типичный случай, когда ручной reference начинает мешать больше, чем помогать. fileciteturn47file0L1-L3 fileciteturn49file0L1-L3 fileciteturn51file0L1-L3 fileciteturn53file0L1-L3

`WorkflowService` тоже шире и богаче, чем обещает `CRATE_API.md`. В коде есть, например, `load_steps`, `record_failure`, `reset_failure_count`, явные tenant-scoped execution query methods и template-based creation flow, а manual reference этого не нормализует в единый public contract. Следствие: разработчик не знает, что считать стабильным API, а что внутренней деталью. fileciteturn47file0L1-L3 fileciteturn52file0L1-L3

Ещё один хороший пример — performance docs. `docs/standards/performance.md` приводит example results вроде `order_queries/is_paid_paid` и другие benchmark labels, но реальный benchmark-файл `benches/benches/order_operations.rs` объявляет группы и функции `get_id_pending`, `get_total_pending`, `get_total_paid`, `complete_order_flow`, `cancellation_flow`, `payment_flow` и т.д. То есть даже если бенчмарк-система существует, приведённые в документации названия результатов уже не совпадают с реальным suite. Для performance-доков это особенно плохо, потому что воспроизводимость — вся их ценность. fileciteturn24file0L1-L3 fileciteturn45file0L1-L3

Та же история видна на уровне API map. `docs/architecture/api.md` в канонических health endpoints перечисляет только `/health`, `/health/live` и `/health/ready`, тогда как реальный контроллер публикует ещё `/health/runtime` и `/health/modules`. Нельзя сказать, что док “совсем неправильный”, но он уже неполный относительно фактической поверхности. Для operational/documentation surface это тоже drift. fileciteturn65file0L1-L3 fileciteturn67file0L1-L3

### Установка, профили запуска и безопасность примеров

Quickstart в целом полезнее и современнее старых документов: он описывает `./scripts/dev-start.sh`, installer preflight/plan/apply, `cargo xtask install-dev --create-db` и разделяет Docker-путь и non-Docker bootstrap. Скрипт `scripts/dev-start.sh` действительно поднимает те сервисы и порты, которые quickstart обещает. Это плюс. fileciteturn27file0L1-L3 fileciteturn28file0L1-L3

Но тут есть важная двусмысленность. В `modules.toml` зашиты manifest-level `public_url` для admin/storefront, где `build.admin.public_url = http://localhost:3001`, а `build.storefront.public_url = http://localhost:3000`; в то же время quickstart и dev script описывают на `3000` Next.js admin, а storefront как `3100/3101`. Возможно, это разные deployment profiles, и в release/monolith это сознательно так. Но в документации это не разъяснено как многопрофильность — для читателя получается как минимум запутанный “источник истины” про адреса и ownership UI-host’ов. fileciteturn37file0L1-L3 fileciteturn27file0L1-L3 fileciteturn28file0L1-L3

По безопасности я не увидел в inspected docs явной утечки production-secrets. Зато есть много **plaintext dev credentials and DSNs**: `admin@local / admin12345`, `postgres://postgres:postgres@localhost...`, разные локальные password examples. Для dev это допустимо, но сейчас такие примеры разбросаны по quickstart, Makefile, CONTRIBUTING и shell scripts без единого визуального маркера “DEV ONLY”. В идеальном состоянии это нужно стандартизовать: dev-примеры оставить, но все non-dev snippets перевести на placeholders и `*-secret-ref` как primary path. fileciteturn27file0L1-L3 fileciteturn28file0L1-L3 fileciteturn30file0L1-L3 fileciteturn34file0L1-L3

### CHANGELOG, история изменений и discoverability

`CHANGELOG.md` сейчас, пожалуй, самый технически небезопасный для доверия документ после корневых onboarding-файлов. В нём есть mojibake, длинные sprint-style narrative blocks, тонны ссылок на файлы вида `docs/BENCHMARKS_GUIDE.md`, `docs/SPRINT_1_COMPLETION.md`, `docs/SECURITY_AUDIT_GUIDE.md`, которые в проверенном контуре не поднимаются как реальные документы, и mix из changelog/roadmap/marketing metrics. Такой changelog нельзя использовать ни как release log, ни как trustworthy change history. fileciteturn62file0L1-L3 fileciteturn63file0L1-L3 fileciteturn64file0L1-L3

Кроме того, примеры использования и reference-материалы разбросаны по разным слоям: README, quickstart, `instrumentation-examples.md`, shell scripts, app docs, package docs. Репозиторий не выглядит как совсем безпримерный, но **discoverability examples** низкая: централизованного `examples/` или хотя бы “Examples index” я в документированном контуре не увидел. Для разработчика это означает, что искать рабочий sample приходится эвристически. fileciteturn21file0L1-L3 fileciteturn21file3L1-L3 fileciteturn27file0L1-L3

## Целевое состояние документации

Идеальное состояние для RusTok — это не “ещё 200 Markdown-файлов”, а **трёхслойная система истины**:

1. **Stable entry docs** — корневые README/CONTRIBUTING/AGENTS и короткий `docs/index.md`, которые почти не меняются и дают правильную навигацию.
2. **Contract docs** — `docs/modules/manifest.md`, `docs/modules/registry.md`, app/module `docs/README.md`, где описано, что является контрактом, а что нет.
3. **Generated reference** — Rust API из rustdoc, REST из OpenAPI, GraphQL schema/reference из live schema export, TypeScript API для Next/shared packages из автогенерации, а не из ручных `CRATE_API.md`.

Сейчас в репозитории уже есть основа именно для такого подхода: модульный manifest, xtask, CI и локальные app docs. То есть не нужен радикальный “переезд на новую док-систему”; нужен **refactor documentation architecture**, где narrative и generated reference перестают подменять друг друга. fileciteturn38file0L1-L3 fileciteturn39file0L1-L3 fileciteturn69file0L1-L3

Ниже — целевая схема.

```mermaid
flowchart TD
    A[Root README / CONTRIBUTING / AGENTS] --> B[docs/index.md как короткая карта]
    B --> C[Архитектура и правила]
    B --> D[App docs]
    B --> E[Module docs]
    B --> F[Generated reference]

    C --> C1[docs/architecture/*]
    C --> C2[docs/standards/*]
    C --> C3[docs/guides/*]

    D --> D1[apps/server/docs/*]
    D --> D2[apps/admin/docs/*]
    D --> D3[apps/storefront/docs/*]
    D --> D4[apps/next-*/docs/*]

    E --> E1[README.md module]
    E --> E2[docs/README.md module]
    E --> E3[docs/implementation-plan.md module]
    E --> E4[rustok-module.toml]

    F --> F1[cargo doc / rustdoc]
    F --> F2[OpenAPI artifacts]
    F --> F3[GraphQL schema export]
    F --> F4[TypeDoc for TS packages]
```

Практическое правило для RusTok я бы формализовал так:

| Тип информации | Где живёт | Как обновляется | Что считается источником истины |
|---|---|---|---|
| Как начать работу | `README.md`, `README.ru.md`, `CONTRIBUTING.md`, `docs/guides/quickstart.md` | вручную, но под линт и linkcheck | quickstart + root docs |
| Архитектурные инварианты | `docs/architecture/*`, `docs/modules/*`, app docs | вручную, через PR review | contract docs |
| Публичные API Rust | rustdoc + curated overview page | автоматически из кода | код |
| REST surface | OpenAPI + curated guide | автоматически из кода | код |
| GraphQL surface | schema export + curated guide | автоматически из кода | код |
| Модульный контракт | `modules.toml`, `rustok-module.toml`, `xtask` | автоматически валидируется | manifest + code |
| История релизов | `CHANGELOG.md` | вручную, но строго по шаблону | changelog only |
| Рабочие примеры | `examples/` или `docs/examples/*` + smoke tests | полуавтоматически | executable examples |

Два принципа, без которых “идеального состояния” не будет:

| Принцип | Что это значит для RusTok |
|---|---|
| **Документация не дублирует сигнатуры руками** | никаких ручных `pub fn ...` списков в Markdown, если их можно взять из rustdoc/OpenAPI/schema export |
| **Навигация не содержит volatile статусов** | `docs/index.md` не должен быть roadmap/phase log; для этого нужны `implementation-plan.md`, ADR и отдельные remediation docs |

## Приоритетный план работ

### Очередь задач

Оценки ниже — **чистые инженерные часы**, без ожидания code review.

| Приоритет | Задача | Трудоёмкость | Зависимости | Критерий приёмки |
|---|---|---:|---|---|
| P0 | Санировать root docs: README/README.ru/CONTRIBUTING/CHANGELOG | 14 ч | нет | нет старых org/repo URLs, нет mojibake, Rust/toolchain/quickstart согласованы, мёртвые ссылки убраны |
| P0 | Зафиксировать единую truth table по версиям, портам, профилям запуска | 8 ч | root docs cleanup | есть одна таблица “profile → ports → owner → canonical doc”, на неё ссылаются README и quickstart |
| P0 | Убрать ручной drift по `CRATE_API.md`, начать с `rustok-workflow` | 12 ч | нет | manual API files либо обновлены и помечены как curated overview, либо заменены generated reference links |
| P0 | Починить `CHANGELOG.md` и привести к строгому release-log формату | 6 ч | root docs cleanup | нет битой кодировки, нет ссылок на отсутствующие guide-файлы, структура пригодна для релизов |
| P1 | Разделить `docs/index.md` на short map и volatile status docs | 10 ч | root docs cleanup | `docs/index.md` ≤ 250–300 строк, только навигация и правила entrypoint |
| P1 | Синхронизировать `docs/modules/registry.md`, `_index.md`, `UI_PACKAGES_INDEX.md` с `modules.toml` | 10 ч | truth table | нет `tax`/других модулей в platform registry, если их нет в manifest; capability/support сущности промаркированы корректно |
| P1 | Сделать generated reference pipeline | 20 ч | API drift cleanup | Rust API, OpenAPI и GraphQL surface публикуются артефактами в CI; curated docs ссылаются на них |
| P1 | Централизовать executable examples и migration guides | 16 ч | truth table | есть `docs/examples/` или `examples/`, минимум 5 smoke-verified сценариев |
| P1 | Добавить docs CI: markdownlint, linkcheck, code fence validation, docs build | 12 ч | root docs cleanup | PR падает на битых ссылках, broken anchors, невалидных fenced blocks |
| P2 | Причесать локализацию и naming consistency | 8 ч | docs split | policy “один файл — один язык” соблюдается или официально смягчена и задокументирована |
| P2 | Ввести review checklists и ownership | 4 ч | docs CI | PR template и reviewer checklist обновлены, в команде есть готовый процесс |
| P2 | Документировать code hotspots first | 18 ч | generated reference pipeline | задокументированы приоритетные узлы из таблицы ниже |

### Дорожная карта

```mermaid
gantt
    title Дорожная карта приведения документации RusTok в рабочее состояние
    dateFormat  YYYY-MM-DD
    axisFormat  %d.%m

    section P0
    Root docs sanitation           :a1, 2026-05-25, 4d
    Truth table портов и версий    :a2, after a1, 2d
    Workflow API drift cleanup     :a3, 2026-05-26, 4d
    CHANGELOG cleanup              :a4, 2026-05-28, 2d

    section P1
    Split docs index               :b1, after a2, 3d
    Registry and manifest sync     :b2, after a2, 4d
    Docs CI and link checks        :b3, after a1, 4d
    Generated reference pipeline   :b4, after a3, 6d
    Examples and migration guides  :b5, after b3, 5d

    section P2
    Localization and naming sweep  :c1, after b1, 3d
    Review checklists              :c2, after b3, 2d
    High-priority code hotspots    :c3, after b4, 6d
```

### Что даст максимальный эффект быстрее всего

Если делать только один короткий sprint, я бы взял вот такой пакет:

| Что делаем в спринт | Почему это даёт максимум |
|---|---|
| README + README.ru + CONTRIBUTING + CHANGELOG | это first-contact docs; исправление сразу снижает число ложных решений при старте |
| `docs/index.md` short map | разработчик перестаёт тонуть в статусных полотнах |
| `rustok-workflow/CRATE_API.md` → generated/curated hybrid | даёт шаблон для всех остальных модулей |
| docs CI с markdown + links | не даёт этим проблемам вернуться через неделю |

## CI и автоматические проверки

Текущий CI уже не пустой: там есть `cargo fmt`, `clippy`, `cargo check`, `cargo doc`, `xtask validate-manifest`, `cargo xtask module validate`, `typos`, security jobs, unused-deps, coverage, build/test jobs и даже freshness-check для upstream snapshot docs. Это хороший фундамент. Но именно **документационный QA** там пока недостаточен: нет обязательного markdown lint, нет dead-link/anchor check, нет проверки fenced code blocks, нет сборки docs site и нет smoke-run для примеров из документации. fileciteturn69file0L1-L3

### Что добавить в CI обязательно

| Проверка | Зачем нужна именно RusTok |
|---|---|
| `markdownlint` | много ручных `.md`, где уже видны структурные and formatting проблемы |
| `lychee` или аналогичный link checker | root docs и changelog уже содержат stale links |
| `mdformat --check` или единый formatter policy | снижает шум в PR и хаос в headings/lists/code fences |
| docs build | нужен единый smoke-test на documentation site / nav |
| code fence checker | quickstart/install snippets должны быть синтаксически валидны |
| example smoke tests | хотя бы `./scripts/dev-start.sh --help`, `cargo xtask --help`, `cargo bench -p rustok-benchmarks -- --help` и т.п. |
| rustdoc coverage gate для public crates | ручные `CRATE_API.md` надо вытеснять комментариями в коде |
| exported OpenAPI / GraphQL schema diff | чтобы API docs не дрейфовали отдельно от кода |

### Рекомендуемый workflow для документации

```yaml
name: Docs Quality

on:
  pull_request:
  push:
    branches: ["main"]

jobs:
  docs-quality:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5

      - uses: actions/setup-python@v6
        with:
          python-version: "3.12"

      - uses: actions/setup-node@v5
        with:
          node-version: "22"

      - name: Install markdown tooling
        run: |
          npm install -g markdownlint-cli2
          pip install mdformat mdformat-gfm

      - name: Markdown lint
        run: markdownlint-cli2 "**/*.md" "#target" "#node_modules"

      - name: Markdown format check
        run: mdformat --check .

      - name: Link check
        uses: lycheeverse/lychee-action@v2
        with:
          args: >
            --no-progress
            --accept 200,429
            --exclude-mail
            --exclude-path target
            --exclude-path node_modules
            "**/*.md"

      - name: Validate docs examples
        run: |
          bash scripts/dev-start.sh --help
          cargo xtask --help
          cargo bench -p rustok-benchmarks -- --help

      - name: Build Rust API docs
        run: cargo doc --workspace --all-features --no-deps
        env:
          RUSTDOCFLAGS: "-D warnings"
```

### Минимальная конфигурация lint для Markdown

```json
{
  "default": true,
  "MD013": false,
  "MD033": false,
  "MD041": false,
  "MD046": { "style": "fenced" }
}
```

`MD033` разумно ослабить, потому что в root README уже есть HTML-block с логотипом; но дальше советую постепенно вычищать даже такие исключения и сводить Markdown к более предсказуемому виду. fileciteturn23file0L1-L3

### Что делать с API reference

Для Rust-части я бы делал так:

| Поверхность | Источник правды | Что публиковать |
|---|---|---|
| Rust public API | rustdoc comments в коде | `cargo doc` артефакт |
| REST | `utoipa` / live OpenAPI | `openapi.json` artifact + curated guide |
| GraphQL | live schema export | `schema.graphql` artifact + curated guide |
| TypeScript shared packages | TypeDoc там, где API реально внешний | typed generated docs |

Для RusTok особенно важно не писать руками “списки методов” там, где их можно достать из кода. Ручные overview-файлы допустимы только как **curated guide**: “что использовать и зачем”, а не “перечень сигнатур”. Эту границу как раз и нарушает текущий `crates/rustok-workflow/CRATE_API.md`. fileciteturn47file0L1-L3 fileciteturn51file0L1-L3

## Примеры исправлений и шаблоны

### Патч для root README

```diff
diff --git a/README.md b/README.md
@@
-[![CI](https://github.com/RustokCMS/RusToK/actions/workflows/ci.yml/badge.svg)](https://github.com/RustokCMS/RusToK/actions/workflows/ci.yml)
+[![CI](https://github.com/RusTokRs/RusTok/actions/workflows/ci.yml/badge.svg)](https://github.com/RusTokRs/RusTok/actions/workflows/ci.yml)
@@
-[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg)](https://www.rust-lang.org)
+[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)
@@
-- Rust toolchain (version from repository `rust-toolchain.toml`)
+- Rust toolchain compatible with workspace `rust-version = 1.85`
@@
-Think of it like Lego for backend systems
+Think of it like Lego for backend systems.
+
+> Developer note:
+> authoritative entrypoints for implementation are `docs/index.md`,
+> `docs/modules/manifest.md`, `xtask/README.md`, and local app/module docs.
```

Это закрывает сразу три проблемы: старый repo URL, неверное обещание по Rust version и отсутствие жёсткого указания на implementation entrypoints. fileciteturn23file0L1-L3 fileciteturn25file0L1-L3

### Патч для CONTRIBUTING

```diff
diff --git a/CONTRIBUTING.md b/CONTRIBUTING.md
@@
-- **Rust**: Version 1.80 or higher
+- **Rust**: MSRV 1.85
@@
-git clone https://github.com/RustokCMS/RusToK.git
-cd RusToK
+git clone https://github.com/RusTokRs/RusTok.git
+cd RusTok
@@
-- **apps/admin**: Leptos CSR admin panel
+- **apps/admin**: Leptos admin host; preferred runtime is SSR/hydrate, CSR is debug/compatibility profile
@@
-See `docs/testing-guidelines.md` for detailed testing strategies.
+See `docs/guides/testing.md`, `docs/guides/testing-property.md`, and local app/module docs for testing strategy.
@@
-- [ ] DOCS_MAP.md updated when docs triggers were touched
+- [ ] `docs/index.md` updated when central navigation changed
@@
-cd docs && mdbook build
+# Optional: build documentation site only if/when a docs-site toolchain is enabled in this repository.
```

Эта правка убирает найболее явные устаревшие точки и ставит `CONTRIBUTING.md` в один ряд с фактическими app docs. fileciteturn30file0L1-L3 fileciteturn41file0L1-L3 fileciteturn46file0L1-L3

### Патч для `crates/rustok-workflow/CRATE_API.md`

```diff
diff --git a/crates/rustok-workflow/CRATE_API.md b/crates/rustok-workflow/CRATE_API.md
@@
-### `WorkflowTriggerHandler`
+### `WorkflowTriggerHandler`
@@
-    pub fn new(db: DatabaseConnection, engine: Arc<WorkflowEngine>) -> Self;
+    pub fn new(db: DatabaseConnection) -> Self;
@@
-### `WorkflowEngine`
+### `WorkflowEngine`
@@
-    pub fn with_step(mut self, step_type: impl Into<String>, step: Arc<dyn WorkflowStep>) -> Self;
-    pub async fn execute(&self, workflow_id: Uuid, trigger_event_id: Uuid, context: Value) -> WorkflowResult<Uuid>;
+    pub fn register_step<S: WorkflowStep + 'static>(&mut self, step: S);
+    pub async fn execute(
+        &self,
+        workflow_id: Uuid,
+        tenant_id: Uuid,
+        trigger_event_id: Option<Uuid>,
+        steps: Vec<crate::entities::WorkflowStep>,
+        initial_context: Value,
+    ) -> WorkflowResult<Uuid>;
@@
-pub enum WorkflowError {
-    NotFound(Uuid),
-    Unauthorized,
-    InvalidConfig(String),
-    StepFailed { step_id: Uuid, reason: String },
-    Database(DbErr),
-}
+pub enum WorkflowError {
+    NotFound(Uuid),
+    StepNotFound(Uuid),
+    ExecutionNotFound(Uuid),
+    NotActive(String),
+    StepFailed(String),
+    UnknownStepType(String),
+    InvalidTriggerConfig(String),
+    InvalidStepConfig(String),
+    Database(DbErr),
+    Serialization(serde_json::Error),
+}
```

Но стратегически я бы не ограничивался patch-by-patch. Лучше переименовать такие файлы в `API_OVERVIEW.md` и сознательно удалить из них ручные сигнатуры, оставив только: “главные типы”, “стабильные entrypoints”, “ссылка на generated docs”. Иначе drift вернётся. fileciteturn47file0L1-L3 fileciteturn49file0L1-L3 fileciteturn51file0L1-L3 fileciteturn53file0L1-L3

### Шаблон для API overview вместо ручной сигнатурной документации

```md
# <crate-name> API Overview

## Purpose
Коротко: что crate считает своим public surface.

## Stable entry points
- `TypeA`
- `TypeB`
- `fn_xxx`
- `controllers::routes()`
- `graphql::QueryRoot`

## Contracts and invariants
- что гарантируется;
- какие tenant/auth/locale assumptions обязательны;
- что считается breaking change.

## Main workflows
### Создание
### Чтение
### Изменение
### Ошибки и edge cases

## Generated reference
- Rust API: `cargo doc ...`
- REST/OpenAPI: `<artifact or endpoint>`
- GraphQL schema: `<artifact or endpoint>`

## Examples
- минимальный happy path
- one failure path
- link to integration tests
```

### Чек-лист ревью документации

- [ ] Документ указывает **канонический источник истины** и не дублирует его без необходимости.
- [ ] Все команды и URL воспроизводимы в текущем репозитории.
- [ ] Версии toolchain, порты и profile assumptions согласованы с `Cargo.toml`, `modules.toml`, скриптами и CI.
- [ ] Нет старых org/repo ссылок.
- [ ] Нет ручных списков API-сигнатур, которые уже можно генерировать из кода.
- [ ] Все примеры либо smoke-verify в CI, либо честно помечены как pseudocode.
- [ ] Для security-related snippets dev-only значения помечены явно.
- [ ] Документ либо целиком на одном языке, либо policy официально разрешает mixed-language category.

### Чек-лист поддержания качества

- [ ] Любое изменение публичного API меняет либо generated reference, либо curated overview, либо оба.
- [ ] Любое изменение `modules.toml` обновляет `docs/modules/registry.md`, `_index.md`, `UI_PACKAGES_INDEX.md`, если релевантно.
- [ ] Любое изменение entrypoint docs проходит linkcheck и markdownlint.
- [ ] Любой новый guide имеет owner, дату последнего обновления и соседний smoke/example.
- [ ] `docs/index.md` остаётся короткой картой, а не статусным полотном.
- [ ] CHANGELOG содержит только release-notes, а не sprint-дневник.

### Приоритетные места в коде, которые нужно документировать первыми

| Приоритет | Место | Почему именно это |
|---|---|---|
| Высокий | `modules.toml` | это composition root, а вокруг него уже есть drift в registry/docs |
| Высокий | `xtask/README.md` + реализация `xtask` | это фактический enforcement layer для всей docs/manifest системы |
| Высокий | `apps/server/src/app.rs` | здесь реальная картография routes/runtime bootstrap и profile behavior |
| Высокий | `apps/server/src/controllers/health.rs` | central API docs уже отстают от реальной health surface |
| Высокий | `crates/rustok-workflow/src/lib.rs`, `services/*`, `CRATE_API.md` | наглядный пример drift ручного API reference |
| Высокий | `crates/rustok-auth/src/jwt.rs` и связанный config docs | security-critical public contract с большим числом assumptions |
| Средний | `benches/benches/*` + performance docs | убрать фейковую/устаревшую benchmark-мифологию |
| Средний | `apps/admin/docs/README.md` и root/CONTRIBUTING cross-links | app docs уже правдивее root docs, их надо сделать каноном onboarding |
| Средний | `docs/modules/registry.md` и `docs/index.md` | убрать смешение “crate exists” vs “platform module” |
| Средний | `scripts/dev-start.sh` и installer docs | зафиксировать один точный путь локального старта |

## Открытые вопросы и ограничения

Я сознательно ограничился **только этим репозиторием и его GitHub-содержимым**. Это хорошо подходит для аудита “доверять ли docs при работе с кодом”, но не покрывает возможные внешние сайты проекта, GitHub Wiki или опубликованные артефакты документации, если они существуют вне текущего дерева репозитория. Через проведённый анализ Wiki как отдельный слой не был явно подтверждён; если она есть и используется, её стоит включить в следующий проход как отдельный источник дрейфа. fileciteturn22file0L1-L3 fileciteturn23file0L1-L3

Также важно честно отметить пределы: я проверил весь **документированный контур** — центральные entrypoints, app docs, модульные реестры, changelog, CI, ключевые reference-файлы, примеры, выбранные исходники и тесты — но не делал построчный ручной аудит каждого Markdown-файла и каждого crate README в огромном workspace. Поэтому мой план ориентирован на быстрое приведение системы в состояние, где дальнейший аудит уже можно будет частично автоматизировать. Это и есть правильный следующий шаг: сначала сократить поверхность дрейфа, потом дочищать остатки. fileciteturn22file0L1-L3 fileciteturn39file0L1-L3 fileciteturn69file0L1-L3

Главный вывод остаётся тем же: **у RusTok уже есть почти все заготовки для образцовой документации, но пока нет дисциплины “один источник истины + автоматические проверки на дрейф”**. Как только вы приведёте в порядок root docs, уберёте ручной API-drift и поставите жёсткий docs CI, документации можно будет реально доверять при написании кода. Сейчас — пока ещё не полностью. fileciteturn23file0L1-L3 fileciteturn47file0L1-L3 fileciteturn69file0L1-L3