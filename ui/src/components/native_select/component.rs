use std::{any::Any, rc::Rc};

use dioxus::prelude::*;

use crate::components::utils::{use_controlled, use_effect_cleanup, use_unique_id};

#[derive(Props, Clone, PartialEq)]
pub struct NativeSelectProps<T: Clone + PartialEq + 'static> {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    pub value: ReadSignal<Option<Option<T>>>,
    pub default_value: Option<T>,
    pub on_value_change: Callback<Option<T>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = select)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn NativeSelect<T: Clone + PartialEq + 'static>(props: NativeSelectProps<T>) -> Element {
    let (value, set_value_internal) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let options = use_store(|| Vec::new());
    let mut selected = use_store(move || value().map(|x| Rc::new(x) as Rc<dyn Any>));

    use_effect(move || {
        let value_ = value();
        selected.set(value_.map(|x| Rc::new(x) as Rc<dyn Any>));
    });

    use_context_provider(|| NativeSelectContext { options, selected });

    let on_change = use_callback(move |evt: Event<FormData>| {
        let value = evt.value();

        let m_selected = options
            .iter()
            .find(|x| x.id().read().as_str() == value.as_str());

        if let Some(v_selected) = m_selected {
            let value = v_selected.read().value.clone();
            let value_as_t = value
                .downcast_ref::<T>()
                .expect("Option of NativeSelect had incorrect type")
                .clone();
            set_value_internal.call(Some(value_as_t));
        } else {
            set_value_internal.call(None);
        }
    });

    rsx! {
        select {
            class: "px-2 py-3 w-fit rounded-[0.5rem] bg-(--primary-color) dark:bg-(--primary-color-3)",
            class: "inset-ring inset-ring-primary-color-6 dark:inset-ring-primary-color-7",
            class: "text-(--secondary-color-4)",
            autocomplete: "off",
            onchange: on_change,
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct NativeSelectOptionProps<T: PartialEq + 'static> {
    pub value: T,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    #[props(extends = option)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn NativeSelectOption<T: Clone + PartialEq + 'static>(
    props: NativeSelectOptionProps<T>,
) -> Element {
    let id = use_unique_id();

    let mut ctx: NativeSelectContext = use_context();
    let value = props.value.clone();
    use_effect(move || {
        let option_state = NativeSelectOptionState {
            value: Rc::new(value.clone()),
            id: id(),
        };

        // Add the option to the context's options
        ctx.options.push(option_state);
    });

    use_effect_cleanup(move || {
        ctx.options.retain(|opt| opt.id != *id.read());
    });

    let selected = use_memo(move || {
        if let Some(v) = ctx.selected.transpose() {
            if let Some(v) = v.read().downcast_ref::<T>() {
                return v == &props.value;
            }
        }

        false
    });

    rsx! {
        option { value: id, selected, ..props.attributes, {props.children} }
    }
}

#[derive(Store)]
struct NativeSelectOptionState {
    value: Rc<dyn Any>,
    id: String,
}

#[derive(Clone, Copy)]
struct NativeSelectContext {
    options: Store<Vec<NativeSelectOptionState>>,
    selected: Store<Option<Rc<dyn Any>>>,
}
