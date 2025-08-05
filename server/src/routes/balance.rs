use alloy::primitives::{Address, U256};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::info;

use crate::AppState;

#[derive(Serialize)]
pub struct WalletBalanceResponse {
    pub address: Address,
    pub balance: U256,
}

pub async fn get_balance(
    Path((chain, address)): Path<(String, Address)>,
    State(state): State<AppState>,
) -> Response {
    info!("Request for {address}");

    match state.registry.get(&chain) {
        Some(client) => match client.get_native_balance(address).await {
            Ok(balance) => {
                let response = WalletBalanceResponse { address, balance };
                (StatusCode::OK, Json(response)).into_response()
            }
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to fetch balance",
                    "details": err.to_string()
                })),
            )
                .into_response(),
        },
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Chain not found"})),
        )
            .into_response(),
    }
}
