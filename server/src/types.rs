use alloy::primitives::{Address, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct TokenMetadata {
    pub address: Address,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Debug, Deserialize)]
pub struct RawTokenMetadata {
    pub symbol: String,
    pub address: String,
    pub decimals: u8,
}

#[derive(Debug, Serialize)]
pub struct TokenBalance {
    pub token: TokenMetadata,
    pub balance: U256,
}
