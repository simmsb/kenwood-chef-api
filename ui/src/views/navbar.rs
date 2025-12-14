use crate::{Route, components::button::LinkButton};
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            class: "m-4 flex p-1 rounded-1 gap-1",

            div {
                class: "relative",

                LinkButton {
                    variant: crate::components::button::ButtonVariant::Secondary,
                    to: Route::Home {},

                    "Home"
                }
            }

        }

        Outlet::<Route> {}
    }
}
