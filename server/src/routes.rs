use axum::{Router, routing::get};

use crate::{
    AppState,
    routes::{balance::get_balance, tokens::get_tokens, wallet::get_wallet},
};

mod balance;
mod tokens;
mod wallet;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/wallet/{address}/balance", get(get_balance))
        .route("/wallet/{address}/tokens", get(get_tokens))
        .route("/wallet/{address}", get(get_wallet))
        .with_state(state)
}
