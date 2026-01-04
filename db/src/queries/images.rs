use color_eyre::eyre::OptionExt as _;
use migration::OnConflict;
use sea_orm::EntityLoaderTrait as _;
use sea_orm::EntityTrait as _;
use sea_orm::{ActiveValue::Set, DatabaseConnection};
use sea_orm::{
    ColumnTrait as _, Condition, DerivePartialModel, EntityLoaderTrait, QueryFilter, QuerySelect,
};

use crate::entities::{image, prelude::*, recipe};

#[derive(DerivePartialModel)]
#[sea_orm(entity = "recipe::Entity")]
struct RecipeIdOnly {
    id: String,
}

pub async fn get_image(db: &DatabaseConnection, image_id: &str) -> color_eyre::Result<Vec<u8>> {
    let recipe: RecipeIdOnly = recipe::Entity::find()
        .filter(
            Condition::any()
                .add(recipe::Column::Id.eq(image_id))
                .add(recipe::Column::ExposedId.eq(image_id)),
        )
        .into_partial_model()
        .one(db)
        .await?
        .ok_or_eyre("Image not found")?;

    let recipe_id = recipe.id;

    let image = Image::load()
        .filter_by_id(recipe_id)
        .one(db)
        .await?
        .ok_or_eyre("Image not found")?;

    Ok(image.data)
}

pub async fn set_image(
    db: &DatabaseConnection,
    image_id: &str,
    data: Vec<u8>,
) -> color_eyre::Result<()> {
    Image::insert(image::ActiveModel {
        id: Set(image_id.to_owned()),
        data: Set(data),
    })
    .on_conflict(
        OnConflict::column(image::Column::Id)
            .update_column(image::Column::Data)
            .to_owned(),
    )
    .exec(db)
    .await?;

    Ok(())
}
