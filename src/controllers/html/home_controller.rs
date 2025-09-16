use askama::Template;
use axum::{http::HeaderMap, response::IntoResponse};
use reqwest::StatusCode;

use crate::{
    auth::{GitHubUser, get_user_from_cookies},
    config::{host, routes::html_response},
    misc::utils::deployed_at,
};

// force html views recompilation by changing this value
const _VIEW_VERSION: u64 = 10;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    host: String,
    page: String,
    deployed_at: String,
    user: Option<GitHubUser>,
}

pub async fn home(headers: HeaderMap) -> impl IntoResponse {
    tracing::debug!("Home controller called");
    tracing::debug!("Headers: {:?}", headers);
    let user = get_user_from_cookies(&headers);
    tracing::debug!("User from cookies: {:?}", user);

    let template = HomeTemplate {
        host: host(),
        page: "home".to_string(),
        deployed_at: deployed_at(),
        user,
    };

    html_response(template.render().unwrap(), StatusCode::OK)
}
