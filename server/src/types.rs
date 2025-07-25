use alloy::primitives::{Address, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct TokenMetadata {
    pub name: String,
    pub address: Address,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Debug, Serialize)]
pub struct TokenBalance {
    pub token: TokenMetadata,
    pub balance: U256,
}
