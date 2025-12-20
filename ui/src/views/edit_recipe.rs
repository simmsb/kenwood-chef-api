use core::str::FromStr;
use std::string::ToString;

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
use dioxus::prelude::*;
use dioxus_primitives::checkbox::CheckboxState;
use itertools::Itertools as _;
use types::KnownOptions;

#[component]
fn PreparationSelector(
    preparation: WriteSignal<types::ReferencePreparation>,
    preparations_matcher: Memo<FuzzyFinder<types::ReferencePreparation>>,
) -> Element {
    let search_input = use_signal(|| String::new());
    let matching_input = use_memo(move || {
        preparations_matcher
            .write()
            .search(search_input().as_str(), 10)
            .into_iter()
            .cloned()
            .collect_vec()
    });

    rsx! {
        SearchingSelect::<types::ReferencePreparation> {
            placeholder: "{preparation().name}",
            typeahead_buffer: search_input,
            on_value_change: move |v| {
                if let Some(v) = v {
                    *preparation.write() = v;
                }
            },

            SelectTrigger { SelectValue {} }

            SelectList {
                for (idx , preparation) in matching_input().iter().enumerate() {
                    SelectOption::<types::ReferencePreparation> {
                        index: idx,
                        value: preparation.clone(),
                        text_value: preparation.name.clone(),

                        "{preparation.name}"

                        SelectItemIndicator {}
                    }
                }
            }
        }
    }
}

