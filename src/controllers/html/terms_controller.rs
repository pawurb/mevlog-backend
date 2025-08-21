use crate::{
    config::{host, routes::html_response},
    misc::utils::deployed_at,
};
use askama::Template;
use axum::response::IntoResponse;
use reqwest::StatusCode;

#[derive(Template)]
#[template(path = "terms.html")]
struct HomeTemplate {
    host: String,
    page: String,
    deployed_at: String,
}

pub async fn terms() -> impl IntoResponse {
    let template = HomeTemplate {
        host: host(),
        page: "terms".to_string(),
        deployed_at: deployed_at(),
    };

    html_response(template.render().unwrap(), StatusCode::OK)
}
