//! Types related to ERC-20 token metadata and balances.

use alloy::primitives::{Address, U256};
use serde::{Deserialize, Serialize};

/// Metadata about an ERC-20 token.
///
/// This includes static, chain-specific details about the token.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenMetadata {
    /// Human-readable token name (e.g., "Tether USD")
    pub name: String,

    /// Contract address of the token on the chain
    pub address: Address,

    /// Token symbol (e.g., "USDT")
    pub symbol: String,

    /// Number of decimals the token uses (e.g., 6 for USDT, 18 for most others)
    pub decimals: u8,
}

/// A dynamic balance of a token for a specific wallet address.
///
/// Wraps a [`TokenMetadata`] struct and the current `balance` as a `U256`.
#[derive(Debug, Deserialize, Serialize)]
pub struct TokenBalance {
    /// Token metadata (name, symbol, decimals, address)
    pub token: TokenMetadata,

    /// Balance of the token for the address being queried
    pub balance: U256,
}
