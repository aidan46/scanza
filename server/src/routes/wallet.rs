use alloy::primitives::{Address, U256};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::info;

use crate::{
    AppState,
    routes::{balance::fetch_balance, tokens::fetch_token_balances},
    types::TokenBalance,
};

#[derive(Debug, Serialize)]
struct WalletResponse {
    address: Address,
    native_balance: U256,
    tokens: Vec<TokenBalance>,
}

pub async fn get_wallet(Path(address): Path<Address>, State(state): State<AppState>) -> Response {
    info!("Fetching wallet summary for {address}");

    let native_balance = match fetch_balance(address, &state.client).await {
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

    let tokens = fetch_token_balances(address, &state.client, &state.tokens).await;

    let response = WalletResponse {
        address,
        native_balance,
        tokens,
    };

    (StatusCode::OK, Json(response)).into_response()
}
