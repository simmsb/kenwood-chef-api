use crate::{components::button::LinkButton, Route};
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { class: "m-4 flex p-1 rounded-1 gap-1",

            // div { class: "relative",

            LinkButton {
                variant: crate::components::button::ButtonVariant::Secondary,
                to: Route::Home {},

                "Home"
            }

            LinkButton {
                variant: crate::components::button::ButtonVariant::Secondary,
                to: Route::NewRecipe {},

                "New recipe"
            }
                // }
        }

        div { class: "flex justify-center",
            div { class: "flex flex-col w-3/4 gap-4", Outlet::<Route> {} }
        }
    }
}
