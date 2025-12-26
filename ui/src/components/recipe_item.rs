use dioxus::prelude::*;

use crate::components::button::*;
use crate::components::card::*;
use crate::Route;

#[component]
pub fn RecipeItem(recipe: types::Recipe) -> Element {
    let recipe_id = recipe.id.clone();
    let image = use_loader(move || image(recipe_id.clone()))?;
    let image = use_memo(move || {
        image
            .cloned()
            .map(|x| photon_rs::PhotonImage::new_from_byteslice(x).get_base64())
    });

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
                img { width: "100px", height: "100px", src: image }
            }
        }
    }
}

#[server]
async fn image(recipe_id: String) -> Result<Option<Vec<u8>>> {
    use dioxus::logger::tracing::{info_span, Instrument as _};

    let image = db::queries::images::get_image(crate::db::db(), &recipe_id)
        .instrument(info_span!("Loading image"))
        .await
        .ok();

    Ok(image)
}
