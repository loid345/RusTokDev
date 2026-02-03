use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct SysEventsMigration;

#[async_trait::async_trait]
impl MigrationTrait for SysEventsMigration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysEvents::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysEvents::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SysEvents::Payload).json_binary().not_null())
                    .col(ColumnDef::new(SysEvents::Status).string_len(32).not_null())
                    .col(
                        ColumnDef::new(SysEvents::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SysEvents::DispatchedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysEvents::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SysEvents {
    Table,
    Id,
    Payload,
    Status,
    CreatedAt,
    DispatchedAt,
}
