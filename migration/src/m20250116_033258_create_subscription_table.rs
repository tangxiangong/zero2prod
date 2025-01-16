use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Subscription::Table)
                    .if_not_exists()
                    .col(
                        big_unsigned(Subscription::Id)
                            .primary_key()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        string(Subscription::Email)
                            .string_len(50)
                            .not_null()
                            .unique_key(),
                    )
                    .col(string(Subscription::Name).string_len(50).not_null())
                    .col(
                        timestamp(Subscription::SubscribedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Subscription::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Subscription {
    Table,
    Id,
    Email,
    Name,
    #[sea_orm(iden = "subscribed_at")]
    SubscribedAt,
}
