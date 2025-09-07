use crate::{
    auth::{get_user_from_cookies, GitHubUser},
    config::{host, routes::html_response},
    misc::utils::deployed_at,
};
use askama::Template;
use axum::{http::HeaderMap, response::IntoResponse};
use reqwest::StatusCode;

#[derive(Template)]
#[template(path = "terms.html")]
struct HomeTemplate {
    host: String,
    page: String,
    deployed_at: String,
    user: Option<GitHubUser>,
}

pub async fn terms(headers: HeaderMap) -> impl IntoResponse {
    let user = get_user_from_cookies(&headers);

    let template = HomeTemplate {
        host: host(),
        page: "terms".to_string(),
        deployed_at: deployed_at(),
        user,
    };

    html_response(template.render().unwrap(), StatusCode::OK)
}
