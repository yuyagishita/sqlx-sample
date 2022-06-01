use entity::document::*;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220601_220000_create_documents"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                .table(Entity)
                .if_not_exists()
                .col(ColumnDef::new(Column::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(Column::Title).string().not_null())
                .col(ColumnDef::new(Column::Body).text().not_null())
                .to_owned()
                ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                .table(Entity)
                .to_owned()
                ).await
    }
}
