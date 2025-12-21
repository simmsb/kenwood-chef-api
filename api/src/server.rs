use axum::http::{HeaderMap, Request};
use axum::response::IntoResponse;
use axum::{
    body::Body,
    extract::{Path, Query},
};
use color_eyre::eyre::{Context, OptionExt as _};
use http_body_util::BodyExt;
use sea_orm::DatabaseConnection;
use std::{
    io::Cursor,
    sync::LazyLock,
};
use tracing::{Instrument as _, debug, debug_span, error, info, warn};

pub(crate) static CERT: &[u8] = include_bytes!("../../server.crt");
pub(crate) static KEY: &[u8] = include_bytes!("../../server.key");

pub(crate) static REQ_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

pub static DB: tokio::sync::OnceCell<DatabaseConnection> = tokio::sync::OnceCell::const_new();

pub async fn db() -> &'static DatabaseConnection {
    DB.get_or_init(async || db::connect().await.unwrap()).await
}

pub struct AppError(color_eyre::eyre::Report);

pub(crate) type Result<T, E = AppError> = ::core::result::Result<T, E>;

impl From<color_eyre::eyre::Report> for AppError {
    fn from(value: color_eyre::eyre::Report) -> Self {
        Self(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        error!(err = ?self.0, "Error in handler");

        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Oops").into_response()
    }
}

#[derive(serde::Deserialize)]
struct ImageDimensions {
    width: u32,
    height: u32,
}

#[axum::debug_handler]
async fn recipe_hero(
    Path(recipe_id): Path<String>,
    Query(dims): Query<ImageDimensions>,
    headers: HeaderMap,
) -> Result<axum::response::Response> {
    if let Ok(mut image) = db::queries::images::get_image(db().await, &recipe_id).await {
        info!(recipe_id = recipe_id, "Found custom image");

        let decoded = image::ImageReader::new(Cursor::new(&image))
            .with_guessed_format()
            .context("Guessing image format")?
            .decode()
            .context("Decoding image")?;

        let resized = decoded.resize_to_fill(
            dims.width,
            dims.height,
            image::imageops::FilterType::Triangle,
        );

        image.clear();
        resized
            .write_to(Cursor::new(&mut image), image::ImageFormat::WebP)
            .context("Converting image")?;

        return Ok((
            axum::http::StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, "image/webp")],
            image,
        )
            .into_response());
    }

    info!(recipe_id = recipe_id, "Falling back on server image");

    let domain = headers
        .get(axum::http::header::HOST)
        .ok_or_eyre("Expected a host header")?
        .to_str()
        .context("Stringifying host")?;

    let url = reqwest::Url::parse(&format!(
        "https://{domain}/media/images/recipes/{recipe_id}/hero?width={}&height={}",
        dims.width, dims.height
    ))
    .context("Building URL")?;

    let resp = REQ_CLIENT
        .get(url)
        .headers(headers.clone())
        .send()
        .instrument(debug_span!("fallback_request"))
        .await
        .context("Making fallback proxy request")?;

    let (resp_parts, resp_body) = axum::http::Response::from(resp).into_parts();
    let resp_body = resp_body
        .collect()
        .await
        .context("Reading response body")?
        .to_bytes();

    Ok(axum::http::Response::from_parts(
        resp_parts,
        resp_body.into(),
    ))
}

#[axum::debug_handler]
async fn recipe(
    Path(recipe_id): Path<String>,
    headers: HeaderMap,
) -> Result<axum::Json<types::Recipe>> {
    if let Ok(custom) = db::queries::recipes::get_recipe(db().await, &recipe_id).await {
        info!(recipe_id = recipe_id, "Found custom recipe");
        debug!(recipe = ?custom, "Full recipe json");

        return Ok(axum::Json(custom));
    }

    info!(recipe_id = recipe_id, "Falling back on server recipe");

    let domain = headers
        .get(axum::http::header::HOST)
        .ok_or_eyre("Expected a host header")?
        .to_str()
        .context("Stringifying host")?;

    let url = reqwest::Url::parse(&format!("https://{domain}/recipes/{recipe_id}"))
        .context("Building URL")?;

    let resp = REQ_CLIENT
        .get(url)
        .headers(headers.clone())
        .send()
        .instrument(debug_span!("fallback_request"))
        .await
        .context("Making fallback proxy request")?
        .json::<types::Recipe>()
        .await
        .context("Reading json")?;

    Ok(axum::Json(resp))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct RecipesResponse {
    total: usize,
    items: Vec<types::RecipeItem>,
}

#[axum::debug_handler]
async fn collections_saved_recipes(headers: HeaderMap) -> Result<axum::Json<RecipesResponse>> {
    let domain = headers
        .get(axum::http::header::HOST)
        .ok_or_eyre("Expected a host header")?
        .to_str()
        .context("Stringifying host")?;

    let url = reqwest::Url::parse(&format!("https://{domain}/collections/saved-recipes"))
        .context("Building URL")?;

    let mut resp = REQ_CLIENT
        .get(url)
        .headers(headers.clone())
        .send()
        .instrument(debug_span!("fallback_request"))
        .await
        .context("Making fallback proxy request")?
        .json::<RecipesResponse>()
        .await
        .context("Reading json")?;

    let mut custom = db::queries::recipes::list_recipe_items(db().await, None, None, false).await?;

    custom.extend(resp.items);
    resp.items = custom;

    Ok(axum::Json(resp))
}

#[axum::debug_handler]
pub(crate) async fn api_fallback(req: Request<Body>) -> Result<axum::response::Response> {
    let (parts, body) = req.into_parts();
    let body = axum::body::to_bytes(body, usize::MAX)
        .await
        .context("Reading request body")?;
    warn!(req = ?parts, body = ?body, "Unhandled method");

    let domain = parts
        .headers
        .get(axum::http::header::HOST)
        .ok_or_eyre("Expected a host header")?
        .to_str()
        .context("Stringifying host")?;
    let mut url = reqwest::Url::parse(&format!("https://{}", domain)).context("Building URL")?;
    url.set_path(parts.uri.path());
    url.set_query(parts.uri.query());

    let resp = REQ_CLIENT
        .request(parts.method, url)
        .headers(parts.headers.clone())
        .body(body)
        .send()
        .instrument(debug_span!("fallback_request"))
        .await
        .context("Making fallback proxy request")?;

    let (resp_parts, resp_body) = axum::http::Response::from(resp).into_parts();
    let resp_body = resp_body
        .collect()
        .await
        .context("Reading response body")?
        .to_bytes();

    let preview_body = resp_body.slice(0..resp_body.len().min(200));

    info!(req = ?resp_parts, body = ?preview_body, "Got response for fallback");

    Ok(axum::http::Response::from_parts(
        resp_parts,
        resp_body.into(),
    ))
}

pub async fn run() -> color_eyre::Result<()> {
    let rustls =
        axum_server::tls_rustls::RustlsConfig::from_pem(CERT.to_vec(), KEY.to_vec()).await?;

    let app = axum::Router::new()
        .route(
            "/collections/saved-recipes/",
            axum::routing::get(collections_saved_recipes),
        )
        .route("/recipes/{recipe_id}", axum::routing::get(recipe))
        .route(
            "/media/images/recipes/{recipe_id}/hero",
            axum::routing::get(recipe_hero),
        )
        .fallback(axum::routing::any(api_fallback));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 443));

    axum_server::bind_rustls(addr, rustls)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
