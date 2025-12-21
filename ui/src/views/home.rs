use std::num::Saturating;

use crate::components::{paginate::Pagination, toggle::*, RecipeItem};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let mut current_page = use_signal(|| Saturating(0u64));
    let mut show_all = use_signal(|| false);
    let recipes =
        use_loader(move || recipes_server(Some(current_page().0), Some(100), show_all()))?;

    rsx! {
        Toggle {
            class: "p-2",
            pressed: show_all(),
            on_pressed_change: move |p| show_all.set(p),

            span { "Show all recipes" }
        }

        Pagination {
            prev_page: move |()| {
                *current_page.write() -= 1;
            },
            next_page: move |()| {
                *current_page.write() += 1;
            },

            div { class: "flex flex-col gap-4",

                for recipe in recipes.cloned() {
                    RecipeItem { recipe }
                }
            }
        }
    }
}

struct AlternateDisplay<T>(pub T);

impl<T: core::fmt::Display + core::fmt::Debug> core::fmt::Display for AlternateDisplay<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[server]
async fn recipes_server(
    offset: Option<u64>,
    limit: Option<u64>,
    all: bool,
) -> Result<Vec<types::Recipe>> {
    use dioxus::{
        logger::tracing::{info_span, Instrument as _},
        CapturedError,
    };

    let recipes = db::queries::recipes::list_recipes(crate::db::db(), offset, limit, all)
        .instrument(info_span!("Loading recipes"))
        .await
        // .map_err(|e| CapturedError::from_boxed(e.into()))?;
        .map_err(|e| CapturedError::from_display(AlternateDisplay(e)))?;

    Ok(recipes)
}
