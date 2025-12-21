use color_eyre::eyre::OptionExt as _;
use migration::OnConflict;
use sea_orm::EntityLoaderTrait;
use sea_orm::EntityLoaderTrait as _;
use sea_orm::EntityTrait as _;
use sea_orm::{ActiveValue::Set, DatabaseConnection};

use crate::entities::{image, prelude::*};

pub async fn get_image(db: &DatabaseConnection, image_id: &str) -> color_eyre::Result<Vec<u8>> {
    let image = Image::load()
        .filter_by_id(image_id)
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