#[component]
fn Ingredient(
    ingredient: WriteSignal<types::RecipeIngredient>,
    ingredients_matcher: Memo<FuzzyFinder<types::Ingredient>>,
    preparations_matcher: Memo<FuzzyFinder<types::ReferencePreparation>>,
) -> Element {
    let ingredient_search_input = use_signal(|| String::new());
    let unit_search_input = use_signal(|| String::new());

    let matching_ingredients = use_memo(move || {
        ingredients_matcher
            .write()
            .search(ingredient_search_input().as_str(), 10)
            .into_iter()
            .cloned()
            .collect_vec()
    });

    let allowed_units = use_loader(move || {
        allowed_units_server(ingredient.read().reference_ingredient.id.clone())
    })?;
    let mut units_matcher = use_memo(move || {
        FuzzyFinder::new(allowed_units.iter().map(|i| (i.name.clone(), i.clone())))
    });

    let matching_units = use_memo(move || {
        units_matcher
            .write()
            .search(unit_search_input().as_str(), 10)
            .into_iter()
            .cloned()
            .collect_vec()
    });

    let amount = use_memo(move || ingredient().quantity.amount);
    let reference_unit = use_memo(move || ingredient().quantity.reference_unit);

    use_effect(move || {
        if let Some(amount) = amount() {
            let abbr = reference_unit().abbreviation;
            let abbr = if abbr.trim().is_empty() {
                english::Noun::count(reference_unit().name, amount as u32)
            } else {
                abbr
            };
            ingredient.write().quantity.text = format!("{amount} {abbr}",);
        } else {
            ingredient.write().quantity.text = reference_unit().abbreviation;
        }
    });

    rsx! {
        Card {
            CardHeader {
                div { class: "flex flex-row items-center gap-4",
                    CardTitle {
                        div { class: "flex flex-col gap-4",
                            Label { html_for: "ingredient", "Name" }
                            SearchingSelect::<types::Ingredient> {
                                placeholder: "{ingredient.read().reference_ingredient.name}",
                                typeahead_buffer: ingredient_search_input,
                                on_value_change: move |v| {
                                    if let Some(v) = v {
                                        ingredient.write().reference_ingredient = v;
                                    }
                                },

                                SelectTrigger { SelectValue {} }

                                SelectList {
                                    for (idx , ingredient) in matching_ingredients().iter().enumerate() {
                                        SelectOption::<types::Ingredient> {
                                            index: idx,
                                            value: ingredient.clone(),
                                            text_value: ingredient.name.clone(),

                                            "{ingredient.name}"

                                            SelectItemIndicator {}
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "flex flex-col gap-4",
                        Label { html_for: "description", "Description" }
                        Input {
                            value: ingredient.read().source_text.clone().unwrap_or_else(String::new),
                            oninput: move |e: FormEvent| {
                                ingredient.write().source_text = Some(e.value());
                            },
                        }
                    }
                }
            }

            CardContent { class: "flex flex-col gap-4 justify-center",

                Label { html_for: "quantity", "Quantity ({ingredient.read().quantity.text.clone()})" }
                div { class: "flex flex-col sm:flex-row sm:items-center gap-4",

                    SearchingSelect::<types::IngredientAllowedUnit> {
                        name: "quantity",
                        placeholder: "{ingredient.read().quantity.reference_unit.name}",
                        typeahead_buffer: unit_search_input,
                        on_value_change: move |v: Option<types::IngredientAllowedUnit>| {
                            if let Some(v) = v {
                                ingredient.write().quantity.reference_unit = v.as_reference_unit();
                            }
                        },

                        SelectTrigger { SelectValue {} }

                        SelectList {
                            for (idx , unit) in matching_units().iter().enumerate() {
                                SelectOption::<types::IngredientAllowedUnit> {
                                    index: idx,
                                    value: unit.clone(),
                                    text_value: unit.name.clone(),

                                    if let Some(abbreviation) = unit.abbreviation.clone() {
                                        "{unit.name} ({abbreviation})"
                                    } else {
                                        "{unit.name}"
                                    }

                                    SelectItemIndicator {}
                                }
                            }
                        }
                    }

                    Input {
                        r#type: "number",
                        min: 0,
                        value: ingredient.read().quantity.amount,
                        oninput: move |e: FormEvent| {
                            if let Ok(t) = e.value().parse::<f64>() {
                                ingredient.write().quantity.amount = Some(t);
                            }
                        },
                    }

                    span { "{ingredient.read().quantity.reference_unit.abbreviation}" }
                }

                Label { html_for: "preparations", "Preparations" }
                div { class: "flex flex-col justify-start gap-4",

                    for idx in 0..ingredient().reference_preparations.len() {
                        div { class: "flex flex-row gap-4",

                            Button {
                                variant: ButtonVariant::Outline,

                                onclick: move |_| {
                                    ingredient.write().reference_preparations.remove(idx);
                                },

                                "X"
                            }

                            PreparationSelector {
                                preparation: ingredient
                                    .map_mut(
                                        move |r| &r.reference_preparations[idx],
                                        move |r| &mut r.reference_preparations[idx],
                                    ),
                                preparations_matcher,
                            }
                        }
                    }

                    Button {
                        class: "sm:max-w-1/2",
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| {
                            ingredient
                                .with_mut(|i| {
                                    let prep = preparations_matcher().corpus.first().unwrap().1.clone();
                                    i.reference_preparations.push(prep);
                                })
                        },

                        "Add preparation"
                    }
                }
            }
        }
    }
}

#[component]
fn KeepWarmSettingSelector(setting: WriteSignal<types::CapabilitySetting>) -> Element {
    let checked = use_memo(move || match setting().value {
        types::SettingValue::Boolean { value, .. } => {
            if value {
                CheckboxState::Checked
            } else {
                CheckboxState::Unchecked
            }
        }
        _ => CheckboxState::Indeterminate,
    });

    rsx! {
        div { class: "flex flex-row gap-4",

            span { "Keep warm?" }

            Checkbox {
                checked: Some(checked()),
                on_checked_change: move |v: CheckboxState| {
                    let checked: bool = v.into();

                    setting.write().value = types::SettingValue::Boolean {
                        text: "placeholder".to_owned(),
                        value: checked,
                    };
                },
            }
        }
    }
}

#[component]
fn SettingsSelector(setting: WriteSignal<types::CapabilitySetting>) -> Element {
    let type_ = setting().reference_setting.id;
    let type_str = use_memo(move || Some(setting().reference_setting.id.to_string()));

    rsx! {
        Tabs {
            value: type_str,
            on_value_change: move |v: String| {
                setting.write().reference_setting = types::ReferenceSettingId::from_str(&v)
                    .unwrap()
                    .reference_setting();
            },

            TabList {
                TabTrigger {
                    value: types::ReferenceSettingId::KeepWarm.to_string(),
                    index: 0usize,
                    "Keep warm"
                }
                TabTrigger {
                    value: types::ReferenceSettingId::Temperature.to_string(),
                    index: 0usize,
                    "Temperature"
                }
                TabTrigger {
                    value: types::ReferenceSettingId::Speed.to_string(),
                    index: 0usize,
                    "Speed"
                }
                TabTrigger {
                    value: types::ReferenceSettingId::Time.to_string(),
                    index: 0usize,
                    "Time"
                }
            }
        }


    }
}

#[component]
fn StepCapability(capability: WriteSignal<types::StepCapability>) -> Element {
    rsx! {
        Card {
            CardContent { class: "flex flex-col gap-4 justify-center",

                Label { html_for: "phase", "Phase" }
                select::Select::<types::CapabilityPhase> {
                    placeholder: capability().phase.name.clone(),
                    on_value_change: move |v| {
                        if let Some(v) = v {
                            capability.write().phase = v;
                        }
                    },

                    select::SelectTrigger { select::SelectValue {} }

                    select::SelectList {
                        for (idx , phase) in types::CapabilityPhase::known_options().into_iter().enumerate() {
                            select::SelectOption::<types::CapabilityPhase> {
                                index: idx,
                                value: phase.clone(),
                                text_value: phase.name.clone(),

                                "{phase.name}"
                            }
                        }
                    }
                }


                Label { html_for: "capability", "Capability" }
                select::Select::<types::ReferenceCapability> {
                    placeholder: capability().reference_capability.name.clone(),
                    on_value_change: move |v| {
                        if let Some(v) = v {
                            capability.write().reference_capability = v;
                        }
                    },

                    select::SelectTrigger { select::SelectValue {} }

                    select::SelectList {
                        for (idx , capability) in types::ReferenceCapability::known_options().into_iter().enumerate() {
                            select::SelectOption::<types::ReferenceCapability> {
                                index: idx,
                                value: capability.clone(),
                                text_value: capability.name.clone(),

                                "{capability.name}"
                            }
                        }
                    }
                }

                Label { html_for: "settings", "Settings" }
                div { class: "flex flex-col justify-start gap-4",

                    for idx in 0..capability().settings.len() {
                        div { class: "flex flex-row gap-4",

                            Button {
                                variant: ButtonVariant::Outline,

                                onclick: move |_| {
                                    capability.write().settings.remove(idx);
                                },

                                "X"
                            }

                            SettingsSelector { setting: capability.map_mut(move |r| &r.settings[idx], move |r| &mut r.settings[idx]) }
                        }
                    }

                    Button {
                        class: "sm:max-w-1/2",
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| {
                            capability
                                .with_mut(|i| {
                                    let setting = types::CapabilitySetting::keep_warm_default();
                                    i.settings.push(setting);
                                });
                        },

                        "Add preparation"
                    }
                }
            }
        }
    }
}

// #[component]
// fn Step(
//     step: WriteSignal<types::RecipeStep>,
//     ingredients_matcher: Memo<FuzzyFinder<types::Ingredient>>,
//     preparations_matcher: Memo<FuzzyFinder<types::ReferencePreparation>>,
// ) -> Element {
//     let ingredient_search_input = use_signal(|| String::new());
//     let unit_search_input = use_signal(|| String::new());

//     let matching_ingredients = use_memo(move || {
//         ingredients_matcher
//             .write()
//             .search(ingredient_search_input().as_str(), 10)
//             .into_iter()
//             .cloned()
//             .collect_vec()
//     });

//     let allowed_units = use_loader(move || {
//         allowed_units_server(ingredient.read().reference_ingredient.id.clone())
//     })?;
//     let mut units_matcher = use_memo(move || {
//         FuzzyFinder::new(allowed_units.iter().map(|i| (i.name.clone(), i.clone())))
//     });

//     let matching_units = use_memo(move || {
//         units_matcher
//             .write()
//             .search(unit_search_input().as_str(), 10)
//             .into_iter()
//             .cloned()
//             .collect_vec()
//     });

//     let amount = use_memo(move || ingredient().quantity.amount);
//     let reference_unit = use_memo(move || ingredient().quantity.reference_unit);

//     use_effect(move || {
//         if let Some(amount) = amount() {
//             let abbr = reference_unit().abbreviation;
//             let abbr = if abbr.trim().is_empty() {
//                 english::Noun::count(reference_unit().name, amount as u32)
//             } else {
//                 abbr
//             };
//             ingredient.write().quantity.text = format!("{amount} {abbr}",);
//         } else {
//             ingredient.write().quantity.text = reference_unit().abbreviation;
//         }
//     });

//     rsx! {
//         Card {
//             CardHeader {
//                 div {
//                     class: "flex flex-row items-center gap-4",
//                     CardTitle {
//                         div {
//                             class: "flex flex-col gap-4",
//                         Label { html_for: "ingredient", "Name" }
//                         SearchingSelect<types::Ingredient> {
//                             placeholder: "{ingredient.read().reference_ingredient.name}",
//                             typeahead_buffer: ingredient_search_input,
//                             on_value_change: move |v| {
//                                 if let Some(v) = v {
//                                     ingredient.write().reference_ingredient = v;
//                                 }
//                             },

//                             SelectTrigger {
//                                 SelectValue {}
//                             }

//                             SelectList {
//                                 for (idx, ingredient) in matching_ingredients().iter().enumerate() {
//                                     SelectOption::<types::Ingredient> {
//                                         index: idx,
//                                         value: ingredient.clone(),
//                                         text_value: ingredient.name.clone(),

//                                         "{ingredient.name}"

//                                             SelectItemIndicator { }
//                                     }
//                                 }
//                             }
//                         }
//                         }
//                     }

//                     div {
//                         class: "flex flex-col gap-4",
//                         Label { html_for: "description", "Description" }
//                         Input {
//                             value: ingredient.read().source_text.clone().unwrap_or_else(String::new),
//                             oninput: move |e: FormEvent| {
//                                 ingredient.write().source_text = Some(e.value());
//                             }
//                         }
//                     }
//                 }
//             }

//             CardContent {
//                 class: "flex flex-col gap-4 justify-center",

//                 Label { html_for: "capability", "Capability" }
//                 div {
//                     class: "flex flex-col sm:flex-row sm:items-center gap-4",

//                     SearchingSelect<Option<types::StepCapability>> {
//                         name: "capability",
//                         placeholder: step().capability.map_or_else(),
//                         typeahead_buffer: unit_search_input,
//                         on_value_change: move |v: Option<types::IngredientAllowedUnit>| {
//                             if let Some(v) = v {
//                                 ingredient.write().quantity.reference_unit = v.as_reference_unit();
//                             }
//                         },

//                         SelectTrigger {
//                             SelectValue {}
//                         }

//                         SelectList {
//                             for (idx, unit) in matching_units().iter().enumerate() {
//                                 SelectOption::<types::IngredientAllowedUnit> {
//                                     index: idx,
//                                     value: unit.clone(),
//                                     text_value: unit.name.clone(),

//                                     if let Some(abbreviation) = unit.abbreviation.clone() {
//                                         "{unit.name} ({abbreviation})"
//                                     } else {
//                                         "{unit.name}"
//                                     }

//                                     SelectItemIndicator { }
//                                 }
//                             }
//                         }
//                     }

//                     Input {
//                         r#type: "number",
//                         min: 0,
//                         value: ingredient.read().quantity.amount,
//                         oninput: move |e: FormEvent| {
//                             if let Ok(t) = e.value().parse::<f64>() {
//                                 ingredient.write().quantity.amount = Some(t);
//                             }
//                         }
//                     }

//                     span {
//                         "{ingredient.read().quantity.reference_unit.abbreviation}"
//                     }
//                 }

//                 Label { html_for: "preparations", "Preparations" }
//                 div {
//                     class: "flex flex-col justify-start gap-4",

//                     for reference_preparation in ingredient.map_mut(
//                         move |r| &r.reference_preparations,
//                         move |r| &mut r.reference_preparations
//                     ).iter_mut() {
//                             div {
//                                 class: "flex flex-row gap-4",

//                                 Button {
//                                     variant: ButtonVariant::Outline,

//                                     onclick: move |_| {
//                                         ingredient.write().reference_preparations.as_mut().unwrap().remove(idx);
//                                     },

//                                     "X"
//                                 }

//                                 PreparationSelector {
//                                     preparation: ingredient.map_mut(
//                                         move |r| &r.reference_preparations.as_ref().unwrap()[idx],
//                                         move |r| &mut r.reference_preparations.as_mut().unwrap()[idx],
//                                     ),
//                                     preparations_matcher,
//                                 }
//                             }
//                         }

//                     Button {
//                         class: "sm:max-w-1/2",
//                         variant: ButtonVariant::Secondary,
//                         onclick: move |_| {
//                             ingredient.with_mut(|i| {
//                                 let prep = preparations_matcher().corpus.first().unwrap().1.clone();
//                                 i.reference_preparations.push(prpe)
//                                 if let Some(preps) = .as_mut() {
//                                     preps.push(prep);
//                                 } else {
//                                     i.reference_preparations = Some(vec![prep]);
//                                 }
//                             })
//                         },

//                         "Add preparation"
//                     }
//                 }
//             }
//         }
//     }
// }

#[derive(Clone)]
struct FuzzyFinder<T> {
    matcher: nucleo_matcher::Matcher,
    corpus: Vec<(nucleo_matcher::Utf32String, T, u32)>,
    needle: nucleo_matcher::pattern::Pattern,
}

impl<T: PartialEq> PartialEq for FuzzyFinder<T> {
    fn eq(&self, other: &Self) -> bool {
        self.corpus == other.corpus
    }
}

impl<T> FuzzyFinder<T> {
    pub fn new<'a>(corpus: impl IntoIterator<Item = (String, T)>) -> Self {
        let matcher = nucleo_matcher::Matcher::new(nucleo_matcher::Config::DEFAULT);
        let corpus = corpus
            .into_iter()
            .map(|(x, y)| (nucleo_matcher::Utf32String::from(x), y, 0))
            .collect();
        let needle = nucleo_matcher::pattern::Pattern::default();

        Self {
            matcher,
            corpus,
            needle,
        }
    }

    fn search(&mut self, needle: &str, n: usize) -> Vec<&T> {
        self.needle.reparse(
            needle,
            nucleo_matcher::pattern::CaseMatching::Ignore,
            nucleo_matcher::pattern::Normalization::Smart,
        );

        for (haystack, _, score) in &mut self.corpus {
            *score = self
                .needle
                .score(haystack.slice(..), &mut self.matcher)
                .unwrap_or(0);
        }

        self.corpus
            .iter()
            .k_largest_relaxed_by_key(n, |x| x.2)
            .map(|x| &x.1)
            .collect()
    }
}

#[component]
pub fn EditRecipe(id: String) -> Element {
    let recipe_initial = use_loader(move || recipe_server(id.clone()))?.cloned();
    let mut recipe = use_signal(move || recipe_initial);

    let ingredients = use_loader(ingredients_server)?;
    let ingredients_matcher =
        use_memo(move || FuzzyFinder::new(ingredients.iter().map(|i| (i.name.clone(), i.clone()))));
    let preparations = use_loader(preparations_server)?;
    let preparations_matcher = use_memo(move || {
        FuzzyFinder::new(preparations.iter().map(|i| (i.name.clone(), i.clone())))
    });

    rsx! {

        div { class: "flex justify-center",

            div { class: "flex flex-col w-3/4 gap-4 justify-center",

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
                div { class: "flex flex-row items-center gap-4",

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
                        },
                    }
                    span { "Seconds" }
                }

                Label { html_for: "recipe_cook_time", "Cook time" }
                div { class: "flex flex-row items-center gap-4",

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
                        },
                    }
                    span { "Seconds" }
                }

                Label { html_for: "recipe_serves", "Serves" }
                div { class: "flex flex-row items-center gap-4",

                    Input {
                        id: "recipe_serves",
                        r#type: "number",
                        min: 0,
                        value: recipe.read().serves,
                        oninput: move |e: FormEvent| {
                            if let Ok(x) = e.value().parse::<u8>() {
                                recipe.write().serves = x;
                            }
                        },
                    }
                    span { "People" }
                }

                Tabs { default_value: "ingredients",

                    TabList {
                        TabTrigger { value: "ingredients", index: 0usize, "Ingredients" }
                        TabTrigger { value: "steps", index: 1usize, "Steps" }
                    }

                    TabContent { index: 0usize, value: "ingredients",
                        div { class: "flex flex-col gap-4 p-4",

                            for idx in 0..recipe.read().ingredients.len() {
                                Ingredient {
                                    ingredient: recipe.map_mut(move |r| &r.ingredients[idx], move |r| &mut r.ingredients[idx]),
                                    ingredients_matcher,
                                    preparations_matcher,
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
#[tracing::instrument]
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
async fn preparations_server() -> Result<Vec<types::ReferencePreparation>> {
    use dioxus::{
        logger::tracing::{info_span, Instrument as _},
        CapturedError,
    };

    let preparations = db::queries::preparations::list_preparations(crate::db::db(), None, None)
        .instrument(info_span!("Loading preparations"))
        .await
        .map_err(|e| CapturedError::from_boxed(e.into()))?;

    Ok(preparations)
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
