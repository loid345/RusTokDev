# Custom Fields — Implementation Plan (`rustok-custom-fields`)

## Scope and objective

Универсальная библиотека для определения, валидации и хранения кастомных полей
(custom fields / entity attributes) в RusToK. Любой модуль (users, commerce, content,
forum) может подключить поддержку кастомных полей через общий trait, при этом **данные
остаются внутри модуля** (в существующих `metadata: Json` колонках), а библиотека
предоставляет только типы, валидацию, query-хелперы и GraphQL-интеграцию.

### Принципы

- **Zero coupling** — библиотека не знает о конкретных сущностях; модули подключают её через trait.
- **Schema-first** — админ тенанта определяет схему полей, данные валидируются при записи.
- **Tenant isolation** — схемы полей привязаны к тенанту; каждый тенант задаёт свой набор.
- **JSONB storage** — значения хранятся в существующих `metadata` колонках, без новых столбцов.
- **Backward compatible** — существующие `metadata` без схемы продолжают работать.

## Target architecture

```
rustok-custom-fields (crate, library)
├── src/
│   ├── lib.rs                    — pub exports
│   ├── types.rs                  — FieldType, FieldDefinition, FieldValue, ValidationRule
│   ├── schema.rs                 — CustomFieldsSchema: validate, merge, diff, defaults
│   ├── traits.rs                 — HasCustomFields trait для сущностей
│   ├── validation.rs             — валидация значений по типу и правилам
│   ├── query.rs                  — SeaORM хелперы для фильтрации по JSONB полям
│   ├── graphql.rs                — async-graphql InputObject/SimpleObject для полей
│   └── error.rs                  — CustomFieldError enum
├── tests/
│   ├── validation_tests.rs
│   ├── schema_tests.rs
│   └── query_tests.rs
├── docs/
│   └── implementation-plan.md    — (этот файл)
└── Cargo.toml
```

### Зависимости

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
sea-orm = { version = "1", features = ["with-json", "with-uuid", "with-chrono"] }
async-graphql = { version = "7", optional = true }
regex = "1"

[features]
default = []
graphql = ["dep:async-graphql"]
```

## Data model

### 1. `custom_field_definitions` (новая таблица, миграция в apps/server)

| Column            | Type           | Notes                                      |
|-------------------|----------------|--------------------------------------------|
| `id`              | UUID PK        |                                            |
| `tenant_id`       | UUID FK        | → tenants.id                               |
| `entity_type`     | String(64)     | "user", "product", "node", "order" и т.д.  |
| `field_key`       | String(128)    | snake_case ключ, например `phone_number`   |
| `field_type`      | String(32)     | enum FieldType как строка                  |
| `label`           | JSONB          | `{"en": "Phone", "ru": "Телефон"}`         |
| `description`     | JSONB nullable | локализованное описание                    |
| `is_required`     | Boolean        | default false                              |
| `default_value`   | JSONB nullable | значение по умолчанию                      |
| `validation`      | JSONB nullable | ValidationRule (min, max, regex, options…)  |
| `position`        | i32            | порядок отображения, default 0             |
| `is_active`       | Boolean        | default true, soft-disable без удаления    |
| `created_at`      | TimestampTZ    |                                            |
| `updated_at`      | TimestampTZ    |                                            |

**Indexes:**
- `UNIQUE (tenant_id, entity_type, field_key)`
- `idx_cfd_tenant_entity (tenant_id, entity_type, is_active)`

### 2. Хранение значений

Значения хранятся в **существующих** `metadata: Json` колонках сущностей:

```json
{
  "phone_number": "+7 999 123-45-67",
  "company": "Acme Corp",
  "age": 28,
  "interests": ["rust", "cms"],
  "agreed_to_newsletter": true
}
```

Никаких EAV-таблиц — JSONB в PostgreSQL достаточно эффективен для этого.

## Core types

### FieldType enum

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    /// Однострочный текст
    Text,
    /// Многострочный текст
    Textarea,
    /// Целое число
    Integer,
    /// Число с плавающей точкой
    Decimal,
    /// true/false
    Boolean,
    /// ISO 8601 дата (YYYY-MM-DD)
    Date,
    /// ISO 8601 дата-время
    DateTime,
    /// URL (валидация формата)
    Url,
    /// Email (валидация формата)
    Email,
    /// Телефон (свободный формат с regex-валидацией)
    Phone,
    /// Выбор из списка (один)
    Select,
    /// Выбор из списка (несколько)
    MultiSelect,
    /// Цвет (#RRGGBB)
    Color,
    /// JSON (произвольная структура)
    Json,
}
```

