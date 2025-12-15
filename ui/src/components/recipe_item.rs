use dioxus::prelude::*;

use crate::components::button::*;
use crate::components::card::*;
use crate::Route;

#[component]
pub fn RecipeItem(recipe: types::Recipe) -> Element {
    rsx! {
        Card {
            class: "w-full max-w-8",

            CardHeader {
                CardTitle { "{recipe.name}" }
                CardDescription { "{recipe.description}" }
                CardAction {
                    LinkButton {
                        to: Route::EditRecipe { id: recipe.id.clone() },

                        "Edit"
                    }
                }
            }

            CardContent {
                img {
                    width: "100px",
                    height: "100px",
                    src: "https://media.fresco-kitchenos.com/media/images/recipes/{recipe.id}/hero?width=100&height=100",
                    // loading: "lazy"
                }
            }
        }
    }
}
