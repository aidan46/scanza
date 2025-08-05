use alloy::primitives::Address;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::info;

use crate::{AppState, types::WalletSummary};

pub async fn get_wallet(
    Path((chain, address)): Path<(String, Address)>,
    State(state): State<AppState>,
) -> Response {
    info!("Fetching wallet summary for {address}");

    match state.registry.get(&chain) {
        Some(client) => {
            let native_balance = match client.get_native_balance(address).await {
                Ok(balance) => balance,
                Err(err) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": "Failed to fetch balance",
                            "details": err.to_string()
                        })),
                    )
                        .into_response();
                }
            };

            let tokens = client.get_token_balances(address).await;

            info!("native_balance: {native_balance}");
            let response = WalletSummary {
                address,
                native_balance,
                tokens,
            };

            (StatusCode::OK, Json(response)).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Unknown chain"})),
        )
            .into_response(),
    }
}