### ValidationRule

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidationRule {
    /// Мин. длина строки / мин. значение числа
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    /// Макс. длина строки / макс. значение числа
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    /// Regex паттерн для текстовых полей
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// Допустимые варианты для Select/MultiSelect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<SelectOption>>,
    /// Кастомное сообщение об ошибке (локализованное)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOption {
    pub value: String,
    pub label: HashMap<String, String>,  // {"en": "Male", "ru": "Мужской"}
}
```

### FieldDefinition (DTO, не entity)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub field_key: String,
    pub field_type: FieldType,
    pub label: HashMap<String, String>,
    pub description: Option<HashMap<String, String>>,
    pub is_required: bool,
    pub default_value: Option<serde_json::Value>,
    pub validation: Option<ValidationRule>,
    pub position: i32,
    pub is_active: bool,
}
```

### HasCustomFields trait

```rust
/// Trait для сущностей, поддерживающих кастомные поля.
/// Каждый модуль реализует его для своей entity.
pub trait HasCustomFields {
    /// Тип сущности для поиска определений полей.
    /// Например: "user", "product", "node".
    fn entity_type() -> &'static str;

    /// Возвращает текущие metadata как JSON Value.
    fn metadata(&self) -> &serde_json::Value;

    /// Устанавливает metadata.
    fn set_metadata(&mut self, value: serde_json::Value);
}
```

### CustomFieldsSchema (сервис валидации)

```rust
pub struct CustomFieldsSchema {
    definitions: Vec<FieldDefinition>,
}

impl CustomFieldsSchema {
    /// Загрузить схему из Vec<FieldDefinition>
    pub fn new(definitions: Vec<FieldDefinition>) -> Self;

    /// Валидировать metadata по схеме.
    /// Возвращает список ошибок (пустой = валидно).
    pub fn validate(&self, metadata: &serde_json::Value) -> Vec<FieldValidationError>;

    /// Заполнить default-значения для отсутствующих полей.
    pub fn apply_defaults(&self, metadata: &mut serde_json::Value);

    /// Удалить поля, не описанные в схеме (опционально).
    pub fn strip_unknown(&self, metadata: &mut serde_json::Value);

    /// Вернуть только активные определения.
    pub fn active_definitions(&self) -> Vec<&FieldDefinition>;
}
```

## Delivery phases

### Phase 0 — Library core (foundation)

- [ ] Создать крейт `crates/rustok-custom-fields/` со структурой выше.
- [ ] Реализовать `types.rs`: `FieldType`, `ValidationRule`, `SelectOption`, `FieldDefinition`.
- [ ] Реализовать `validation.rs`: валидация одного значения по `FieldType` + `ValidationRule`.
- [ ] Реализовать `schema.rs`: `CustomFieldsSchema` с `validate()`, `apply_defaults()`, `strip_unknown()`.
- [ ] Реализовать `traits.rs`: `HasCustomFields` trait.
- [ ] Реализовать `error.rs`: `CustomFieldError` enum.
- [ ] Unit-тесты для всех типов полей и правил валидации.
- [ ] Добавить крейт в workspace `Cargo.toml`.

**Результат:** библиотека компилируется, тесты проходят, zero dependencies на другие RusToK-крейты.

### Phase 1 — Database & entity layer

- [ ] Создать миграцию `m20260315_000001_create_custom_field_definitions.rs` в `apps/server/migration/`.
- [ ] Создать SeaORM entity `custom_field_definitions` в `apps/server/src/models/_entities/`.
- [ ] Реализовать `query.rs` в библиотеке: хелперы для JSONB фильтрации через SeaORM.
  - `json_contains(column, key, value)` → Condition
  - `json_exists(column, key)` → Condition
  - `json_extract(column, key)` → SimpleExpr (для сортировки)
