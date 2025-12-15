use color_eyre::eyre::OptionExt as _;
use sea_orm::DatabaseConnection;
use sea_orm::EntityLoaderTrait;
use sea_orm::EntityLoaderTrait as _;
use sea_orm::EntityTrait as _;
use sea_orm::ModelTrait;
use sea_orm::QueryFilter;

use crate::entities::prelude::*;

pub async fn list_ingredients(
    db: &DatabaseConnection,
    offset: Option<u64>,
    limit: Option<u64>,
) -> color_eyre::Result<Vec<types::Ingredient>> {
    let mut ingredient_models_q = Ingredient::find().order_by_id_asc();

    if let Some(offset) = offset {
        ingredient_models_q.query().offset(offset);
    }

    if let Some(limit) = limit {
        ingredient_models_q.query().limit(limit);
    }

    let ingredient_models = ingredient_models_q.all(db).await?;

    let ingredients = ingredient_models
        .into_iter()
        .map(|r| {
            Ok(types::Ingredient {
                id: r.id,
                name: r.name,
            })
        })
        .collect::<color_eyre::Result<Vec<_>>>()?;

    Ok(ingredients)
}

pub async fn list_ingredient_allowed(
    db: &DatabaseConnection,
    id: &str,
) -> color_eyre::Result<Vec<types::IngredientAllowedUnit>> {
    let ingredient = Ingredient::find_by_id(id)
        .one(db)
        .await?
        .ok_or_eyre("Couldn't find ingredient")?;
    let allowed_model = ingredient.find_related(Unit).all(db).await?;

    let allowed = allowed_model
        .into_iter()
        .map(|u| {
            Ok(types::IngredientAllowedUnit {
                id: u.id,
                name: u.name,
                abbreviation: u.abbreviation,
                dimension: u.dimension_id,
            })
        })
        .collect::<color_eyre::Result<Vec<_>>>()?;

    Ok(allowed)
}
