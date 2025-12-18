use color_eyre::eyre::OptionExt as _;
use sea_orm::DatabaseConnection;
use sea_orm::EntityLoaderTrait;
use sea_orm::EntityLoaderTrait as _;
use sea_orm::EntityTrait as _;
use sea_orm::ModelTrait;
use sea_orm::QueryFilter;

use crate::entities::prelude::*;

pub async fn list_preparations(
    db: &DatabaseConnection,
    offset: Option<u64>,
    limit: Option<u64>,
) -> color_eyre::Result<Vec<types::ReferencePreparation>> {
    let mut preparation_models_q = Preparation::find().order_by_id_asc();

    if let Some(offset) = offset {
        preparation_models_q.query().offset(offset);
    }

    if let Some(limit) = limit {
        preparation_models_q.query().limit(limit);
    }

    let preparation_models = preparation_models_q.all(db).await?;

    let preparations = preparation_models
        .into_iter()
        .map(|r| {
            Ok(types::ReferencePreparation {
                id: r.id,
                name: r.name,
            })
        })
        .collect::<color_eyre::Result<Vec<_>>>()?;

    Ok(preparations)
}
