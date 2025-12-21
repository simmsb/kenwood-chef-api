use dioxus::prelude::*;
use dioxus_primitives::toggle;

#[component]
pub fn Toggle(
    #[props(default)] class: String,
    /// The controlled pressed state of the toggle.
    pressed: ReadSignal<Option<bool>>,

    /// The default pressed state when uncontrolled.
    #[props(default)]
    default_pressed: bool,

    /// Whether the toggle is disabled.
    #[props(default)]
    disabled: ReadSignal<bool>,

    /// Callback fired when the pressed state changes.
    #[props(default)]
    on_pressed_change: Callback<bool>,

    // https://github.com/DioxusLabs/dioxus/issues/2467
    /// Callback fired when the toggle is mounted.
    #[props(default)]
    onmounted: Callback<Event<MountedData>>,
    /// Callback fired when the toggle receives focus.
    #[props(default)]
    onfocus: Callback<Event<FocusData>>,
    /// Callback fired when a key is pressed on the toggle.
    #[props(default)]
    onkeydown: Callback<Event<KeyboardData>>,

    /// Additional attributes to apply to the toggle element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children of the toggle component.
    children: Element,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        toggle::Toggle {
            class: "{class} toggle",
            pressed,
            default_pressed,
            disabled,
            on_pressed_change,
            onmounted,
            onfocus,
            onkeydown,
            attributes,
            {children}
        }
    }
}
