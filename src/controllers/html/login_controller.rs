use askama::Template;
use axum::{http::HeaderMap, response::IntoResponse};
use reqwest::StatusCode;

use crate::{
    auth::{GitHubUser, get_user_from_cookies},
    config::{host, routes::html_response},
    misc::utils::deployed_at,
};

const _VIEW_VERSION: u64 = 1;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    host: String,
    page: String,
    deployed_at: String,
    user: Option<GitHubUser>,
}

pub async fn login(headers: HeaderMap) -> impl IntoResponse {
    let user = get_user_from_cookies(&headers);

    let template = LoginTemplate {
        host: host(),
        page: "login".to_string(),
        deployed_at: deployed_at(),
        user,
    };

    html_response(template.render().unwrap(), StatusCode::OK)
}
