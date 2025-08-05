use alloy::primitives::{Address, U256};
use multichain_client::TokenBalance;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WalletSummary {
    pub address: Address,
    pub native_balance: U256,
    pub tokens: Vec<TokenBalance>,
}
