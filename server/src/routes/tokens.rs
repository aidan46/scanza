use alloy::primitives::Address;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use multichain_client::TokenBalance;
use serde::Serialize;
use tracing::info;

use crate::AppState;

#[derive(Debug, Serialize)]
struct TokenResponse {
    address: Address,
    tokens: Vec<TokenBalance>,
}

pub async fn get_tokens(
    Path((chain, address)): Path<(String, Address)>,
    State(state): State<AppState>,
) -> Response {
    info!("Getting token balances for {address}");

    match state.registry.get(&chain) {
        Some(client) => {
            let balances = client.get_token_balances(address).await;

            let response = serde_json::json!(TokenResponse {
                address,
                tokens: balances,
            });
            (StatusCode::OK, Json(response)).into_response()
        }
        None => (StatusCode::NOT_FOUND, "Chain not found").into_response(),
    }
}
