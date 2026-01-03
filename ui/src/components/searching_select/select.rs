//! Main Select component implementation.

use core::panic;
use std::time::Duration;

use dioxus::prelude::*;

use super::context::*;
use super::focus::use_focus_provider;
use crate::components::utils::*;

/// Props for the main Select component
#[derive(Props, Clone, PartialEq)]
pub struct SelectProps<T: Clone + PartialEq + 'static = String> {
    /// The controlled value of the select
    #[props(default)]
    pub value: ReadSignal<Option<Option<T>>>,

    /// The default value of the select
    #[props(default)]
    pub default_value: Option<T>,

    /// Callback when the value changes
    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    /// Whether the select is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Name of the select for form submission
    #[props(default)]
    pub name: ReadSignal<String>,

    #[props(default = Signal::new(String::new()))]
    pub typeahead_buffer: Signal<String>,

    /// Optional placeholder text
    #[props(default = ReadSignal::new(Signal::new(String::from("Select an option"))))]
    pub placeholder: ReadSignal<String>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes for the select element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the Select component
    pub children: Element,
}

/// # Select
///
/// The `Select` component is a searchable dropdown that allows users to choose from a list of options with keyboard navigation and typeahead search functionality.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::select::{
///     Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
///     SelectTrigger, SelectValue,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Select::<String> {
///             placeholder: "Select a fruit...",
///             SelectTrigger {
///                 aria_label: "Select Trigger",
///                 width: "12rem",
///                 SelectValue {}
///             }
///             SelectList {
///                 aria_label: "Select Demo",
///                 SelectGroup {
///                     SelectGroupLabel { "Fruits" }
///                     SelectOption::<String> {
///                         index: 0usize,
///                         value: "apple",
///                         "Apple"
///                         SelectItemIndicator { "✔️" }
///                     }
///                     SelectOption::<String> {
///                         index: 1usize,
///                         value: "banana",
///                         "Banana"
///                         SelectItemIndicator { "✔️" }
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Select`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the select. Values are `open` or `closed`.
#[component]
pub fn Select<T: Clone + PartialEq + 'static>(props: SelectProps<T>) -> Element {
    let (value, set_value_internal) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let mut typeahead_buffer = props.typeahead_buffer;
    let open = use_signal(|| false);
    let options = use_signal(Vec::default);
    let list_id = use_signal(|| None);

    let value = use_memo(move || value().map(RcPartialEqValue::new));
    let set_value = use_callback(move |cursor_opt: Option<RcPartialEqValue>| {
        if let Some(value) = cursor_opt {
            set_value_internal.call(Some(
                value
                    .as_ref::<T>()
                    .unwrap_or_else(|| {
                        panic!("The values of select and all options must match types")
                    })
                    .clone(),
            ));
        } else {
            set_value_internal.call(None);
        }
    });

    let focus_state = use_focus_provider(props.roving_loop);

    // Clear the typeahead buffer when the select is closed
    use_effect(move || {
        if !open() {
            // Cancel any pending clear task
            // Clear the buffer immediately
            typeahead_buffer.take();
        }
    });
    let initial_focus = use_signal(|| None);

    use_context_provider(|| SelectContext {
        typeahead_buffer,
        open,
        value,
        set_value,
        options,
        list_id,
        focus_state,
        disabled: props.disabled,
        placeholder: props.placeholder,
        initial_focus,
    });

    rsx! {
        div {
            // Data attributes
            "data-state": if open() { "open" } else { "closed" },
            ..props.attributes,
            {props.children}
        }
    }
}