- [ ] Создать сервис `CustomFieldDefinitionService` в `apps/server/` (или в отдельном сервисном модуле):
  - `list_definitions(db, tenant_id, entity_type) → Vec<FieldDefinition>`
  - `create_definition(db, tenant_id, input) → FieldDefinition`
  - `update_definition(db, tenant_id, id, input) → FieldDefinition`
  - `delete_definition(db, tenant_id, id)` (soft: is_active = false)
  - `get_schema(db, tenant_id, entity_type) → CustomFieldsSchema`

**Результат:** CRUD для определений полей, загрузка схемы из БД.

### Phase 2 — Users integration (первый потребитель)

- [ ] Реализовать `HasCustomFields` для `users::Model` в `apps/server/src/models/users.rs`.
- [ ] В `CreateUserInput` / `UpdateUserInput` (GraphQL) добавить `custom_fields: Option<Json>`.
- [ ] В мутациях создания/обновления пользователя:
  - Загрузить схему: `CustomFieldDefinitionService::get_schema(db, tenant_id, "user")`.
  - Применить defaults: `schema.apply_defaults(&mut metadata)`.
  - Валидировать: `schema.validate(&metadata)` → ошибки в GraphQL response.
  - Сохранить в `users.metadata`.
- [ ] В GraphQL-типе `User` добавить resolver `custom_fields` → возвращает metadata.
- [ ] В `UsersFilter` (GraphQL) добавить опциональные фильтры по кастомным полям.
- [ ] Тесты: создание пользователя с кастомными полями, валидация, фильтрация.

**Результат:** админ может задать кастомные поля для пользователей тенанта, пользователи заполняют их, данные валидируются.

### Phase 3 — Admin API для управления определениями

- [ ] GraphQL queries:
  - `customFieldDefinitions(entityType: String!): [CustomFieldDefinition!]!`
  - `customFieldDefinition(id: UUID!): CustomFieldDefinition`
- [ ] GraphQL mutations:
  - `createCustomFieldDefinition(input: CreateCustomFieldDefinitionInput!): CustomFieldDefinition!`
  - `updateCustomFieldDefinition(id: UUID!, input: UpdateCustomFieldDefinitionInput!): CustomFieldDefinition!`
  - `deleteCustomFieldDefinition(id: UUID!): Boolean!`
  - `reorderCustomFieldDefinitions(ids: [UUID!]!): [CustomFieldDefinition!]!`
- [ ] RBAC: только `Admin` и `SuperAdmin` могут управлять определениями.
- [ ] Кеширование схемы в памяти (per tenant+entity_type) с инвалидацией при изменении.

**Результат:** полноценный UI-ready API для управления кастомными полями.

### Phase 4 — GraphQL type для определений (async-graphql feature)

- [ ] `graphql.rs` (за feature-флагом `graphql`):
  ```rust
  #[derive(SimpleObject)]
  pub struct GqlCustomFieldDefinition {
      pub id: UUID,
      pub field_key: String,
      pub field_type: String,
      pub label: Json,
      pub is_required: bool,
      pub default_value: Option<Json>,
      pub validation: Option<Json>,
      pub position: i32,
  }

  #[derive(InputObject)]
  pub struct GqlCreateCustomFieldDefinitionInput {
      pub field_key: String,
      pub field_type: String,
      pub label: Json,
      pub is_required: Option<bool>,
      pub default_value: Option<Json>,
      pub validation: Option<Json>,
      pub position: Option<i32>,
  }
  ```
- [ ] Input validation: `field_key` — snake_case, unique per entity_type.
- [ ] Локализация labels через стандартный `HashMap<locale, text>`.

### Phase 5 — Расширение на другие модули (по мере необходимости)

- [ ] `rustok-commerce`: `HasCustomFields` для `Product`, `Order` → `entity_type = "product"`, `"order"`.
- [ ] `rustok-content`: `HasCustomFields` для `Node` → `entity_type = "node"`.
- [ ] `rustok-forum`: `HasCustomFields` для `Topic` → `entity_type = "topic"`.
- [ ] Каждый модуль добавляет зависимость `rustok-custom-fields` и реализует trait.
- [ ] Данные остаются в `metadata` колонке каждой сущности.

