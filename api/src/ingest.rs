use std::{fs::File, io::BufReader, path::PathBuf};

use clap::{Args, ValueHint};
use color_eyre::{Result, eyre::Context};
use resolve_path::PathResolveExt;
use sea_orm::Database;
use serde::Deserialize;

use db::queries::ingest::{self, IngestIngredient, IngestUnit};

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

fn load<'de, T: Deserialize<'de>>(filepath: PathBuf) -> Result<T> {
    let jd = &mut serde_json::Deserializer::from_reader(BufReader::new(
        File::open(filepath.resolve()).wrap_err_with(|| format!("Opening file {filepath:?}"))?,
    ));

    serde_path_to_error::deserialize(jd).wrap_err_with(|| format!("Parsing {filepath:?}"))
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

    ingest::insert_ingredients(&db, &ingredients)
        .await
        .context("Ingesting ingredients")?;
    ingest::insert_preparations(&db, &preparations)
        .await
        .context("Ingesting preparations")?;
    ingest::insert_units(&db, &units)
        .await
        .context("Ingesting units")?;
    ingest::insert_ingredient_units(&db, &ingredients)
        .await
        .context("Ingesting ingredient allowed units")?;
    ingest::insert_recipes(&db, &recipes)
        .await
        .context("Ingesting recipes")?;

    Ok(())
}
