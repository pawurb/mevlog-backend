use askama::Template;
use axum::response::IntoResponse;
use reqwest::StatusCode;

use crate::{
    config::{host, routes::html_response},
    misc::utils::deployed_at,
};

// force html views recompilation by changing this value
const _VIEW_VERSION: u64 = 16;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    host: String,
    page: String,
    deployed_at: String,
}

#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub async fn home() -> impl IntoResponse {
    tracing::debug!("Home controller called");

    let template = HomeTemplate {
        host: host(),
        page: "home".to_string(),
        deployed_at: deployed_at(),
    };

    html_response(template.render().unwrap(), StatusCode::OK)
}
