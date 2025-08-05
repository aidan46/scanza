use axum::{Json, Router, extract::State, routing::get};

use crate::{
    AppState,
    routes::{
        balance::get_balance, tokens::get_tokens, transactions::get_transactions,
        wallet::get_wallet,
    },
};

mod balance;
mod tokens;
mod transactions;
mod wallet;

use serde::Serialize;

#[derive(Serialize)]
pub struct ChainInfo {
    #[serde(rename = "shortName")]
    pub short_name: String,
    pub name: String,
}

/// GET /chains — Returns list of loaded chains
pub async fn get_chains(State(state): State<AppState>) -> Json<Vec<ChainInfo>> {
    let chains = state
        .registry
        .inner()
        .iter()
        .map(|(key, client)| ChainInfo {
            short_name: key.to_string(),
            name: client.name().to_string(),
        })
        .collect();

    Json(chains)
}

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/chains", get(get_chains))
        .route("/{chain}/wallet/{address}/balance", get(get_balance))
        .route("/{chain}/wallet/{address}/tokens", get(get_tokens))
        .route(
            "/{chain}/wallet/{address}/transactions",
            get(get_transactions),
        )
        .route("/{chain}/wallet/{address}", get(get_wallet))
        .with_state(state)
}
