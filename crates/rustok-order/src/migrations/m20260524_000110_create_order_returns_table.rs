use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrderReturns::Table)
                    .if_not_exists()
                    .col(uuid(OrderReturns::Id).primary_key())
                    .col(uuid(OrderReturns::TenantId).not_null())
                    .col(uuid(OrderReturns::OrderId).not_null())
                    .col(string_len(OrderReturns::Reason, 255))
                    .col(text(OrderReturns::Note))
                    .col(string_len(OrderReturns::Status, 32).not_null())
                    .col(json_binary(OrderReturns::Metadata).not_null())
                    .col(timestamp_with_time_zone(OrderReturns::CreatedAt).not_null())
                    .col(timestamp_with_time_zone(OrderReturns::UpdatedAt).not_null())
                    .col(timestamp_with_time_zone(OrderReturns::CompletedAt))
                    .col(timestamp_with_time_zone(OrderReturns::CancelledAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_order_returns_order_id")
                            .from(OrderReturns::Table, OrderReturns::OrderId)
                            .to(Orders::Table, Orders::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrderReturns::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum OrderReturns {
    Table,
    Id,
    TenantId,
    OrderId,
    Reason,
    Note,
    Status,
    Metadata,
    CreatedAt,
    UpdatedAt,
    CompletedAt,
    CancelledAt,
}

#[derive(DeriveIden)]
enum Orders {
    Table,
    Id,
}
