use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                TableAlterStatement::new()
                    .table(Recipe::Table)
                    .add_column(string_null(Recipe::ExposedId).null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("idx-recipe-exposed-id")
                    .table(Recipe::Table)
                    .col(Recipe::ExposedId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                IndexDropStatement::new()
                    .name("idx-recipe-exposed-id")
                    .table(Recipe::ExposedId)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                TableAlterStatement::new()
                    .table(Recipe::Table)
                    .drop_column(Recipe::ExposedId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Recipe {
    Table,
    Id,
    ExposedId,
    Name,
    Description,
    PrepTime,
    CookTime,
    TotalTime,
    AuthorId,
    Serves,
    ETag,
    OrganisationId,
    Locale,
    CreatedAt,
    ModifiedAt,
    PublishedAt,
    CreatedById,
    Steps,
    Ingredients,
    IsCustom,
}
