use alloy::primitives::{Address, U256};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use multichain_client::TokenBalance;
use serde::Serialize;
use tracing::{error, info, warn};

use crate::AppState;

#[derive(Debug, Serialize)]
struct WalletSummary {
    pub address: Address,
    pub native_balance: U256,
    pub tokens: Vec<TokenBalance>,
}

pub async fn get_wallet(
    Path((chain, address)): Path<(String, Address)>,
    State(state): State<AppState>,
) -> Response {
    info!("Getting wallet summary for {address} on {chain}");

    match state.registry.get(&chain) {
        Some(client) => {
            let native_balance = match client.get_native_balance(address).await {
                Ok(balance) => balance,
                Err(err) => {
                    error!("Failed to get wallet summary for {address} on {chain}: {err}");
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": "Failed to get wallet summary",
                            "details": err.to_string()
                        })),
                    )
                        .into_response();
                }
            };

            let tokens = client.get_token_balances(address).await;

            let response = WalletSummary {
                address,
                native_balance,
                tokens,
            };

            (StatusCode::OK, Json(response)).into_response()
        }
        None => {
            warn!("Chain not found: {chain}");
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Unknown chain"})),
            )
                .into_response()
        }
    }
}
