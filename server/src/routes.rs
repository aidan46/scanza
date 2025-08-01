use axum::{Json, Router, extract::State, routing::get};

use crate::{
    AppState,
    chains::Chains,
    routes::{
        balance::get_balance, tokens::get_tokens, transactions::get_transactions,
        wallet::get_wallet,
    },
};

mod balance;
mod tokens;
mod transactions;
mod wallet;

/// GET /chains — Returns list of loaded chains
pub async fn get_chains(State(state): State<AppState>) -> Json<Vec<Chains>> {
    let chains = state.chains.values().map(|c| *c.name()).collect();

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
