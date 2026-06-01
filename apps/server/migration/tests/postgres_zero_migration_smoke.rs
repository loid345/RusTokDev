use migration::Migrator;
use sea_orm_migration::{
    prelude::MigratorTrait,
    sea_orm::{ConnectionTrait, Database, DbBackend, Statement},
};

#[tokio::test]
#[ignore = "requires an empty PostgreSQL database; run scripts/verify/verify-migration-smoke.sh"]
async fn postgres_zero_migration_smoke_applies_from_empty_database() {
    let database_url = std::env::var("RUSTOK_MIGRATION_SMOKE_DATABASE_URL")
        .expect("RUSTOK_MIGRATION_SMOKE_DATABASE_URL must point to an empty PostgreSQL database");
    assert!(
        database_url.starts_with("postgres://") || database_url.starts_with("postgresql://"),
        "migration smoke must run against PostgreSQL"
    );

    let db = Database::connect(&database_url)
        .await
        .expect("smoke database must be reachable");

    Migrator::up(&db, None)
        .await
        .expect("server migrator must apply from zero on PostgreSQL");

    let pending = Migrator::get_pending_migrations(&db)
        .await
        .expect("pending migration list must be readable after smoke apply");
    assert!(
        pending.is_empty(),
        "all migrations should be applied, pending: {:?}",
        pending
            .iter()
            .map(|migration| migration.name().to_string())
            .collect::<Vec<_>>()
    );

    for table in [
        "tenants",
        "users",
        "product_variants",
        "prices",
        "inventory_items",
        "channels",
        "oauth_apps",
        "blog_post_tags",
        "forum_topic_tags",
        "taxonomy_terms",
    ] {
        assert_table_exists(&db, table).await;
    }
}

async fn assert_table_exists(db: &sea_orm_migration::sea_orm::DatabaseConnection, table: &str) {
    let row = db
        .query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT to_regclass($1) IS NOT NULL AS exists",
            [format!("public.{table}").into()],
        ))
        .await
        .unwrap_or_else(|error| panic!("table existence query for {table} must succeed: {error}"))
        .unwrap_or_else(|| panic!("table existence query for {table} returned no row"));
    let exists: bool = row
        .try_get("", "exists")
        .unwrap_or_else(|error| panic!("table existence result for {table} must decode: {error}"));
    assert!(exists, "expected table {table} to exist after migrations");
}
