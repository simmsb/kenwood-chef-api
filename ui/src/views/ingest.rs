use core::str::FromStr;
use rand::distr::SampleString;
use std::{string::ToString, time::Duration};

use dioxus::{fullstack::MultipartFormData, prelude::*};
use dioxus_primitives::{
    checkbox::CheckboxState,
    toast::{consume_toast, ToastOptions},
};
use itertools::Itertools;
use types::KnownOptions;
use types::{traits::*, UID};

use crate::components::{
    button::{Button, ButtonVariant},
    card::*,
    checkbox::*,
    input::Input,
    label::Label,
    searching_select::*,
    select,
    tabs::*,
    textarea::Textarea,
};

#[component]
pub fn Ingest() -> Element {
    rsx! {
        form {
            class: "flex flex-col gap-4",

            onsubmit: move |e: FormEvent| async move {
                e.prevent_default();

                upload_data(e.into()).await.unwrap();

                let toast_api = consume_toast();

                toast_api
                    .info(
                        "Uploaded data".to_owned(),
                        ToastOptions::new().duration(Duration::from_secs(3)),
                    );
            },

            Label { html_for: "ingredients", "Ingredients" }
            Input { r#type: "file", name: "ingredients", accept: ".json" }
            Label { html_for: "preparations", "Preparations" }
            Input { r#type: "file", name: "preparations", accept: ".json" }
            Label { html_for: "units", "Units" }
            Input { r#type: "file", name: "units", accept: ".json" }
            Label { html_for: "recipes", "Recipes" }
            Input { r#type: "file", name: "recipes", accept: ".json" }
            Input { r#type: "submit", name: "submit", value: "Upload" }
        }
    }
}

fn load<'de, T: serde::Deserialize<'de>>(data: &'de [u8], name: &str) -> Result<T> {
    let jd = &mut serde_json::Deserializer::from_slice(data);

    Ok(serde_path_to_error::deserialize(jd)?)
}

#[server]
#[middleware(dioxus::fullstack::axum_core::extract::DefaultBodyLimit::max(1024 * 1024 * 256))]
async fn upload_data(mut form: MultipartFormData) -> Result<()> {
    use db::queries::ingest;
    use dioxus::CapturedError;

    let mut ingredients = None;
    let mut preparations = None;
    let mut units = None;
    let mut recipes = None;

    while let Ok(Some(field)) = form.next_field().await {
        if field.name() == Some("ingredients") {
            ingredients = Some(field.bytes().await?);
            continue;
        }

        if field.name() == Some("preparations") {
            preparations = Some(field.bytes().await?);
            continue;
        }

        if field.name() == Some("units") {
            units = Some(field.bytes().await?);
            continue;
        }

        if field.name() == Some("recipes") {
            recipes = Some(field.bytes().await?);
            continue;
        }
    }

    let (Some(ingredients), Some(preparations), Some(units), Some(recipes)) =
        (ingredients, preparations, units, recipes)
    else {
        return Err(CapturedError::from_display("Couldn't get form fields"));
    };

    let db = crate::db::db();

    let ingredients: Vec<db::queries::ingest::IngestIngredient> =
        load(&ingredients, "ingredients")?;
    let preparations: Vec<types::ReferencePreparation> = load(&preparations, "preparations")?;
    let units: Vec<db::queries::ingest::IngestUnit> = load(&units, "units")?;
    let recipes: Vec<types::Recipe> = load(&recipes, "recipes")?;

    ingest::insert_ingredients(db, &ingredients)
        .await
        .map_err(|e| CapturedError::from_boxed(e.into()))?;
    ingest::insert_preparations(db, &preparations)
        .await
        .map_err(|e| CapturedError::from_boxed(e.into()))?;
    ingest::insert_units(db, &units)
        .await
        .map_err(|e| CapturedError::from_boxed(e.into()))?;
    ingest::insert_ingredient_units(&db, &ingredients)
        .await
        .map_err(|e| CapturedError::from_boxed(e.into()))?;
    ingest::insert_recipes(db, &recipes)
        .await
        .map_err(|e| CapturedError::from_boxed(e.into()))?;

    Ok(())
}
