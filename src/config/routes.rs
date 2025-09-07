use crate::{auth::AuthState, controllers::*, misc::utils::deployed_at};
use axum::{
    Extension, Router,
    body::Body,
    http::{HeaderMap, HeaderValue, Response, StatusCode},
    middleware,
    response::IntoResponse,
    routing::get,
};
use tower::Layer;
use tower_http::services::{ServeDir, ServeFile};

use super::{cache_control, middleware::update_user_activity};

pub async fn app() -> Router {
    let deployed_at = deployed_at();

    let auth_state = AuthState::new().await.expect("Failed to create auth state");
    let database = auth_state.database.clone();

    let app = Router::new()
        .route("/", get(html::home_controller::home))
        .route("/search", get(html::search_controller::search))
        .route("/terms", get(html::terms_controller::terms))
        .route("/explore", get(html::explore_controller::explore))
        .route(
            "/api/chain-info",
            get(json::chain_info_controller::chain_info),
        )
        .route("/api/chains", get(json::chains_controller::chains))
        .route("/api/explore", get(json::explore_controller::explore))
        .route("/ws/search", get(websocket::search_controller::ws_handler))
        .route("/uptime", get(|| async move { "OK".into_response() }))
        .merge(crate::auth::auth_routes())
        .layer(Extension(database))
        .layer(middleware::from_fn(update_user_activity))
        .with_state(auth_state)
        .route_service(
            &format!("/{deployed_at}-scripts.js"),
            cache_control().layer(ServeFile::new(format!("assets/{deployed_at}-scripts.js"))),
        )
        .route_service(
            &format!("/{deployed_at}-styles.css"),
            cache_control().layer(ServeFile::new(format!("assets/{deployed_at}-styles.css"))),
        )
        .route_service(
            &format!("/{deployed_at}-terminal.css"),
            cache_control().layer(ServeFile::new(format!("assets/{deployed_at}-terminal.css"))),
        )
        .route_service(
            &format!("/{deployed_at}-react-bundle.js"),
            cache_control().layer(ServeFile::new(format!(
                "assets/{deployed_at}-react-bundle.js"
            ))),
        )
        .fallback_service(cache_control().layer(ServeDir::new("assets")));

    app
}

pub fn invalid_req(reason: &str) -> Response<Body> {
    (StatusCode::BAD_REQUEST, reason.to_string()).into_response()
}

pub fn html_response(body: String, status: StatusCode) -> Response<Body> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("text/html; charset=utf-8"),
    );

    (status, headers, body).into_response()
}

pub fn json_response(body: String, status: StatusCode) -> Response<Body> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    (status, headers, body).into_response()
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use axum::http::Request;
    use eyre::Result;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    pub async fn get_test_app() -> Result<Router> {
        Ok(app().await)
    }

    #[tokio::test]
    async fn uptime_test() -> Result<()> {
        let app = get_test_app().await?;
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/uptime")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(body, "OK");
        Ok(())
    }
}
