//! SelectList component implementation.

use std::time::Duration;

use dioxus::prelude::*;
use dioxus_sdk::time::use_timeout;
use dioxus_sdk::time::TimeoutHandle;

use super::context::*;
use super::utils::*;

/// The props for the [`SelectList`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectListProps {
    /// The ID of the list for ARIA attributes
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes for the list
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the list
    pub children: Element,
}

/// # SelectList
///
/// The dropdown list container for the [`Select`](super::select::Select) component that contains the
/// [`SelectOption`](super::option::SelectOption)s. The list will only be rendered when the select is open.
///
/// This must be used inside a [`Select`](super::select::Select) component.
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
#[component]
pub fn SelectList(props: SelectListProps) -> Element {
    let mut ctx = use_context::<SelectContext>();

    let id = use_unique_id();
    let id = use_id_or(id, props.id);
    use_effect(move || {
        ctx.list_id.set(Some(id()));
    });

    let mut open = ctx.open;
    let mut listbox_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    let mut input_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);

    let focused = move || open() && !ctx.focus_state.any_focused();

    use_effect(move || {
        let Some(listbox_ref) = listbox_ref() else {
            return;
        };
        if focused() {
            spawn(async move {
                _ = listbox_ref.set_focus(true).await;
            });
        }
    });

    let onkeydown = move |event: KeyboardEvent| {
        let key = event.key();

        let arrow_key_navigation = |event: KeyboardEvent| {
            event.prevent_default();
            event.stop_propagation();
        };

        match key {
            Key::Character(_) => {
                if let Some(input_ref) = input_ref() {
                    spawn(async move {
                        _ = input_ref.set_focus(true).await;
                    });
                }
            }
            Key::ArrowUp => {
                arrow_key_navigation(event);
                ctx.focus_state.focus_prev();
            }
            Key::End => {
                arrow_key_navigation(event);
                ctx.focus_state.focus_last();
            }
            Key::ArrowDown => {
                arrow_key_navigation(event);
                ctx.focus_state.focus_next();
            }
            Key::Home => {
                arrow_key_navigation(event);
                ctx.focus_state.focus_first();
            }
            Key::Enter => {
                ctx.select_current_item();
                open.set(false);
                event.prevent_default();
                event.stop_propagation();
            }
            Key::Escape => {
                open.set(false);
                event.prevent_default();
                event.stop_propagation();
            }
            _ => {}
        }
    };

    let render = use_animated_open(id, open);
    let render = use_memo(render);

    use_context_provider(|| SelectListContext {
        render: render.into(),
    });

    use_effect(move || {
        if render() {
            ctx.focus_state.set_focus(ctx.initial_focus.cloned());
        } else {
            ctx.initial_focus.set(None);
        }
    });

    // close if we've been unfocused for a timeout, as we only know if we should
    // close after seeing all the onfocusin/onfocusout events this frame.
    let mut current_closed_timeout: Signal<Option<TimeoutHandle>> = use_signal(|| None);
    let closed_timeout = use_timeout(Duration::from_millis(10), move |()| {
        current_closed_timeout.set(None);
        open.set(false);
    });

    rsx! {
        if render() {
            div {
                id,
                role: "listbox",
                tabindex: if focused() { "0" } else { "-1" },

                // Data attributes
                "data-state": if open() { "open" } else { "closed" },

                onmounted: move |evt| listbox_ref.set(Some(evt.data())),
                onkeydown,
                onfocusin: move |_| {
                    if let Some(handle) = current_closed_timeout.write().take() {
                        handle.cancel();
                    }
                },
                onfocusout: move |_| {
                    if let Some(handle) = *current_closed_timeout.read() {
                        handle.cancel();
                    }

                    current_closed_timeout.set(Some(closed_timeout.action(())));
                },

                ..props.attributes,

                crate::components::input::Input {
                    class: "w-full mb-1",
                    value: ctx.typeahead_buffer,
                    oninput: move |e: FormEvent| {
                        *ctx.typeahead_buffer.write() = e.value();
                    },
                    onmounted: move |evt: MountedEvent| input_ref.set(Some(evt.data())),
                }
                {props.children}
            }
        } else {
            // If not rendering, return children directly so we can populate the selected list, but they should choose to not render themselves
            {props.children}
        }
    }
}
