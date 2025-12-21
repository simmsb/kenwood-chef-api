use core::str::FromStr;
use rand::distr::SampleString;
use std::string::ToString;

use dioxus::{
    fullstack::{Form, MultipartFormData},
    prelude::*,
};
use dioxus_primitives::checkbox::CheckboxState;
use itertools::Itertools;
use types::traits::*;
use types::KnownOptions;

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
    trace!("Render quantity");
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

    let update_text = use_callback(move |()| {
        let text = if let Some(amount) = *quantity.amount().read() {
            let abbr = quantity.reference_unit().abbreviation().cloned();
            let abbr = if abbr.trim().is_empty() {
                english::Noun::count(quantity.reference_unit().name().cloned(), amount as u32)
            } else {
                abbr
            };
            format!("{amount} {abbr}",)
        } else {
            quantity.reference_unit().abbreviation().cloned()
        };

        quantity.text().set(text);
    });

    rsx! {
        Label { html_for: "quantity", "Quantity ({quantity.text()})" }
        div { class: "flex flex-col sm:flex-row sm:items-center gap-4",

            SearchingSelect::<types::IngredientAllowedUnit> {
                name: "quantity",
                placeholder: "{quantity.reference_unit().name()}",
                typeahead_buffer: unit_search_input,
                on_value_change: move |v: Option<types::IngredientAllowedUnit>| {
                    if let Some(v) = v {
                        quantity.reference_unit().set(v.as_reference_unit());
                        update_text.call(());
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
                value: quantity.amount(),
                oninput: move |e: FormEvent| {
                    if let Ok(t) = e.value().parse::<f64>() {
                        quantity.amount().set(Some(t));
                        update_text.call(());
                    }
                },
            }

            span { "{quantity.reference_unit().abbreviation()}" }
        }
    }
}

#[component]
fn Ingredient(
    ingredient: Store<types::RecipeIngredient>,
    ingredients_matcher: Memo<FuzzyFinder<types::Ingredient>>,
    preparations_matcher: Memo<FuzzyFinder<types::ReferencePreparation>>,
) -> Element {
    trace!("Render ingredient");
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
        Card { class: "grow",

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
                            value: ingredient.source_text().cloned().unwrap_or_else(String::new),
                            oninput: move |e: FormEvent| {
                                ingredient.source_text().set(Some(e.value()));
                            },
                        }
                    }
                }
            }

            CardContent { class: "flex flex-col gap-4 justify-center",

                Quantity { quantity: ingredient.quantity(), allowed_units }

                Label { html_for: "preparations", "Preparations" }
                div { class: "flex flex-col justify-start gap-4",

                    for (idx , preparation) in ingredient.reference_preparations().iter().enumerate() {
                        div { class: "flex flex-row gap-4",
                            PreparationSelector { preparation, preparations_matcher }

                            Button {
                                class: "place-self-end",
                                variant: ButtonVariant::Outline,

                                onclick: move |_| {
                                    ingredient.reference_preparations().remove(idx);
                                },

                                "X"
                            }
                        
                        }
                    }

                    Button {
                        class: "sm:max-w-1/2",
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| {
                            ingredient
                                .reference_preparations()
                                .push(preparations_matcher().corpus.first().unwrap().1.clone());
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
        Label { html_for: "keep_warm", "Keep warm?" }

        Checkbox {
            name: "keep_warm",
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

#[component]
fn TemperatureSettingSelector(setting: WriteSignal<types::CapabilitySetting>) -> Element {
    let value = use_memo(move || match setting().value {
        types::SettingValue::Numeric { value, .. } => Some(value),
        types::SettingValue::Nominal { .. } => Some(0.0),
        _ => None,
    });

    rsx! {
        div { class: "flex w-full flex-row gap-4",

            Label { html_for: "temperature", "Temperature" }
            div { class: "flex w-full flex-row items-center gap-4",

                Input {
                    id: "recipe_prep_time",
                    r#type: "number",
                    min: 0,
                    max: 200,
                    value: value().map(|x| x.to_string()).unwrap_or_else(|| "Enter temperature".to_owned()),
                    oninput: move |e: FormEvent| {
                        if let Ok(t) = e.value().parse::<f64>() {
                            setting.write().value = if t < 0.01 {
                                types::SettingValue::Nominal {
                                    text: "No temperature".to_owned(),
                                    reference_value: types::ReferenceValue::temperature_off(),
                                }
                            } else {
                                types::SettingValue::Numeric {
                                    reference_unit: Some(types::ReferenceUnit::celcius()),
                                    text: format!("{t:.2} °C"),
                                    value: t,
                                }
                            };
                        }
                    },
                }
                span { "°C" }
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
        div { class: "flex w-full flex-row gap-4",

            Label { html_for: "speed", "Speed" }
            select::Select::<types::ReferenceValue> {
                value: Some(value()),
                on_value_change: move |v: Option<types::ReferenceValue>| {
                    if let Some(v) = v {
                        setting.write().value = types::SettingValue::Nominal {
                            text: v.name.clone(),
                            reference_value: v,
                        };
                    }
                },

                select::SelectTrigger { select::SelectValue {} }

                select::SelectList {
                    for (idx , value) in types::ReferenceValue::stir_settings().into_iter().enumerate() {
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
                .and_then(|x| {
                    x.round(jiff::SpanRound::new().largest(jiff::Unit::Hour))
                        .ok()
                })
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
        div { class: "flex w-full flex-row gap-4",

            Label { html_for: "time",
                "Time ({unwrapped_value().0.total(jiff::Unit::Second).unwrap_or(0.0)} s)"
            }
            div { class: "flex w-full flex-row gap-4 items-center",

                Input {
                    class: "w-24",
                    value: unwrapped_value().0.get_hours(),
                    oninput: move |e: FormEvent| {
                        if let Ok(t) = e.value().parse::<i64>() {
                            set(unwrapped_value().0.hours(t));
                        }
                    },
                }

                span { "h" }

                Input {
                    class: "w-24",
                    value: unwrapped_value().0.get_minutes(),
                    oninput: move |e: FormEvent| {
                        if let Ok(t) = e.value().parse::<i64>() {
                            set(unwrapped_value().0.minutes(t));
                        }
                    },
                }

                span { "m" }

                Input {
                    class: "w-24",
                    value: unwrapped_value().0.get_seconds(),
                    oninput: move |e: FormEvent| {
                        if let Ok(t) = e.value().parse::<i64>() {
                            set(unwrapped_value().0.seconds(t));
                        }
                    },
                }

                span { "s" }
            }
        }
    }
}

#[component]
fn SettingsSelector(setting: WriteSignal<types::CapabilitySetting>) -> Element {
    trace!("Render step selector");
    let type_ = setting().reference_setting.id;
    let type_str = use_memo(move || Some(setting().reference_setting.id.to_string()));

    rsx! {
        div { class: "flex w-full flex-row items-center justify-start gap-4",
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
                        index: 1usize,
                        "Temperature"
                    }
                    TabTrigger {
                        value: types::ReferenceSettingId::Speed.to_string(),
                        index: 2usize,
                        "Speed"
                    }
                    TabTrigger {
                        value: types::ReferenceSettingId::Time.to_string(),
                        index: 3usize,
                        "Time"
                    }
                }
            }

            match type_ {
                types::ReferenceSettingId::KeepWarm => rsx! {
                    KeepWarmSettingSelector { setting }
                },
                types::ReferenceSettingId::Temperature => rsx! {
                    TemperatureSettingSelector { setting }
                },
                types::ReferenceSettingId::Speed => rsx! {
                    SpeedSettingSelector { setting }
                },
                types::ReferenceSettingId::Time => rsx! {
                    TimeSettingSelector { setting }
                },
            }
        }
    }
}

#[component]
fn StepCapability(capability: Store<types::StepCapability>) -> Element {
    trace!("Render step capability");
    rsx! {
        Card {
            CardContent { class: "flex flex-col gap-4 justify-center",
                div { class: "flex flex-col sm:flex-row sm:items-center gap-4",

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
                        value: Some(Some(capability.reference_capability().cloned())),
                        on_value_change: move |v| {
                            if let Some(v) = v {
                                capability.reference_capability().set(v);
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
                }

                Label { html_for: "settings", "Settings" }
                div { class: "flex flex-col justify-start gap-4",

                    for (idx , setting) in capability.settings().iter().enumerate() {
                        div { class: "flex flex-row gap-4",

                            Button {
                                variant: ButtonVariant::Outline,

                                onclick: move |_| {
                                    capability.settings().remove(idx);
                                },

                                "X"
                            }

                            SettingsSelector { setting }
                        }
                    }

                    Button {
                        class: "sm:max-w-1/2",
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| {
                            capability.settings().push(types::CapabilitySetting::keep_warm_default());
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
    trace!("Render step ingredients");
    // signal that tracks the ingredient, used to move the ingredient_idx when
    // the list changes
    let mut recipe_ingredient = use_signal(move || {
        ingredients
            .get(ingredient.ingredient_idx().cloned() as usize)
            .map(|x| x.cloned())
    });

    let current_ingredient = use_memo(move || {
        ingredients
            .get(ingredient.ingredient_idx().cloned() as usize)
            .map(|x| x.cloned())
    });

    use_effect(move || {
        let Some(recipe_ingredient_) = recipe_ingredient() else {
            return;
        };

        // if the recipe positions changed, try to reconcile

        match current_ingredient() {
            Some(current_ingredient_) => {
                if recipe_ingredient_.reference_ingredient.id
                    != current_ingredient_.reference_ingredient.id
                {
                    debug!("saved and current at index changed. saved: {recipe_ingredient_:?}, current: {recipe_ingredient_:?}");
                    if let Some((new_index, _)) = ingredients().iter().find_position(|i| {
                        i.reference_ingredient.id == recipe_ingredient_.reference_ingredient.id
                    }) {
                        debug!("Found it's now at {new_index}");
                        ingredient.write().ingredient_idx = new_index as u8;
                    }

                    recipe_ingredient.set(Some(current_ingredient_));
                }
            }
            None => {
                debug!("saved index missing. saved: {recipe_ingredient_:?}");
                debug!("Ingredients: {:#?}", ingredients());
                if let Some((new_index, _)) = ingredients().iter().find_position(|i| {
                    i.reference_ingredient.id == recipe_ingredient_.reference_ingredient.id
                }) {
                    debug!("Found it's now at {new_index}");
                    ingredient.write().ingredient_idx = new_index as u8;
                } else {
                    // couldn't find :S
                    debug!("Couldn't find new index, clearing");
                    recipe_ingredient.set(None);
                }
            }
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
        Card { class: "grow",

            CardHeader {
                div { class: "flex flex-row items-center gap-4",
                    CardTitle {
                        div { class: "flex flex-col gap-4",
                            Label { html_for: "ingredient", "Name" }
                            select::Select::<u8> {
                                value: Some(ingredient().ingredient_idx),
                                on_value_change: move |v| {
                                    if let Some(v) = v {
                                        ingredient.ingredient_idx().set(v);
                                        recipe_ingredient.set(current_ingredient());
                                    }
                                },

                                select::SelectTrigger { select::SelectValue {} }

                                select::SelectList {
                                    for (idx , ingredient) in ingredients().iter().enumerate() {
                                        select::SelectOption::<u8> {
                                            index: idx,
                                            value: idx as u8,
                                            text_value: ingredient.reference_ingredient.name.clone(),

                                            "{ingredient.reference_ingredient.name}"

                                            select::SelectItemIndicator {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            CardContent { class: "flex flex-col gap-4 justify-center",

                Label { html_for: "use_custom_quantity", "Use custom quantity" }
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
                    },
                }

                if show_quantity() {
                    Quantity { quantity: ingredient.quantity(), allowed_units }
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
    trace!("Render step");
    // let ingredient_search_input = use_signal(|| String::new());
    // let unit_search_input = use_signal(|| String::new());
    rsx! {
        Card { class: "grow",
            CardHeader { "Step {idx + 1}" }

            CardContent { class: "flex flex-col gap-4 justify-center",

                Label { html_for: "capability", "Capability" }
                if let Some(capability) = step.capability().transpose() {
                    StepCapability { capability }

                    Button {
                        class: "sm:max-w-1/2",
                        variant: ButtonVariant::Destructive,
                        onclick: move |_| { step.capability().set(None) },

                        "Remove capability"
                    }
                } else {
                    Button {
                        class: "sm:max-w-1/2",
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| {
                            step.capability()
                                .set(
                                    Some(types::StepCapability {
                                        phase: types::CapabilityPhase::setup(),
                                        reference_capability: types::ReferenceCapability::bake(),
                                        settings: vec![],
                                    }),
                                )
                        },

                        "Add capability"
                    }
                }

                Label { html_for: "preparations", "Preparations" }
                div { class: "flex flex-col justify-start gap-4",

                    for ingredient in step.ingredients().iter() {
                        div { class: "flex flex-row gap-4",
                            StepIngredient { ingredient, ingredients }

                            Button {
                                class: "place-self-end",
                                variant: ButtonVariant::Outline,

                                onclick: move |_| {
                                    step.ingredients().remove(idx);
                                },

                                "X"
                            }
                        }
                    }

                    Button {
                        class: "sm:max-w-1/2",
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| {
                            if let Some(first_ingredient_quantity) = ingredients
                                .first()
                                .map(|x| x.quantity.clone())
                            {
                                step.ingredients()
                                    .push(types::StepIngredient {
                                        ingredient_idx: 0,
                                        quantity: first_ingredient_quantity,
                                    });
                            }
                        },

                        "Add ingredient"
                    }
                }

                Label { html_for: "instructions", "Instructions" }
                Textarea {
                    value: step.text(),
                    oninput: move |e: FormEvent| step.text().set(e.value()),
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
pub fn EditRecipeInner(recipe: Store<types::Recipe>) -> Element {
    let ingredients = use_loader(ingredients_server)?;
    let ingredients_matcher =
        use_memo(move || FuzzyFinder::new(ingredients.iter().map(|i| (i.name.clone(), i.clone()))));
    let preparations = use_loader(preparations_server)?;
    let preparations_matcher = use_memo(move || {
        FuzzyFinder::new(preparations.iter().map(|i| (i.name.clone(), i.clone())))
    });

    trace!("Render editrecipe");

    let sync_total_time = use_callback(move |()| {
        let prep_time = recipe
            .prep_time()
            .read()
            .clone()
            .unwrap_or_else(|| jiff::SignedDuration::ZERO);
        let cook_time = recipe
            .cook_time()
            .read()
            .clone()
            .unwrap_or_else(|| jiff::SignedDuration::ZERO);
        recipe.total_time().set(prep_time + cook_time);
    });

    rsx! {

        div { class: "flex justify-center",

            div { class: "flex flex-col w-3/4 gap-4 justify-center",

                form {
                    class: "flex flex-col gap-4",

                    onsubmit: move |e: FormEvent| async move {
                        e.prevent_default();

                        upload_image(e.into()).await.unwrap();
                    },

                    input { name: "id", hidden: true, value: recipe.id() }

                    Label { html_for: "data", "Image" }
                    div { class: "flex flex-row gap-4 justify-start items-center",
                        Input {
                            r#type: "file",
                            name: "data",
                            accept: ".png,.jpg,.jpeg,.webp",
                        }
                        Input {
                            r#type: "submit",
                            name: "submit",
                            value: "Set image",
                        }
                    }
                }

                Label { html_for: "recipe_name", "Name" }
                Input {
                    id: "recipe_name",
                    value: recipe.name().clone(),
                    oninput: move |e: FormEvent| recipe.name().set(e.value()),
                }

                Label { html_for: "recipe_description", "Description" }
                Textarea {
                    id: "recipe_description",
                    value: recipe.description(),
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
                            .read()
                            .clone()
                            .map(|x| x.as_mins())
                            .map(|x| x.to_string())
                            .unwrap_or(String::new()),
                        oninput: move |e: FormEvent| {
                            if let Ok(t) = e.value().parse::<i64>() {
                                recipe.prep_time().set(Some(jiff::SignedDuration::from_mins(t)));
                                sync_total_time.call(());
                            }
                        },
                    }
                    span { "Minutes" }
                }

                Label { html_for: "recipe_cook_time", "Cook time" }
                div { class: "flex flex-row items-center gap-4",

                    Input {
                        name: "recipe_cook_time",
                        r#type: "number",
                        min: 0,
                        value: recipe
                            .cook_time()
                            .read()
                            .clone()
                            .map(|x| x.as_mins())
                            .map(|x| x.to_string())
                            .unwrap_or(String::new()),
                        oninput: move |e: FormEvent| {
                            if let Ok(t) = e.value().parse::<i64>() {
                                recipe.cook_time().set(Some(jiff::SignedDuration::from_mins(t)));
                                sync_total_time.call(());
                            }
                        },
                    }
                    span { "Minutes" }
                }

                Label { html_for: "recipe_serves", "Serves" }
                div { class: "flex flex-row items-center gap-4",

                    Input {
                        name: "recipe_serves",
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

                span { "Total time: {recipe.total_time().read().as_mins()} Minutes" }

                Tabs { default_value: "ingredients",

                    TabList {
                        TabTrigger { value: "ingredients", index: 0usize, "Ingredients" }
                        TabTrigger { value: "steps", index: 1usize, "Steps" }
                    }

                    TabContent { index: 0usize, value: "ingredients",
                        div { class: "flex flex-col gap-4 p-4",

                            for (idx , ingredient) in recipe.ingredients().iter().enumerate() {
                                div { class: "flex flex-row gap-4",
                                    Ingredient {
                                        ingredient,
                                        ingredients_matcher,
                                        preparations_matcher,
                                    }

                                    div { class: "place-self-end",
                                        Button {
                                            variant: ButtonVariant::Outline,

                                            onclick: move |_| {
                                                recipe.ingredients().remove(idx);
                                            },

                                            "X"
                                        }
                                    }
                                }
                            }

                            Button {
                                class: "sm:max-w-1/2",
                                variant: ButtonVariant::Secondary,
                                onclick: move |_| {
                                    recipe
                                        .ingredients()
                                        .push(types::RecipeIngredient {
                                            quantity: types::Quantity {
                                                amount: Some(1.0),
                                                reference_unit: types::ReferenceUnit::gram(),
                                                text: "1 gram".to_owned(),
                                            },
                                            reference_ingredient: types::Ingredient::flour(),
                                            reference_preparations: vec![],
                                            source_text: None,
                                        });
                                },

                                "Add ingredient"
                            }
                        }
                    }

                    TabContent { index: 1usize, value: "steps",
                        div { class: "flex flex-col gap-4 p-4",
                            for (idx , step) in recipe.steps().iter().enumerate() {
                                div { class: "flex flex-row gap-4",
                                    Step {
                                        idx,
                                        step,
                                        ingredients: recipe.ingredients(),
                                    }

                                    div { class: "place-self-end",
                                        Button {
                                            variant: ButtonVariant::Outline,

                                            onclick: move |_| {
                                                recipe.steps().remove(idx);
                                            },

                                            "X"
                                        }
                                    }
                                }
                            }

                            Button {
                                class: "sm:max-w-1/2",
                                variant: ButtonVariant::Secondary,
                                onclick: move |_| {
                                    recipe
                                        .steps()
                                        .push(types::RecipeStep {
                                            capability: None,
                                            ingredients: vec![],
                                            source_text: None,
                                            text: "Do something".to_owned(),
                                        });
                                },

                                "Add step"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn NewRecipe() -> Element {
    // todo: use correct author
    let id = rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 10);
    let recipe = use_store(move || types::Recipe {
        author: types::Author {
            name: "Kenwood".to_owned(),
            image: "https://configs.fresco-kitchenos.com/fresco-1180x1180.svg".to_owned(),
            url: "https://configs.fresco-kitchenos.com/fresco-1180x1180.svg".to_owned(),
        },
        name: "".to_owned(),
        description: "".to_owned(),
        prep_time: None,
        cook_time: None,
        total_time: jiff::SignedDuration::new(0, 0),
        created_at: chrono::Utc::now(),
        created_by_id: "toad".to_owned(),
        etag: id.clone(),
        forked_into_other_locales: vec![],
        ingredients: vec![],
        locale: "en-GB".to_owned(),
        modified_at: chrono::Utc::now(),
        organization_id: "toads".to_owned(),
        published_at: chrono::Utc::now(),
        reference_tags: vec![],
        serves: 1,
        state: "published".to_owned(),
        steps: vec![],
        visibility: "all-users".to_owned(),
        referenced: None,
        requester_role: None,
        id,
    });

    let nav = use_navigator();

    rsx! {
        EditRecipeInner { recipe }

        div { class: "flex flex-row justify-end gap-4",
            Button {
                onclick: move |_| {
                    let recipe = recipe();
                    let id = recipe.id.clone();
                    async move {
                        let _ = save_recipe_server(recipe, true).await;

                        nav.replace(crate::Route::EditRecipe { id });
                    }
                },

                "Create recipe"
            }
        }
    }
}

#[component]
pub fn EditRecipe(id: String) -> Element {
    let recipe_initial = use_loader(move || recipe_server(id.clone()))?.cloned();
    let recipe = use_store(move || recipe_initial);

    rsx! {
        EditRecipeInner { recipe }

        div { class: "flex flex-row justify-end gap-4",
            Button {
                onclick: move |_| {
                    let recipe = recipe();
                    async {
                        let _ = save_recipe_server(recipe, false).await;
                    }
                },

                "Update recipe"
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct UploadImage {
    id: String,
    data: Vec<u8>,
}

#[server]
async fn upload_image(mut form: MultipartFormData) -> Result<()> {
    use dioxus::CapturedError;

    let mut recipe_id = None;
    let mut data = None;

    while let Ok(Some(field)) = form.next_field().await {
        if field.name() == Some("id") {
            recipe_id = Some(field.text().await?);
            continue;
        }

        if field.name() == Some("data") {
            data = Some(field.bytes().await?);
            continue;
        }
    }

    let (Some(recipe_id), Some(data)) = (recipe_id, data) else {
        return Ok(());
    };

    let image = image::ImageReader::new(std::io::Cursor::new(&data))
        .with_guessed_format()
        .context("Guessing image format")?
        .decode()
        .context("Decoding image")?;

    let mut data = Vec::<u8>::new();
    image
        .write_to(std::io::Cursor::new(&mut data), image::ImageFormat::WebP)
        .context("Coverting image")?;

    db::queries::images::set_image(crate::db::db(), &recipe_id, data)
        .await
        .map_err(|e| CapturedError::from_boxed(e.into()))?;

    Ok(())
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
async fn save_recipe_server(recipe: types::Recipe, create: bool) -> Result<()> {
    use dioxus::{
        logger::tracing::{info_span, Instrument as _},
        CapturedError,
    };

    let recipe = db::queries::recipes::set_recipe(crate::db::db(), recipe, create)
        .instrument(info_span!("Setting recipe"))
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
