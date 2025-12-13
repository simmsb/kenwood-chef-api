use axum::body::Body;
use axum::http::Request;
use axum::response::IntoResponse;
use color_eyre::eyre::{Context, OptionExt as _};
use http_body_util::BodyExt;
use std::sync::LazyLock;
use tracing::{Instrument as _, debug_span, error, info, warn};

pub(crate) static CERT: &[u8] = include_bytes!("../../server.crt");
pub(crate) static KEY: &[u8] = include_bytes!("../../server.key");

pub(crate) static REQ_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());

pub struct AppError(color_eyre::eyre::Report);

pub(crate) type Result<T, E = AppError> = ::core::result::Result<T, E>;

impl From<color_eyre::eyre::Report> for AppError {
    fn from(value: color_eyre::eyre::Report) -> Self {
        Self(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        error!(err = %self.0, "Error in handler");

        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Oops").into_response()
    }
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

    let app = axum::Router::new().fallback(axum::routing::method_routing::any(api_fallback));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 443));

    axum_server::bind_rustls(addr, rustls)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
