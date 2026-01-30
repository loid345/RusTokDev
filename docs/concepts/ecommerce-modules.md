RUST E-COMMERCE MODULAR MONOLITH

Версия архитектуры: 1.0 (Unified Final)
Философия: строгая изоляция данных, общение через Traits/Contracts, функциональное разделение, готовность к эволюции в микросервисы.

Ключевые решения (зафиксировано)

Commerce = 3 модуля: Pricing + Inventory + Marketing (разные предметные области).

Cart отдельно от Order: разный профиль нагрузки и надежности (hot vs durable).

Inventory как Ledger: транзакционная история движений, аудит, резервирование.

Fulfillment внутри Order: это часть жизненного цикла заказа (OMS).

Checkout — не модуль, а Application Service: координатор без собственной персистентной модели.

SEO — часть Platform: относится к публичному представлению (CMS/Storefront).

Relations — отдельная kernel-lib: универсальная механика M:N, нужна в нескольких доменах.

Search — kernel-lib (абстракция): чтобы не “пришить” домены к конкретному движку.

1) ФУНДАМЕНТ (KERNEL = libs/)

Библиотеки ядра не зависят ни от чего, но от них зависит всё.

Kernel Libraries (6)

libs/common
Общие типы: Money, IDs, SKU, ошибки, time, result types, сериализация/валидация.

libs/events
Trait EventBus, DomainEvent, outbox-friendly паттерны, идемпотентность обработчиков.

libs/rules_engine
Универсальный движок правил: Condition + Action + Evaluator.
Используется: pricing, marketing, customer, order (и опционально shipping/tax).

libs/relations
M:N связи как граф: Link(SourceType, SourceId, TargetType, TargetId, metadata).

libs/media
Абстракция файлов/изображений: адаптеры под Amazon S3 / MinIO, ресайзы, метаданные, версионирование.

libs/search
Абстракция над Meilisearch / Elasticsearch: индексация, query DSL, маппинг результатов.
Важно: домены не знают “кого именно” вы используете — только интерфейс.

2) БИЗНЕС-МОДУЛИ (DOMAIN CRATES = 8)

8 независимых крейтов. У каждого — своя ответственность и логически своя БД/схема (в модульном монолите это может быть один кластер, но границы — железные).

Группа A: “Товар и условия” (Base)

