
use crate::config::{host, routes::html_response};
use crate::controllers::base_controller::empty_string_as_none;
use crate::controllers::json::base_controller::extract_query_params;
use crate::misc::utils::deployed_at;
use askama::Template;
use axum::{extract::Query, response::IntoResponse};
use eyre::Result;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::controllers::base_controller::{
    error_message, get_default_blocks, get_default_position, loading_spinner,
};

#[derive(Template)]
#[template(path = "search.html")]
struct SearchTemplate {
    output: String,
    blocks: String,
    position: String,
    from: String,
    to: String,
    event: String,
    not_event: String,
    method: String,
    erc20_transfer: String,
    tx_cost: String,
    gas_price: String,
    host: String,
    page: String,
    deployed_at: String,
    chain_id: String,
}

impl SearchTemplate {
    pub fn new(params: SearchParams, output: String) -> Self {
        let blocks = get_default_blocks(params.blocks);
        let position = get_default_position(params.position);

        Self {
            output,
            blocks,
            position,
            from: params.from.unwrap_or_default(),
            to: params.to.unwrap_or_default(),
            event: params.event.unwrap_or_default(),
            not_event: params.not_event.unwrap_or_default(),
            method: params.method.unwrap_or_default(),
            erc20_transfer: params.erc20_transfer.unwrap_or_default(),
            tx_cost: params.tx_cost.unwrap_or_default(),
            gas_price: params.gas_price.unwrap_or_default(),
            host: host(),
            page: "search".to_string(),
            deployed_at: deployed_at(),
            chain_id: params.chain_id.unwrap_or(1).to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct SearchParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub blocks: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub position: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub from: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub to: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub event: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub not_event: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub method: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub erc20_transfer: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub tx_cost: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub gas_price: Option<String>,
    pub chain_id: Option<u64>,
}

impl SearchParams {
    pub async fn validate(&self) -> Result<()> {
        Ok(())
    }
}

pub async fn search(
    query: Result<Query<SearchParams>, axum::extract::rejection::QueryRejection>,
) -> impl IntoResponse {
    let params = match extract_query_params(query) {
        Ok(params) => params,
        Err(e) => return error_message(&e).into_response(),
    };

    let (output, status) = match params.validate().await {
        Ok(_) => (loading_spinner(), StatusCode::OK),
        Err(e) => (error_message(&e.to_string()), StatusCode::BAD_REQUEST),
    };

    let template = SearchTemplate::new(params, output);

    html_response(template.render().unwrap(), status)
}
