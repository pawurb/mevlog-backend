use crate::{
    config::{host, routes::html_response},
    misc::utils::deployed_at,
};
use askama::Template;
use axum::response::IntoResponse;
use reqwest::StatusCode;

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate {
    host: String,
    page: String,
    deployed_at: String,
}

pub async fn not_found() -> impl IntoResponse {
    let template = NotFoundTemplate {
        host: host(),
        page: "404".to_string(),
        deployed_at: deployed_at(),
    };

    html_response(template.render().unwrap(), StatusCode::NOT_FOUND)
}
