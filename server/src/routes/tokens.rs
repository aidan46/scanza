use alloy::{
    hex,
    primitives::{Address, U256},
    rpc::client::ReqwestClient,
    sol,
    sol_types::SolCall,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use futures::future::join_all;
use serde::Serialize;
use tracing::info;

use crate::{
    AppState,
    chains::Chains,
    types::{TokenBalance, TokenMetadata},
};

sol! {
    function balanceOf(address) external view returns (uint256);
}

#[derive(Debug, Serialize)]
struct TokenResponse {
    address: Address,
    tokens: Vec<TokenBalance>,
}

pub async fn get_tokens(
    Path((chain, address)): Path<(Chains, Address)>,
    State(state): State<AppState>,
) -> Response {
    info!("Getting token balances for {address}");

    match state.chains.get(&chain) {
        Some(chain) => {
            let balances = fetch_token_balances(address, chain.client(), chain.tokens()).await;

            let response = serde_json::json!(TokenResponse {
                address,
                tokens: balances,
            });
            (StatusCode::OK, Json(response)).into_response()
        }
        None => (StatusCode::NOT_FOUND, "Chain not found").into_response(),
    }
}

pub async fn fetch_token_balances(
    address: Address,
    client: &ReqwestClient,
    tokens: &[TokenMetadata],
) -> Vec<TokenBalance> {
    let futures = tokens.iter().map(|token| {
        let token = token.clone();
        let client = client.clone();
        let call_data = balanceOfCall(address).abi_encode();

        let call = serde_json::json!({
            "to": format!("{:?}", token.address),
            "data": format!("0x{}", hex::encode(call_data)),
        });

        async move {
            match client
                .request::<_, U256>("eth_call", (call, "latest"))
                .await
            {
                Ok(balance) => {
                    if balance == U256::ZERO {
                        None
                    } else {
                        Some(TokenBalance { token, balance })
                    }
                }
                Err(err) => {
                    tracing::warn!("Error fetching balance for {}: {err}", token.symbol);
                    None
                }
            }
        }
    });

    join_all(futures).await.into_iter().flatten().collect()
}