### Phase 6 — Продвинутые возможности (future)

- [ ] Conditional fields: показывать поле B только если поле A = X.
- [ ] Computed fields: значение вычисляется по формуле.
- [ ] Field groups: визуальная группировка полей в UI.
- [ ] Import/export схемы полей между тенантами.
- [ ] Аудит изменений кастомных полей через events.
- [ ] Полнотекстовый поиск по кастомным полям через `rustok-index`.

## Миграции

### m20260315_000001_create_custom_field_definitions.rs

```rust
// Pseudo-code for migration
manager.create_table(
    Table::create()
        .table(CustomFieldDefinitions::Table)
        .col(ColumnDef::new(Id).uuid().not_null().primary_key())
        .col(ColumnDef::new(TenantId).uuid().not_null())
        .col(ColumnDef::new(EntityType).string_len(64).not_null())
        .col(ColumnDef::new(FieldKey).string_len(128).not_null())
        .col(ColumnDef::new(FieldType).string_len(32).not_null())
        .col(ColumnDef::new(Label).json_binary().not_null())
        .col(ColumnDef::new(Description).json_binary())
        .col(ColumnDef::new(IsRequired).boolean().not_null().default(false))
        .col(ColumnDef::new(DefaultValue).json_binary())
        .col(ColumnDef::new(Validation).json_binary())
        .col(ColumnDef::new(Position).integer().not_null().default(0))
        .col(ColumnDef::new(IsActive).boolean().not_null().default(true))
        .col(ColumnDef::new(CreatedAt).timestamp_with_time_zone().not_null())
        .col(ColumnDef::new(UpdatedAt).timestamp_with_time_zone().not_null())
        .foreign_key(ForeignKey::create()
            .from(CustomFieldDefinitions::Table, TenantId)
            .to(Tenants::Table, Tenants::Id)
            .on_delete(ForeignKeyAction::Cascade))
        .to_owned(),
)?;

// Unique constraint: one definition per key per entity type per tenant
manager.create_index(
    Index::create()
        .name("uq_cfd_tenant_entity_key")
        .table(CustomFieldDefinitions::Table)
        .col(TenantId).col(EntityType).col(FieldKey)
        .unique()
        .to_owned(),
)?;

// Lookup index
manager.create_index(
    Index::create()
        .name("idx_cfd_tenant_entity_active")
        .table(CustomFieldDefinitions::Table)
        .col(TenantId).col(EntityType).col(IsActive)
        .to_owned(),
)?;
```

## Интеграция с существующей архитектурой

| Аспект             | Подход                                                            |
|--------------------|-------------------------------------------------------------------|
| **Tenant isolation**| `tenant_id` FK на определениях, фильтрация во всех запросах       |
| **i18n**           | `label` и `description` — JSONB с ключами-локалями              |
| **RBAC**           | Управление определениями — Admin+; заполнение — по правам entity |
| **Events**         | `custom_field_definition.created/updated/deleted` events          |
| **Search**         | Индексация кастомных полей через `rustok-index` (Phase 6)        |
| **GraphQL**        | Feature-gated async-graphql types                                 |
| **REST**           | Можно добавить REST эндпоинты по аналогии (не приоритет)          |

## Risks and mitigations

| Risk                                      | Mitigation                                           |
|-------------------------------------------|------------------------------------------------------|
| JSONB performance на больших объёмах       | GIN индекс на `metadata`, пагинация, лимиты полей   |
| Несогласованность схемы и данных           | Валидация при записи; migration tool для ретро-валидации |
| Слишком много полей от тенантов            | Лимит на количество определений per entity_type (50) |
| Сложность UI для управления полями        | Стандартный паттерн form builder, Phase 3 API        |

## Tracking and updates

When updating custom fields architecture, API contracts, or integration patterns:

1. Update this file first.
2. Coordinate with affected modules (users, commerce, content).
3. Ensure migration compatibility with existing `metadata` data.
