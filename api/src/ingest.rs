use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use clap::{Args, ValueHint};
use color_eyre::{Result, eyre::Context};
use itertools::Itertools;
use resolve_path::PathResolveExt;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    Database, DatabaseConnection, EntityTrait,
};
use serde::{Deserialize, Serialize};

use db::entities;

#[derive(Args, Debug)]
pub struct IngestData {
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    ingredients: PathBuf,

    #[clap(short, long, value_hint = ValueHint::FilePath)]
    preparations: PathBuf,

    #[clap(short, long, value_hint = ValueHint::FilePath)]
    units: PathBuf,

    #[clap(short, long, value_hint = ValueHint::FilePath)]
    recipes: PathBuf,
}

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

fn load<'de, T: Deserialize<'de>>(filepath: PathBuf) -> Result<T> {
    let jd = &mut serde_json::Deserializer::from_reader(BufReader::new(
        File::open(&filepath.resolve()).wrap_err_with(|| format!("Opening file {filepath:?}"))?,
    ));

    serde_path_to_error::deserialize(jd).wrap_err_with(|| format!("Parsing {filepath:?}"))
}

async fn insert_ingredients(
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

async fn insert_preparations(
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

async fn insert_units(db: &DatabaseConnection, units: &[IngestUnit]) -> Result<()> {
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

async fn insert_ingredient_units(
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

async fn insert_recipes(db: &DatabaseConnection, recipes: &[types::Recipe]) -> Result<()> {
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
            published_at: Set(r.published_at),
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

pub async fn run(
    IngestData {
        ingredients,
        preparations,
        units,
        recipes,
    }: IngestData,
) -> Result<()> {
    let db = Database::connect("sqlite://db.sqlite").await?;

    let ingredients: Vec<IngestIngredient> = load(ingredients)?;
    let preparations: Vec<types::ReferencePreparation> = load(preparations)?;
    let units: Vec<IngestUnit> = load(units)?;
    let recipes: Vec<types::Recipe> = load(recipes)?;

    insert_ingredients(&db, &ingredients)
        .await
        .context("Ingesting ingredients")?;
    insert_preparations(&db, &preparations)
        .await
        .context("Ingesting preparations")?;
    insert_units(&db, &units).await.context("Ingesting units")?;
    insert_ingredient_units(&db, &ingredients)
        .await
        .context("Ingesting ingredient allowed units")?;
    insert_recipes(&db, &recipes)
        .await
        .context("Ingesting recipes")?;

    Ok(())
}
