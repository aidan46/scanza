//! This library provides a modular interface for interacting with multiple EVM-compatible
//! chains using a generic client. It includes functionality to:
//! - Parse JSON into metadata structs
//! - Initialize JSON-RPC and Etherscan clients
//! - Fetch native and token balances
//! - Aggregate transactions across multiple chains
//!

mod client;
mod metadata;

pub use client::{EvmChainClient, EvmClientRegistry};
pub use metadata::{ChainMetaData, NativeCurrency, TokenBalance, TokenMetadata};
