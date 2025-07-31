use alloy::primitives::Address;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::info;

use crate::{
    AppState,
    chains::Chains,
    routes::{balance::fetch_balance, tokens::fetch_token_balances},
    types::WalletSummary,
};

pub async fn get_wallet(
    Path((chain, address)): Path<(Chains, Address)>,
    State(state): State<AppState>,
) -> Response {
    info!("Fetching wallet summary for {address}");

    match state.chains.get(&chain) {
        Some(chain) => {
            let native_balance = match fetch_balance(address, chain.client()).await {
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

            let tokens = fetch_token_balances(address, chain.client(), chain.tokens()).await;

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
