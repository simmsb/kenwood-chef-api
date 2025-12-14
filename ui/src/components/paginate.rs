use dioxus::prelude::*;

use super::button::Button;

#[component]
pub fn Pagination(prev_page: EventHandler, next_page: EventHandler, children: Element) -> Element {
    rsx! {
        div {
            {children}
        }

        div {
            class: "flex justify-end gap-4",

            Button {
                onclick: move |_| prev_page.call(()),

                "Previous"
            }

            Button {
                onclick: move |_| next_page.call(()),

                "Next"
            }
        }
    }
}
