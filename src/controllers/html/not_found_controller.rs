use crate::{
    auth::{GitHubUser, get_user_from_cookies},
    config::{host, routes::html_response},
    misc::utils::deployed_at,
};
use askama::Template;
use axum::{http::HeaderMap, response::IntoResponse};
use reqwest::StatusCode;

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate {
    host: String,
    page: String,
    deployed_at: String,
    user: Option<GitHubUser>,
}

pub async fn not_found(headers: HeaderMap) -> impl IntoResponse {
    let user = get_user_from_cookies(&headers);

    let template = NotFoundTemplate {
        host: host(),
        page: "404".to_string(),
        deployed_at: deployed_at(),
        user,
    };

    html_response(template.render().unwrap(), StatusCode::NOT_FOUND)
}
