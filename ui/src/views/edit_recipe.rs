use std::collections::HashMap;

use crate::components::{
    card::*, input::Input, label::Label, tabs::*,
    textarea::Textarea,
};
use dioxus::prelude::*;

#[component]
fn Ingredient(
    ingredient: WriteSignal<types::RecipeIngredient>,
    ingredient_map: Memo<HashMap<String, types::Ingredient>>,
) -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "{ingredient.read().reference_ingredient.name}" }
            }
        }
    }
}

#[component]
pub fn EditRecipe(id: String) -> Element {
    let recipe_initial = use_loader(move || recipe_server(id.clone()))?.cloned();
    let mut recipe = use_signal(move || recipe_initial);

    let ingredients = use_loader(ingredients_server)?;
    let ingredients_map = use_memo(move || {
        ingredients
            .iter()
            .map(|i| (i.name.clone(), i.clone()))
            .collect::<HashMap<_, _>>()
    });

    rsx! {

        div {
            class: "flex justify-center",

            div {
                class: "flex flex-col w-3/4 gap-4 justify-center",

                Label { html_for: "recipe_name", "Name" }
                Input {
                    id: "recipe_name",
                    value: recipe.read().name.clone(),
                    oninput: move |e: FormEvent| recipe.write().name = e.value(),
                }

                Label { html_for: "recipe_description", "Description" }
                Textarea {
                    id: "recipe_description",
                    placeholder: recipe.read().description.clone(),
                    value: recipe.read().description.clone(),
                    oninput: move |e: FormEvent| recipe.write().description = e.value(),
                }

                Label { html_for: "recipe_prep_time", "Prep time" }
                div {
                    class: "flex flex-row items-center gap-4",

                    Input {
                        id: "recipe_prep_time",
                        r#type: "number",
                        min: 0,
                        value: recipe
                            .read()
                            .prep_time
                            .map(|x| x.as_mins())
                            .map(|x| x.to_string())
                            .unwrap_or(String::new()),
                        oninput: move |e: FormEvent| {
                            if let Ok(t) = e.value().parse::<i64>() {
                                recipe.write().prep_time = Some(jiff::SignedDuration::from_mins(t));
                            }
                        }
                    }
                    span {
                        "Seconds"
                    }
                }

                Label { html_for: "recipe_cook_time", "Cook time" }
                div {
                    class: "flex flex-row items-center gap-4",

                    Input {
                        id: "recipe_cook_time",
                        r#type: "number",
                        min: 0,
                        value: recipe
                            .read()
                            .cook_time
                            .map(|x| x.as_mins())
                            .map(|x| x.to_string())
                            .unwrap_or(String::new()),
                        oninput: move |e: FormEvent| {
                            if let Ok(t) = e.value().parse::<i64>() {
                                recipe.write().cook_time = Some(jiff::SignedDuration::from_mins(t));
                            }
                        }
                    }
                    span {
                        "Seconds"
                    }
                }

                Label { html_for: "recipe_serves", "Serves" }
                div {
                    class: "flex flex-row items-center gap-4",

                    Input {
                        id: "recipe_serves",
                        r#type: "number",
                        min: 0,
                        value: recipe.read().serves,
                        oninput: move |e: FormEvent| {
                            if let Ok(x) = e.value().parse::<u8>() {
                                recipe.write().serves = x;
                            }
                        }
                    }
                    span {
                        "People"
                    }
                }

                Tabs {
                    default_value: "ingredients",

                    TabList {
                        TabTrigger { value: "ingredients", index: 0usize, "Ingredients" },
                        TabTrigger { value: "steps", index: 1usize, "Steps" },
                    }

                    TabContent { index: 0usize, value: "ingredients",
                        div {
                            class: "w-100 flex flex-col gap-4 p-4",

                            for idx in 0..recipe.read().ingredients.len() {
                                Ingredient {
                                    ingredient: recipe.map_mut(
                                        move |r| &r.ingredients[idx],
                                        move |r| &mut r.ingredients[idx]
                                    ),
                                    ingredient_map: ingredients_map,
                                }
                            }
                        }
                    }

                    TabContent { index: 1usize, value: "steps",
                        div {
                            width: "100%",
                            height: "5rem",
                            display: "flex",
                            align_items: "center",
                            justify_content: "center",
                            "Tab 2 Content"
                        }
                    }
                }
            }
        }
    }
}

#[server]
async fn recipe_server(id: String) -> Result<types::Recipe> {
    use dioxus::{
        logger::tracing::{info_span, Instrument as _},
        CapturedError,
    };

    let recipe = db::queries::recipes::get_recipe(crate::db::db(), &id)
        .instrument(info_span!("Loading recipe"))
        .await
        .map_err(|e| CapturedError::from_boxed(e.into()))?;

    Ok(recipe)
}

#[server]
async fn ingredients_server() -> Result<Vec<types::Ingredient>> {
    use dioxus::{
        logger::tracing::{info_span, Instrument as _},
        CapturedError,
    };

    let recipe = db::queries::ingredients::list_ingredients(crate::db::db(), None, None)
        .instrument(info_span!("Loading ingredients"))
        .await
        .map_err(|e| CapturedError::from_boxed(e.into()))?;

    Ok(recipe)
}

#[server]
async fn allowed_units_server(ingredient_id: String) -> Result<Vec<types::IngredientAllowedUnit>> {
    use dioxus::{
        logger::tracing::{info_span, Instrument as _},
        CapturedError,
    };

    let recipe = db::queries::ingredients::list_ingredient_allowed(crate::db::db(), &ingredient_id)
        .instrument(info_span!("Loading ingredient allowed"))
        .await
        .map_err(|e| CapturedError::from_boxed(e.into()))?;

    Ok(recipe)
}
