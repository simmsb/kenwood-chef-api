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
use itertools::Itertools;
use types::traits::*;
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
fn Quantity(
    quantity: Store<types::Quantity>,
    allowed_units: Memo<Vec<types::IngredientAllowedUnit>>,
) -> Element {
    let unit_search_input = use_signal(|| String::new());
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

    let reference_unit = quantity.reference_unit();

    use_effect(move || {
        if let Some(amount) = *quantity.amount().read() {
            let abbr = reference_unit.abbreviation().read().clone();
            let abbr = if abbr.trim().is_empty() {
                english::Noun::count(reference_unit().name, amount as u32)
            } else {
                abbr
            };
            quantity.write().text = format!("{amount} {abbr}",);
        } else {
            quantity.write().text = reference_unit().abbreviation;
        }
    });

    rsx! {
        Label { html_for: "quantity", "Quantity ({quantity().text})" }
        div { class: "flex flex-col sm:flex-row sm:items-center gap-4",

              SearchingSelect::<types::IngredientAllowedUnit> {
                  name: "quantity",
                  placeholder: "{quantity().reference_unit.name}",
                  typeahead_buffer: unit_search_input,
                  on_value_change: move |v: Option<types::IngredientAllowedUnit>| {
                      if let Some(v) = v {
                          quantity.write().reference_unit = v.as_reference_unit();
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
                  value: quantity().amount,
                  oninput: move |e: FormEvent| {
                      if let Ok(t) = e.value().parse::<f64>() {
                          quantity.write().amount = Some(t);
                      }
                  },
              }

              span { "{quantity().reference_unit.abbreviation}" }
        }
    }
}

#[component]
fn Ingredient(
    ingredient: Store<types::RecipeIngredient>,
    ingredients_matcher: Memo<FuzzyFinder<types::Ingredient>>,
    preparations_matcher: Memo<FuzzyFinder<types::ReferencePreparation>>,
) -> Element {
    let ingredient_search_input = use_signal(|| String::new());

    let matching_ingredients = use_memo(move || {
        ingredients_matcher
            .write()
            .search(ingredient_search_input().as_str(), 10)
            .into_iter()
            .cloned()
            .collect_vec()
    });

    let reference_unit_id = ingredient.reference_ingredient().id();
    let allowed_units = use_loader(move || allowed_units_server(reference_unit_id()))?;
    let allowed_units = use_memo(move || allowed_units());

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
                                        ingredient.reference_ingredient().set(v);
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
                            value: ingredient.source_text().read().clone().unwrap_or_else(String::new),
                            oninput: move |e: FormEvent| {
                                ingredient.source_text().set(Some(e.value()));
                            },
                        }
                    }
                }
            }

            CardContent { class: "flex flex-col gap-4 justify-center",

                Quantity {
                    quantity: ingredient.quantity(),
                    allowed_units
                }

                Label { html_for: "preparations", "Preparations" }
                div { class: "flex flex-col justify-start gap-4",

                    for (idx, preparation) in ingredient.reference_preparations().iter().enumerate() {
                        div { class: "flex flex-row gap-4",

                            Button {
                                variant: ButtonVariant::Outline,

                                onclick: move |_| {
                                    ingredient.reference_preparations().remove(idx);
                                },

                                "X"
                            }

                            PreparationSelector {
                                preparation,
                                preparations_matcher,
                            }
                        }
                    }

                    Button {
                        class: "sm:max-w-1/2",
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| {
                            ingredient
                                .reference_preparations().push(preparations_matcher().corpus.first().unwrap().1.clone());
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
fn TemperatureSettingSelector(setting: WriteSignal<types::CapabilitySetting>) -> Element {
    let value = use_memo(move || match setting().value {
        types::SettingValue::Numeric { value, .. } => Some(value),
        types::SettingValue::Nominal { .. } => Some(0.0),
        _ => None,
    });

    rsx! {
        div { class: "flex flex-row gap-4",

              Label { html_for: "temperature", "Temperature" }
              div { class: "flex flex-row items-center gap-4",

                    Input {
                        id: "recipe_prep_time",
                        r#type: "number",
                        min: 0,
                        max: 200,
                        value: value().map(|x| x.to_string()).unwrap_or_else(|| "Enter temperature".to_owned()),
                        oninput: move |e: FormEvent| {
                            if let Ok(t) = e.value().parse::<f64>() {
                                setting.write().value = if t < 0.01 {
                                    types::SettingValue::Nominal { text: "No temperature".to_owned(), reference_value: types::ReferenceValue::temperature_off() }
                                } else {
                                    types::SettingValue::Numeric {
                                        reference_unit: Some(types::ReferenceUnit::celcius()),
                                        text: format!("{t:.2} °C"),
                                        value: t
                                    }
                                }
                            }
                        },
                    }
                    span { "Seconds" }
              }

        }
    }
}

#[component]
fn SpeedSettingSelector(setting: WriteSignal<types::CapabilitySetting>) -> Element {
    let value = use_memo(move || match setting().value {
        types::SettingValue::Nominal {
            reference_value, ..
        } => Some(reference_value),
        _ => None,
    });

    rsx! {
        div { class: "flex flex-row gap-4",

              Label { html_for: "speed", "Speed" }
                select::Select::<types::ReferenceValue> {
                    value: Some(value()),
                    on_value_change: move |v: Option<types::ReferenceValue>| {
                        if let Some(v) = v {
                            setting.write().value = types::SettingValue::Nominal {
                                text: v.name.clone(),
                                reference_value: v
                            };
                        }
                    },

                    select::SelectTrigger { select::SelectValue {} }

                    select::SelectList {
                        for (idx, value) in types::ReferenceValue::stir_settings().into_iter().enumerate() {
                            select::SelectOption::<types::ReferenceValue> {
                                index: idx,
                                value: value.clone(),
                                text_value: value.name.clone(),

                                "{value.name}"
                            }
                        }
                    }
                }

        }
    }
}

#[component]
fn TimeSettingSelector(setting: WriteSignal<types::CapabilitySetting>) -> Element {
    let value = use_memo(move || match setting().value {
        types::SettingValue::Numeric { value, .. } => {
            jiff::Span::try_from(jiff::SignedDuration::from_secs_f64(value))
                .ok()
                .map(|x| x.fieldwise())
        }
        _ => None,
    });

    let unwrapped_value =
        use_memo(move || value().unwrap_or_else(|| jiff::Span::new().fieldwise()));

    let set = use_callback(move |span: jiff::Span| {
        setting.write().value = types::SettingValue::Numeric {
            text: format!("{span:#}"),
            value: span.total(jiff::Unit::Second).unwrap_or(0.0),
            reference_unit: None,
        }
    });

    rsx! {
        div { class: "flex flex-row gap-4",

              Label { html_for: "time", "Time" }
              div {
                  class: "flex flex-row gap-4 items-center",

                  Input {
                      oninput: move |e: FormEvent| {
                          if let Ok(t) = e.value().parse::<i64>() {
                              set(unwrapped_value().0.hours(t));
                          }
                      }
                  }
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

        match type_ {
            types::ReferenceSettingId::KeepWarm =>
                rsx! { KeepWarmSettingSelector { setting } },
            types::ReferenceSettingId::Temperature =>
                rsx! { TemperatureSettingSelector { setting } },
            types::ReferenceSettingId::Speed =>
                rsx! { SpeedSettingSelector { setting } },
            types::ReferenceSettingId::Time =>
                rsx! { TimeSettingSelector { setting } },
        }
    }
}

#[component]
fn StepCapability(capability: Store<types::StepCapability>) -> Element {
    rsx! {
        Card {
            CardContent { class: "flex flex-col gap-4 justify-center",

                Label { html_for: "phase", "Phase" }
                select::Select::<types::CapabilityPhase> {
                    // placeholder: capability().phase.name.clone(),
                    value: Some(Some(capability().phase)),
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
                    // placeholder: capability().reference_capability.name.clone(),
                    value: Some(Some(capability().reference_capability)),
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

#[component]
fn StepIngredient(
    ingredients: Store<Vec<types::RecipeIngredient>>,
    ingredient: Store<types::StepIngredient>,
) -> Element {
    // signal that tracks the ingredient, used to move the ingredient_idx when
    // the list changes
    let mut recipe_ingredient = use_signal(move || {
        ingredients
            .get(ingredient.ingredient_idx().read().clone() as usize)
            .map(|x| x.read().clone())
    });

    let current_ingredient = use_memo(move || {
        ingredients
            .get(ingredient.ingredient_idx().read().clone() as usize)
            .map(|x| x.read().clone())
    });

    use_effect(move || {
        let Some(recipe_ingredient_) = recipe_ingredient() else {
            return;
        };
        let Some(current_ingredient_) = current_ingredient() else {
            return;
        };

        // if the recipe positions changed, try to reconcile
        if recipe_ingredient_.reference_ingredient.id != current_ingredient_.reference_ingredient.id
        {
            if let Some((new_index, _)) = ingredients().iter().find_position(|i| {
                i.reference_ingredient.id == recipe_ingredient_.reference_ingredient.id
            }) {
                ingredient.write().ingredient_idx = new_index as u8;
            }

            recipe_ingredient.set(Some(current_ingredient_));
        }
    });

    let reference_unit_id =
        use_memo(move || current_ingredient().map(|x| x.reference_ingredient.id.clone()));
    let allowed_units = use_loader(move || {
        let ref_unit_id = reference_unit_id();
        async {
            if let Some(ref_unit_id) = ref_unit_id {
                allowed_units_server(ref_unit_id).await
            } else {
                Ok(vec![])
            }
        }
    })?;
    let allowed_units = use_memo(move || allowed_units());

    let using_custom_quantity = use_memo(move || {
        let Some(current_ingredient_) = current_ingredient() else {
            return CheckboxState::Indeterminate;
        };
        if ingredient().quantity == current_ingredient_.quantity {
            CheckboxState::Unchecked
        } else {
            CheckboxState::Checked
        }
    });

    let mut force_show_quantity = use_signal(|| false);
    let show_quantity = use_memo(move || force_show_quantity() || using_custom_quantity().into());

    rsx! {
        Card {
            CardHeader {
                div { class: "flex flex-row items-center gap-4",
                    CardTitle {
                        div { class: "flex flex-col gap-4",
                            Label { html_for: "ingredient", "Name" }
                            select::Select::<u8> {
                                value: Some(ingredient().ingredient_idx),
                                on_value_change: move |v| {
                                    if let Some(v) = v {
                                        ingredient.write().ingredient_idx = v;
                                    }
                                },

                                select::SelectTrigger { select::SelectValue {} }

                                select::SelectList {
                                    for (idx, ingredient) in ingredients().iter().enumerate() {
                                        select::SelectOption::<u8> {
                                            index: idx,
                                            value: idx as u8,
                                            text_value: ingredient.reference_ingredient.name.clone(),

                                            "{ingredient.reference_ingredient.name}"

                                            SelectItemIndicator {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            CardContent {
                class: "flex flex-col gap-4 justify-center",

                Label { html_for: "use_custom_quantity", "Use custom quantity" },
                Checkbox {
                    name: "use_custom_quantity",
                    default_checked: using_custom_quantity(),

                    on_checked_change: move |e: CheckboxState| {
                        let checked: bool = e.into();

                        if checked {
                            force_show_quantity.set(true);
                        } else {
                            force_show_quantity.set(false);
                            if let Some(current_ingredient_) = current_ingredient() {
                                ingredient.write().quantity = current_ingredient_.quantity;
                            }
                        }
                    }
                }

                if show_quantity() {
                    Quantity {
                        quantity: ingredient.quantity(),
                        allowed_units
                    }
                }
            }
        }
    }
}

#[component]
fn Step(
    idx: usize,
    step: Store<types::RecipeStep>,
    ingredients: Store<Vec<types::RecipeIngredient>>,
    // ingredients_matcher: Memo<FuzzyFinder<types::Ingredient>>,
    // preparations_matcher: Memo<FuzzyFinder<types::ReferencePreparation>>,
) -> Element {
    // let ingredient_search_input = use_signal(|| String::new());
    // let unit_search_input = use_signal(|| String::new());
    rsx! {
        Card {
            CardHeader {
                "Step {idx}"
            }

            CardContent {
                class: "flex flex-col gap-4 justify-center",

                Label { html_for: "capability", "Capability" }
                if let Some(capability) = step.capability().transpose() {
                    StepCapability {
                        capability
                    }

                    Button {
                        class: "sm:max-w-1/2",
                        variant: ButtonVariant::Destructive,
                        onclick: move |_| {
                            step.capability().set(None)
                        },

                        "Remove capability"
                    }
                } else {
                    Button {
                        class: "sm:max-w-1/2",
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| {
                            step.capability().set(Some(types::StepCapability {
                                phase: types::CapabilityPhase::setup(),
                                reference_capability: types::ReferenceCapability::bake(),
                                settings: vec![]
                            }))
                        },

                        "Add capability"
                    }
                }

                Label { html_for: "preparations", "Preparations" }
                div {
                    class: "flex flex-col justify-start gap-4",

                    for ingredient in step.ingredients().iter() {
                            div {
                                class: "flex flex-row gap-4",

                                Button {
                                    variant: ButtonVariant::Outline,

                                    onclick: move |_| {
                                        step.ingredients().remove(idx);
                                    },

                                    "X"
                                }

                                StepIngredient {
                                    ingredient,
                                    ingredients
                                }
                            }
                        }

                    Button {
                        class: "sm:max-w-1/2",
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| {
                            if let Some(first_ingredient) = ingredients.first() {
                                step.ingredients().push(types::StepIngredient {
                                    ingredient_idx: 0,
                                    quantity: first_ingredient.quantity.clone(),
                                });
                            }
                        },

                        "Add ingredient"
                    }
                }
            }
        }
    }
}

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
    let recipe = use_store(move || recipe_initial);

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
                    value: recipe.name().clone(),
                    oninput: move |e: FormEvent| recipe.name().set(e.value()),
                }

                Label { html_for: "recipe_description", "Description" }
                Textarea {
                    id: "recipe_description",
                    placeholder: recipe.description().clone(),
                    value: recipe.description().clone(),
                    oninput: move |e: FormEvent| recipe.description().set(e.value()),
                }

                Label { html_for: "recipe_prep_time", "Prep time" }
                div { class: "flex flex-row items-center gap-4",

                    Input {
                        id: "recipe_prep_time",
                        r#type: "number",
                        min: 0,
                        value: recipe
                            .prep_time()
                            .read().clone()
                            .map(|x| x.as_mins())
                            .map(|x| x.to_string())
                            .unwrap_or(String::new()),
                        oninput: move |e: FormEvent| {
                            if let Ok(t) = e.value().parse::<i64>() {
                                recipe.prep_time().set(Some(jiff::SignedDuration::from_mins(t)));
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
                            .cook_time()
                            .read().clone()
                            .map(|x| x.as_mins())
                            .map(|x| x.to_string())
                            .unwrap_or(String::new()),
                        oninput: move |e: FormEvent| {
                            if let Ok(t) = e.value().parse::<i64>() {
                                recipe.cook_time().set(Some(jiff::SignedDuration::from_mins(t)));
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
                        value: recipe.serves(),
                        oninput: move |e: FormEvent| {
                            if let Ok(x) = e.value().parse::<u8>() {
                                recipe.serves().set(x);
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

                            for ingredient in recipe.ingredients().iter() {
                                Ingredient {
                                    ingredient,
                                    ingredients_matcher,
                                    preparations_matcher,
                                }
                            }
                        }
                    }

                    TabContent { index: 1usize, value: "steps",
                        div { class: "flex flex-col gap-4 p-4",
                            for (idx, step) in recipe.steps().iter().enumerate() {
                                Step {
                                    idx,
                                    step,
                                    ingredients: recipe.ingredients()
                                }
                            }
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
