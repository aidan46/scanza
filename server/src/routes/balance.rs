use alloy::{
    primitives::{Address, U256},
    rpc::client::ReqwestClient,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::{error, info};

use crate::{AppState, chains::Chains};

#[derive(Serialize)]
pub struct WalletBalanceResponse {
    pub address: Address,
    pub balance: U256,
}

pub async fn get_balance(
    Path((chain, address)): Path<(Chains, Address)>,
    State(state): State<AppState>,
) -> Response {
    info!("Request for {address}");

    match state.chains.get(&chain) {
        Some(chain) => match fetch_balance(address, chain.client()).await {
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

pub async fn fetch_balance(address: Address, client: &ReqwestClient) -> Result<U256, String> {
    match client
        .request::<_, U256>("eth_getBalance", (address, "latest"))
        .await
    {
        Ok(balance) => Ok(balance),
        Err(err) => {
            let error_msg = format!("Error while fetching balance: {err}");
            error!("Error while fetching balance: {err}");
            Err(error_msg)
        }
    }
}
