// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use views::{EditRecipe, Home, Ingest, Navbar, NewRecipe};

/// Define a components module that contains all shared components for our app.
mod components;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

#[cfg(feature = "server")]
pub mod db;

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
/// 
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/edit/:id")]
    EditRecipe { id: String },
    #[route("/new")]
    NewRecipe {},
    #[route("/ingest")]
    Ingest {},
}

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const THEME_CSS: Asset = asset!("/assets/dx-components-theme.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[cfg(feature = "server")]
fn setup_server() -> Option<core::convert::Infallible> {
    let Some(supervisor_token) = std::env::var("SUPERVISOR_TOKEN").ok() else {
        return None;
    };

    let client = reqwest::blocking::Client::new();
    let Some(path_prefix) = client
        .get("http://supervisor/addons/self/info")
        .bearer_auth(supervisor_token)
        .send()
        .ok()
        .and_then(|x| x.json::<serde_json::Map<String, serde_json::Value>>().ok())
        .and_then(|mut x| x.remove("ingress_url"))
        .and_then(|x| match x {
            serde_json::Value::String(s) => Some(s),
            _ => None,
        })
    else {
        return None;
    };

    info!("Found ingress prefix: {path_prefix}");

    Some(serve_server(App, path_prefix).into())
}

#[cfg(feature = "server")]
fn serve_server(original_root: fn() -> Result<VNode, RenderError>, base_path: String) -> ! {
    let mut cfg = ServeConfig::new();

    let cb = move || {
        let cfg = cfg.clone();
        let base_path = base_path.clone();
        Box::pin(async move {
            Ok(apply_base_path(
                dioxus_server::axum::Router::new()
                    .serve_dioxus_application(cfg.clone(), original_root),
                original_root,
                cfg.clone(),
                base_path.clone(),
            ))
        })
    };

    serve(cb)
}

#[cfg(feature = "server")]
fn apply_base_path<M: 'static>(
    router: dioxus::server::axum::Router,
    root: impl dioxus_core::ComponentFunction<(), M> + Send + Sync,
    cfg: ServeConfig,
    base_path: String,
) -> dioxus::server::axum::Router {
    let base_path = base_path.trim_matches('/');

    // If there is a base path, nest the router under it and serve the root route manually
    // Nesting a route in axum only serves /base_path or /base_path/ not both
    dioxus::server::axum::Router::new()
        .nest(&format!("/{base_path}/"), router)
        .route(
            &format!("/{base_path}"),
            dioxus::server::axum::routing::method_routing::get(
                |state: dioxus_fullstack::extract::State<dioxus_server::FullstackState>,
                 mut request: http::Request<dioxus_fullstack::body::Body>| async move {
                    // The root of the base path always looks like the root from dioxus fullstack
                    *request.uri_mut() = "/".parse().unwrap();
                    dioxus_server::FullstackState::render_handler(state, request).await
                },
            )
            .with_state(dioxus_server::FullstackState::new(cfg, root)),
        )
}

fn main() {
    #[cfg(feature = "server")]
    {
        setup_server();
    }

    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: THEME_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // The router component renders the route enum we defined above. It will handle synchronization of the URL and render
        // the layouts and components for the active route.
        components::toast::ToastProvider { Router::<Route> {} }

        div { class: "mt-10" }
    }
}
