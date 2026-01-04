use dioxus::prelude::*;

use crate::components::button::*;
use crate::components::card::*;
use crate::Route;

#[component]
pub fn RecipeItem(recipe: types::Recipe) -> Element {
    let recipe_id = recipe.id.clone();

    rsx! {
        Card { class: "w-full",

            CardHeader {
                CardTitle { "{recipe.name}" }
                CardDescription { "{recipe.description}" }
                CardAction {
                    LinkButton {
                        to: Route::EditRecipe {
                            id: recipe.id.clone(),
                        },

                        "Edit"
                    }
                }
            }

            CardContent {
                img {
                    width: "100px",
                    height: "100px",
                    src: "image/{recipe_id}",
                }
            }
        }
    }
}

#[get("/image/:recipe_id")]
async fn image(recipe_id: String) -> Result<dioxus_fullstack::response::Response> {
    use dioxus::fullstack::response::IntoResponse;
    use dioxus::logger::tracing::{info_span, Instrument as _};

    let image = db::queries::images::get_image(crate::db::db(), &recipe_id)
        .instrument(info_span!("Loading image"))
        .await
        .ok();

    if let Some(image) = image {
        return Ok((
            StatusCode::OK,
            [(http::header::CONTENT_TYPE, "image/webp")],
            image,
        )
            .into_response());
    }

    Ok(StatusCode::NOT_FOUND.into_response())
}
