use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Ingredient::Table)
                    .if_not_exists()
                    .col(string(Ingredient::Id).primary_key().not_null())
                    .col(string(Ingredient::Name).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Unit::Table)
                    .if_not_exists()
                    .col(string(Unit::Id).primary_key().not_null())
                    .col(string(Unit::Name).not_null())
                    .col(string_null(Unit::Abbreviation).null())
                    .col(string_null(Unit::DimensionId).null())
                    .col(string_null(Unit::MeasurementSystemId).null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(IngredientUnit::Table)
                    .if_not_exists()
                    .col(string(IngredientUnit::IngredientId).not_null())
                    .col(string(IngredientUnit::UnitId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_ingredient_unit_ingredient_id")
                            .from(IngredientUnit::Table, IngredientUnit::IngredientId)
                            .to(Ingredient::Table, Ingredient::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_ingredient_unit_unit_id")
                            .from(IngredientUnit::Table, IngredientUnit::UnitId)
                            .to(Unit::Table, Unit::Id),
                    )
                    .primary_key(
                        Index::create()
                            .primary()
                            .col(IngredientUnit::IngredientId)
                            .col(IngredientUnit::UnitId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Preparation::Table)
                    .if_not_exists()
                    .col(string(Preparation::Id).primary_key().not_null())
                    .col(string(Preparation::Name).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Author::Table)
                    .if_not_exists()
                    .col(
                        integer(Author::Id)
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(string(Author::Name).not_null())
                    .col(string(Author::Image).not_null())
                    .col(string(Author::Url).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Recipe::Table)
                    .if_not_exists()
                    .col(string(Recipe::Id).primary_key().not_null())
                    .col(string(Recipe::Name).not_null())
                    .col(string(Recipe::Description).not_null())
                    .col(string_null(Recipe::PrepTime).null())
                    .col(string_null(Recipe::CookTime).null())
                    .col(string(Recipe::TotalTime).not_null())
                    .col(integer(Recipe::AuthorId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_recipe_author_id")
                            .from(Recipe::Table, Recipe::AuthorId)
                            .to(Author::Table, Author::Id),
                    )
                    .col(integer(Recipe::Serves).not_null())
                    .col(string(Recipe::ETag).not_null())
                    .col(string(Recipe::OrganisationId).not_null())
                    .col(string(Recipe::Locale).not_null())
                    .col(
                        timestamp(Recipe::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp(Recipe::ModifiedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp(Recipe::PublishedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(string(Recipe::CreatedById).not_null())
                    .col(json(Recipe::Steps).not_null())
                    .col(json(Recipe::Ingredients).not_null())
                    .col(boolean(Recipe::IsCustom).not_null())
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();

        db.execute_unprepared(
            "CREATE TRIGGER recipe_modified_at
             AFTER UPDATE ON recipe
             FOR EACH ROW
             BEGIN
                 UPDATE recipe
                 SET modified_at = (datetime('now','localtime'))
                 WHERE id = NEW.id;
             END;",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Recipe::Table)
                    .table(Author::Table)
                    .table(Preparation::Table)
                    .table(IngredientUnit::Table)
                    .table(Unit::Table)
                    .table(Ingredient::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Ingredient {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum Unit {
    Table,
    Id,
    Name,
    Abbreviation,
    DimensionId,
    MeasurementSystemId,
}

#[derive(DeriveIden)]
enum IngredientUnit {
    Table,
    IngredientId,
    UnitId,
}

#[derive(DeriveIden)]
enum Preparation {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum Author {
    Table,
    Id,
    Name,
    Image,
    Url,
}

#[derive(DeriveIden)]
enum Recipe {
    Table,
    Id,
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