mod-catalog (PIM)
Ответственность: товары, категории, атрибуты, словари, варианты/SKU, наследование (“матрёшка”), фасеты.
Хранилище: PostgreSQL (schema/catalog).
Зависимости: kernel/*

mod-pricing (Calculator)
Ответственность: прайс-листы, цены, налоги/правила, валюты, расчет цены.
Ключ: stateless-калькулятор: Context → PriceBreakdown.
Хранилище: PostgreSQL (schema/pricing).
Зависимости: kernel/* + mod-catalog-api (только ID/минимальные контракты)

mod-inventory (Ledger)
Ответственность: склады, остатки, резервы, движения (double-entry/ledger-подход).
Хранилище: PostgreSQL (schema/inventory) + строгие транзакции.
Зависимости: kernel/*

mod-marketing (Promotions)
Ответственность: акции, купоны, кампании, gift card; управление правилами для rules_engine.
Хранилище: PostgreSQL (schema/marketing).
Зависимости: kernel/*

Группа B: “Сделка” (Process)

mod-cart (Hot Storage)
Ответственность: корзина как draft, merge, быстрые обновления.
Хранилище: Redis.
Зависимости: kernel/* + mod-catalog-api + mod-pricing-api (+ опционально marketing-api для купонов)

mod-order (OMS)
Ответственность: заказы, платежи, инвойсы, отгрузки, возвраты (RMA), state machine, snapshots.
Хранилище: PostgreSQL (schema/order).
Зависимости: kernel/* + mod-cart-api (и только через контракт)

Группа C: “Люди и витрина” (Environment)

mod-customer (CRM)
Ответственность: аккаунт, профиль, адреса, сегменты, wallet, UGC (reviews/questions).
Хранилище: PostgreSQL (schema/customer).
Зависимости: kernel/*

mod-platform (CMS + Storefront Context)
Ответственность: Store/Channel/Locale/Currency context, страницы, меню, SEO, конфиги, шаблоны нотификаций.
Хранилище: PostgreSQL (schema/platform).
Зависимости: kernel/*

3) КОНТРАКТЫ МЕЖДУ МОДУЛЯМИ (как “не развалить границы”)

Чтобы модули физически не лезли в БД друг друга — вводим тонкие API-крейты:

mod-catalog-api — только DTO/IDs/минимальные read-интерфейсы (например, “получить SKU axes / name / attributes for snapshot”).

mod-pricing-api — PriceCalculator trait + DTO PriceQuote.

mod-cart-api — CartReader trait для order/checkout.

Аналогично по мере надобности: inventory-api, customer-api, marketing-api.

Правило: доменный модуль экспортирует интерфейсы, а реализация живет внутри него. В app-слое происходит “склейка” реализаций (trait-based DI).

4) СЛОЙ ПРИЛОЖЕНИЯ (APP = app/)

Здесь — main.rs, wiring, сценарии, BFF/HTTP, orchestration, event handlers.

Application Services (3)

CheckoutService (оркестратор покупки)
Поток: Cart → (Marketing rules) → Pricing snapshot → Inventory reserve → Payment → Order create

Нет собственной БД → поэтому не модуль, а сервис.

Легко выделяется в отдельный сервис позже (если нужен abandoned cart, SAGA и т.д.)

SearchService (агрегатор результатов поиска)
Собирает: Catalog (контент) + Pricing (цены) + Inventory (наличие) → frontend JSON
Использует libs/search как абстракцию.

StorefrontService (BFF/агрегация для UI)
Сшивает домены в удобные модели: PDP/PLP/checkout views, “личный кабинет”, витринные конфиги.

Event Handlers (рекомендуемый набор)

OnOrderPlaced → reserve inventory / send notification / index update

OnPaymentSuccess → advance order state / start fulfillment

OnStockLow → alerts / auto-reorder hook

OnCartAbandoned → recovery notifications (если включаете)

5) ГРАФ ЗАВИСИМОСТЕЙ (DAG, без циклов)

Правило: зависимости только “вниз”, никаких кругов.

Логический DAG (упрощенно)

kernel/*
↓

catalog, pricing, inventory, marketing, customer, platform
↓

cart (зависит от catalog/pricing/marketing через api)
↓

order (зависит от cart через api)
↓

app (зависит от всех как оркестратор/витрина)

Mermaid (если нужно вставить в доку)
graph TD
  subgraph Kernel
    common
    events
    rules
    relations
    media
    search
  end

  subgraph Base
    catalog
    pricing
    inventory
    marketing
    customer
    platform
  end

  subgraph Process
    cart
    order
  end

  subgraph App
    checkoutSvc
    searchSvc
    storefrontSvc
  end

  Base --> Kernel
  Process --> Kernel

  cart --> catalog
  cart --> pricing
  cart --> marketing

  order --> cart

  checkoutSvc --> cart
  checkoutSvc --> pricing
  checkoutSvc --> inventory
  checkoutSvc --> order
  checkoutSvc --> customer

  searchSvc --> catalog
  searchSvc --> pricing
  searchSvc --> inventory
  searchSvc --> platform

  storefrontSvc --> catalog
  storefrontSvc --> pricing
  storefrontSvc --> inventory
  storefrontSvc --> customer
  storefrontSvc --> order
  storefrontSvc --> platform

6) ИТОГОВАЯ СПЕЦИФИКАЦИЯ (коротко, как “паспорт”)
Kernel (6 libs)

common — типы/ID/Money/errors

events — event bus + domain events

rules_engine — условия/действия/оценка

relations — универсальные связи M:N

media — assets/resize/storage adapters

search — search abstraction

Domain Modules (8)

catalog — Product/Category/Variant/Attribute (Postgres)

pricing — PriceList/Price/TaxRule (Postgres)

inventory — Warehouse/Stock/Movement/Reservation (Postgres ledger)

marketing — Promotion/Coupon/Campaign/GiftCard (Postgres)

cart — Cart/CartItem/Merger (Redis)

order — Order/Payment/Shipment/Return/Invoice (Postgres + snapshots)

customer — Account/Profile/Wallet/Segment/Review (Postgres)

platform — Store/Page/Menu/SEO/Config/Notifications (Postgres)

7) Маппинг на “идеальную матрицу” (из ТЗ)

CATALOG → mod-catalog

COMMERCE → mod-pricing + mod-inventory + mod-marketing

CUSTOMER → mod-customer

ORDER → mod-cart + mod-order

PLATFORM → mod-platform

INFRASTRUCTURE → libs/* (+ app orchestration)

8) Почему это выигрывает у MedusaJS (суть, без воды)

Границы модулей гарантируются компилятором (traits/contracts), а не “договоренностями”.

Inventory — не “число”, а аудитируемый ledger с движениями/резервами.

Cart и Order разделены по надежности/нагрузке.

Checkout — прозрачный сценарий, а не “черный ящик”.

Производительность и параллелизм экосистемы Node.js не конкурируют с Rust-подходом при сложных промо/калькуляциях.