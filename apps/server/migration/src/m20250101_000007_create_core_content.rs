use sea_orm_migration::prelude::*;

use super::m20250101_000001_create_tenants::Tenants;
use super::m20250101_000002_create_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Nodes::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Nodes::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Nodes::TenantId).uuid().not_null())
                    .col(ColumnDef::new(Nodes::ParentId).uuid().null())
                    .col(ColumnDef::new(Nodes::AuthorId).uuid().null())
                    .col(ColumnDef::new(Nodes::Kind).string_len(32).not_null())
                    .col(
                        ColumnDef::new(Nodes::Status)
                            .string_len(32)
                            .not_null()
                            .default("draft"),
                    )
                    .col(
                        ColumnDef::new(Nodes::Position)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Nodes::Depth)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Nodes::ReplyCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Nodes::Metadata)
                            .json_binary()
                            .not_null()
                            .default("{}"),
                    )
                    .col(
                        ColumnDef::new(Nodes::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Nodes::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Nodes::PublishedAt).timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_nodes_tenant_id")
                            .from(Nodes::Table, Nodes::TenantId)
                            .to(Tenants::Table, Tenants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_nodes_parent_id")
                            .from(Nodes::Table, Nodes::ParentId)
                            .to(Nodes::Table, Nodes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_nodes_author_id")
                            .from(Nodes::Table, Nodes::AuthorId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .index(
                        Index::create()
                            .name("idx_nodes_tenant_kind_status")
                            .table(Nodes::Table)
                            .col(Nodes::TenantId)
                            .col(Nodes::Kind)
                            .col(Nodes::Status),
                    )
                    .index(
                        Index::create()
                            .name("idx_nodes_parent")
                            .table(Nodes::Table)
                            .col(Nodes::ParentId),
                    )
                    .index(
                        Index::create()
                            .name("idx_nodes_author")
                            .table(Nodes::Table)
                            .col(Nodes::AuthorId),
                    )
                    .index(
                        Index::create()
                            .name("idx_nodes_published")
                            .table(Nodes::Table)
                            .col(Nodes::TenantId)
                            .col(Nodes::Kind)
                            .col(Nodes::PublishedAt),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(NodesTranslations::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(NodesTranslations::NodeId).uuid().not_null())
                    .col(ColumnDef::new(NodesTranslations::TenantId).uuid().not_null())
                    .col(
                        ColumnDef::new(NodesTranslations::Locale)
                            .string_len(10)
                            .not_null(),
                    )
                    .col(ColumnDef::new(NodesTranslations::Title).string_len(255))
                    .col(ColumnDef::new(NodesTranslations::Slug).string_len(255))
                    .col(ColumnDef::new(NodesTranslations::Excerpt).text())
                    .col(
                        ColumnDef::new(NodesTranslations::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(NodesTranslations::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(NodesTranslations::NodeId)
                            .col(NodesTranslations::Locale),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_nodes_translations_node_id")
                            .from(NodesTranslations::Table, NodesTranslations::NodeId)
                            .to(Nodes::Table, Nodes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_nodes_translations_tenant_id")
                            .from(NodesTranslations::Table, NodesTranslations::TenantId)
                            .to(Tenants::Table, Tenants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("idx_nodes_translations_slug")
                            .table(NodesTranslations::Table)
                            .col(NodesTranslations::TenantId)
                            .col(NodesTranslations::Locale)
                            .col(NodesTranslations::Slug),
                    )
                    .index(
                        Index::create()
                            .name("idx_nodes_translations_tenant_locale")
                            .table(NodesTranslations::Table)
                            .col(NodesTranslations::TenantId)
                            .col(NodesTranslations::Locale),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Bodies::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Bodies::NodeId).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(Bodies::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_bodies_node_id")
                            .from(Bodies::Table, Bodies::NodeId)
                            .to(Nodes::Table, Nodes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(BodiesTranslations::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(BodiesTranslations::NodeId).uuid().not_null())
                    .col(ColumnDef::new(BodiesTranslations::TenantId).uuid().not_null())
                    .col(
                        ColumnDef::new(BodiesTranslations::Locale)
                            .string_len(10)
                            .not_null(),
                    )
                    .col(ColumnDef::new(BodiesTranslations::Body).text())
                    .col(
                        ColumnDef::new(BodiesTranslations::Format)
                            .string_len(16)
                            .not_null()
                            .default("markdown"),
                    )
                    .col(
                        ColumnDef::new(BodiesTranslations::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(BodiesTranslations::NodeId)
                            .col(BodiesTranslations::Locale),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_bodies_translations_node_id")
                            .from(BodiesTranslations::Table, BodiesTranslations::NodeId)
                            .to(Bodies::Table, Bodies::NodeId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_bodies_translations_tenant_id")
                            .from(BodiesTranslations::Table, BodiesTranslations::TenantId)
                            .to(Tenants::Table, Tenants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_bodies_translations_tenant_locale")
                            .table(BodiesTranslations::Table)
                            .col(BodiesTranslations::TenantId)
                            .col(BodiesTranslations::Locale),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Tags::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Tags::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Tags::TenantId).uuid().not_null())
                    .col(
                        ColumnDef::new(Tags::UseCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Tags::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tags_tenant_id")
                            .from(Tags::Table, Tags::TenantId)
                            .to(Tenants::Table, Tenants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_tags_tenant")
                            .table(Tags::Table)
                            .col(Tags::TenantId),
                    )
                    .index(
                        Index::create()
                            .name("idx_tags_popular")
                            .table(Tags::Table)
                            .col(Tags::TenantId)
                            .col(Tags::UseCount),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TagsTranslations::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TagsTranslations::TagId).uuid().not_null())
                    .col(ColumnDef::new(TagsTranslations::TenantId).uuid().not_null())
                    .col(
                        ColumnDef::new(TagsTranslations::Locale)
                            .string_len(10)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TagsTranslations::Name)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TagsTranslations::Slug)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TagsTranslations::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(TagsTranslations::TagId)
                            .col(TagsTranslations::Locale),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tags_translations_tag_id")
                            .from(TagsTranslations::Table, TagsTranslations::TagId)
                            .to(Tags::Table, Tags::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tags_translations_tenant_id")
                            .from(TagsTranslations::Table, TagsTranslations::TenantId)
                            .to(Tenants::Table, Tenants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("idx_tags_translations_slug")
                            .table(TagsTranslations::Table)
                            .col(TagsTranslations::TenantId)
                            .col(TagsTranslations::Locale)
                            .col(TagsTranslations::Slug),
                    )
                    .index(
                        Index::create()
                            .name("idx_tags_translations_tenant_locale")
                            .table(TagsTranslations::Table)
                            .col(TagsTranslations::TenantId)
                            .col(TagsTranslations::Locale),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Meta::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Meta::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Meta::TenantId).uuid().not_null())
                    .col(ColumnDef::new(Meta::TargetType).string_len(32).not_null())
                    .col(ColumnDef::new(Meta::TargetId).uuid().not_null())
                    .col(ColumnDef::new(Meta::OgImage).string_len(500))
                    .col(ColumnDef::new(Meta::OgType).string_len(32))
                    .col(ColumnDef::new(Meta::TwitterCard).string_len(32))
                    .col(
                        ColumnDef::new(Meta::NoIndex)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Meta::NoFollow)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Meta::CanonicalUrl).string_len(500))
                    .col(ColumnDef::new(Meta::StructuredData).json_binary())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_meta_tenant_id")
                            .from(Meta::Table, Meta::TenantId)
                            .to(Tenants::Table, Tenants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("idx_meta_target")
                            .table(Meta::Table)
                            .col(Meta::TargetType)
                            .col(Meta::TargetId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MetaTranslations::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(MetaTranslations::MetaId).uuid().not_null())
                    .col(ColumnDef::new(MetaTranslations::TenantId).uuid().not_null())
                    .col(
                        ColumnDef::new(MetaTranslations::Locale)
                            .string_len(10)
                            .not_null(),
                    )
                    .col(ColumnDef::new(MetaTranslations::Title).string_len(255))
                    .col(ColumnDef::new(MetaTranslations::Description).string_len(500))
                    .col(ColumnDef::new(MetaTranslations::Keywords).string_len(255))
                    .col(ColumnDef::new(MetaTranslations::OgTitle).string_len(255))
                    .col(ColumnDef::new(MetaTranslations::OgDescription).string_len(500))
                    .col(
                        ColumnDef::new(MetaTranslations::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(MetaTranslations::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(MetaTranslations::MetaId)
                            .col(MetaTranslations::Locale),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_meta_translations_meta_id")
                            .from(MetaTranslations::Table, MetaTranslations::MetaId)
                            .to(Meta::Table, Meta::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_meta_translations_tenant_id")
                            .from(MetaTranslations::Table, MetaTranslations::TenantId)
                            .to(Tenants::Table, Tenants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_meta_translations_tenant_locale")
                            .table(MetaTranslations::Table)
                            .col(MetaTranslations::TenantId)
                            .col(MetaTranslations::Locale),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Media::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Media::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Media::TenantId).uuid().not_null())
                    .col(ColumnDef::new(Media::UploadedBy).uuid().null())
                    .col(ColumnDef::new(Media::Filename).string_len(255).not_null())
                    .col(ColumnDef::new(Media::OriginalName).string_len(255).not_null())
                    .col(ColumnDef::new(Media::MimeType).string_len(100).not_null())
                    .col(ColumnDef::new(Media::Size).big_integer().not_null())
                    .col(ColumnDef::new(Media::StoragePath).string_len(500).not_null())
                    .col(
                        ColumnDef::new(Media::StorageDriver)
                            .string_len(32)
                            .not_null()
                            .default("local"),
                    )
                    .col(ColumnDef::new(Media::Width).integer())
                    .col(ColumnDef::new(Media::Height).integer())
                    .col(
                        ColumnDef::new(Media::Metadata)
                            .json_binary()
                            .not_null()
                            .default("{}"),
                    )
                    .col(
                        ColumnDef::new(Media::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_media_tenant_id")
                            .from(Media::Table, Media::TenantId)
                            .to(Tenants::Table, Tenants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_media_uploaded_by")
                            .from(Media::Table, Media::UploadedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .index(
                        Index::create()
                            .name("idx_media_tenant")
                            .table(Media::Table)
                            .col(Media::TenantId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MediaTranslations::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(MediaTranslations::MediaId).uuid().not_null())
                    .col(ColumnDef::new(MediaTranslations::TenantId).uuid().not_null())
                    .col(
                        ColumnDef::new(MediaTranslations::Locale)
                            .string_len(10)
                            .not_null(),
                    )
                    .col(ColumnDef::new(MediaTranslations::AltText).string_len(255))
                    .col(
                        ColumnDef::new(MediaTranslations::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(MediaTranslations::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(MediaTranslations::MediaId)
                            .col(MediaTranslations::Locale),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_media_translations_media_id")
                            .from(MediaTranslations::Table, MediaTranslations::MediaId)
                            .to(Media::Table, Media::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_media_translations_tenant_id")
                            .from(MediaTranslations::Table, MediaTranslations::TenantId)
                            .to(Tenants::Table, Tenants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_media_translations_tenant_locale")
                            .table(MediaTranslations::Table)
                            .col(MediaTranslations::TenantId)
                            .col(MediaTranslations::Locale),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MediaTranslations::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Media::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MetaTranslations::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Meta::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(TagsTranslations::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Tags::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(BodiesTranslations::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Bodies::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(NodesTranslations::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Nodes::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Nodes {
    Table,
    Id,
    TenantId,
    ParentId,
    AuthorId,
    Kind,
    Status,
    Position,
    Depth,
    ReplyCount,
    Metadata,
    CreatedAt,
    UpdatedAt,
    PublishedAt,
}

#[derive(Iden)]
enum NodesTranslations {
    Table,
    NodeId,
    TenantId,
    Locale,
    Title,
    Slug,
    Excerpt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Bodies {
    Table,
    NodeId,
    UpdatedAt,
}

#[derive(Iden)]
enum BodiesTranslations {
    Table,
    NodeId,
    TenantId,
    Locale,
    Body,
    Format,
    UpdatedAt,
}

#[derive(Iden)]
enum Tags {
    Table,
    Id,
    TenantId,
    UseCount,
    CreatedAt,
}

#[derive(Iden)]
enum TagsTranslations {
    Table,
    TagId,
    TenantId,
    Locale,
    Name,
    Slug,
    CreatedAt,
}

#[derive(Iden)]
enum Meta {
    Table,
    Id,
    TenantId,
    TargetType,
    TargetId,
    OgImage,
    OgType,
    TwitterCard,
    NoIndex,
    NoFollow,
    CanonicalUrl,
    StructuredData,
}

#[derive(Iden)]
enum MetaTranslations {
    Table,
    MetaId,
    TenantId,
    Locale,
    Title,
    Description,
    Keywords,
    OgTitle,
    OgDescription,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Media {
    Table,
    Id,
    TenantId,
    UploadedBy,
    Filename,
    OriginalName,
    MimeType,
    Size,
    StoragePath,
    StorageDriver,
    Width,
    Height,
    Metadata,
    CreatedAt,
}

#[derive(Iden)]
enum MediaTranslations {
    Table,
    MediaId,
    TenantId,
    Locale,
    AltText,
    CreatedAt,
    UpdatedAt,
}
