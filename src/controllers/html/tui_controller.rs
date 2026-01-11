use crate::{
    config::{host, routes::html_response},
    misc::utils::deployed_at,
};
use askama::Template;
use axum::response::IntoResponse;
use reqwest::StatusCode;

#[derive(Template)]
#[template(path = "tui.html")]
struct TuiTemplate {
    host: String,
    page: String,
    deployed_at: String,
}

#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub async fn tui() -> impl IntoResponse {
    let template = TuiTemplate {
        host: host(),
        page: "tui".to_string(),
        deployed_at: deployed_at(),
    };

    html_response(template.render().unwrap(), StatusCode::OK)
}
