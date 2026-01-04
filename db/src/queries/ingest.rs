use std::collections::HashMap;

use color_eyre::Result;
use itertools::Itertools;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    DatabaseConnection, EntityTrait,
};
use serde::{Deserialize, Serialize};

use crate::entities;

#[derive(Serialize, Deserialize, Clone)]
pub struct IngestIngredientUnit {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IngestIngredient {
    pub id: String,
    pub name: String,
    pub allowed_units: Vec<IngestIngredientUnit>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IngestMeasurementSystem {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IngestUnit {
    pub id: String,
    pub name: String,
    pub abbreviation: Option<String>,
    pub dimension: Option<String>,
    pub measurement_system: Option<IngestMeasurementSystem>,
}

pub async fn insert_ingredients(
    db: &DatabaseConnection,
    ingredients: &[IngestIngredient],
) -> Result<()> {
    entities::prelude::Ingredient::insert_many(ingredients.iter().map(|i| {
        entities::ingredient::ActiveModel {
            id: Set(i.id.clone()),
            name: Set(i.name.clone()),
        }
    }))
    .on_conflict_do_nothing()
    .exec(db)
    .await?;

    Ok(())
}

pub async fn insert_preparations(
    db: &DatabaseConnection,
    preparations: &[types::ReferencePreparation],
) -> Result<()> {
    entities::prelude::Preparation::insert_many(preparations.iter().map(|i| {
        entities::preparation::ActiveModel {
            id: Set(i.id.clone()),
            name: Set(i.name.clone()),
        }
    }))
    .on_conflict_do_nothing()
    .exec(db)
    .await?;

    Ok(())
}

pub async fn insert_units(db: &DatabaseConnection, units: &[IngestUnit]) -> Result<()> {
    entities::prelude::Unit::insert_many(units.iter().map(|i| entities::unit::ActiveModel {
        id: Set(i.id.clone()),
        name: Set(i.name.clone()),
        abbreviation: Set(i.abbreviation.clone()),
        dimension_id: Set(i.dimension.clone()),
        measurement_system_id: Set(i.measurement_system.clone().map(|m| m.id)),
    }))
    .on_conflict_do_nothing()
    .exec(db)
    .await?;

    Ok(())
}

pub async fn insert_ingredient_units(
    db: &DatabaseConnection,
    ingredients: &[IngestIngredient],
) -> Result<()> {
    for chunk in &ingredients
        .iter()
        .flat_map(|i| {
            i.allowed_units
                .iter()
                .map(|u| entities::ingredient_unit::ActiveModel {
                    ingredient_id: Set(i.id.clone()),
                    unit_id: Set(u.id.clone()),
                })
        })
        .chunks(1000)
    {
        entities::prelude::IngredientUnit::insert_many(chunk)
            .on_conflict_do_nothing()
            .exec(db)
            .await?;
    }

    Ok(())
}

pub async fn insert_recipes(db: &DatabaseConnection, recipes: &[types::Recipe]) -> Result<()> {
    let authors = recipes.iter().map(|r| &r.author).unique_by(|a| &a.name);

    entities::prelude::Author::insert_many(authors.map(|a| entities::author::ActiveModel {
        id: NotSet,
        name: Set(a.name.clone()),
        image: Set(a.image.clone()),
        url: Set(a.image.clone()),
    }))
    .exec(db)
    .await?;

    let authors = entities::prelude::Author::find()
        .all(db)
        .await?
        .into_iter()
        .map(|a| (a.name, a.id))
        .collect::<HashMap<_, _>>();

    for chunk in &recipes
        .iter()
        .map(|r| entities::recipe::ActiveModel {
            id: Set(r.id.clone()),
            exposed_id: NotSet,
            name: Set(r.name.clone()),
            description: Set(r.description.clone()),
            prep_time: Set(r.prep_time.map(|x| x.to_string())),
            cook_time: Set(r.cook_time.map(|x| x.to_string())),
            total_time: Set(r.total_time.to_string()),
            author_id: Set(*authors.get(&r.author.name).unwrap()),
            serves: Set(r.serves as i64),
            e_tag: Set(r.etag.clone()),
            organisation_id: Set(r.organization_id.clone()),
            locale: Set(r.locale.clone()),
            created_at: Set(r.created_at),
            modified_at: Set(r.modified_at),
            published_at: Set(r.published_at.expect("Imported recipes should be published")),
            created_by_id: Set(r.created_by_id.clone()),
            steps: Set(serde_json::to_value(&r.steps).unwrap()),
            ingredients: Set(serde_json::to_value(&r.ingredients).unwrap()),
            is_custom: Set(false),
        })
        .chunks(1000)
    {
        entities::prelude::Recipe::insert_many(chunk)
            .on_conflict_do_nothing()
            .exec(db)
            .await?;
    }

    Ok(())
}
