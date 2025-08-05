//! Provides metadata structures and utility methods for initializing `ChainClient` instances
//! from chain-list-style JSON metadata (e.g. `chain-list.json`).

use std::sync::Arc;

use alloy::{
    rpc::client::{ClientBuilder, ReqwestClient},
    transports::http::reqwest::Url,
};
use alloy_chains::Chain;
use anyhow::{Context, Result};
use foundry_block_explorers::Client as EtherscanClient;
use serde::Deserialize;

use crate::{EvmChainClient, metadata::TokenMetadata};

/// Metadata describing a chain's native currency (e.g., ETH, MATIC, etc.)
#[derive(Clone, Debug, Deserialize)]
pub struct NativeCurrency {
    /// Full name of the native currency (e.g., "Ether")
    pub name: String,
    /// Symbol of the native currency (e.g., "ETH")
    pub symbol: String,
    /// Number of decimals used by the currency (typically 18)
    pub decimals: u64,
}

/// Chain metadata, usually parsed from chain-list JSON files.
///
/// Includes basic chain identity and a list of RPC endpoints.
#[derive(Debug, Deserialize)]
pub struct ChainMetaData {
    /// Full name of the chain (e.g., "Ethereum Mainnet")
    pub name: String,

    /// EVM chain ID (e.g., 1 for Ethereum)
    #[serde(rename = "chainId")]
    pub chain_id: u64,

    /// Short name identifier (e.g., "eth")
    #[serde(rename = "shortName")]
    pub short_name: String,

    /// Network ID (sometimes differs from chain ID)
    #[serde(rename = "networkId")]
    pub network_id: u64,

    /// Native currency metadata (name, symbol, decimals)
    #[serde(rename = "nativeCurrency")]
    pub native_currency: NativeCurrency,

    /// List of RPC URLs for the chain
    ///
    /// The first valid `http(s)` URL without template variables will be used.
    pub rpc: Vec<String>,
}

impl ChainMetaData {
    /// Create a new [`EvmChainClient`] from this metadata.
    ///
    /// Uses the first available HTTP RPC URL that doesn't contain any template variables (e.g. `{API_KEY}`).
    ///
    /// # Arguments
    /// * `etherscan_api_key` - A valid Etherscan API key for the corresponding chain.
    pub fn create_rpc_client(&self, etherscan_api_key: &str) -> Result<EvmChainClient> {
        let http_url = self
            .rpc
            .iter()
            .find(|url| url.starts_with("http") && !url.contains('{')) // skip urls that require templating
            .context(format!("No usable RPC HTTP URL for chain: {}", self.name))?;

        let url = Url::parse(http_url)?;
        let rpc_client: ReqwestClient = ClientBuilder::default().http(url);
        let etherscan = EtherscanClient::new(Chain::from_id(self.chain_id), etherscan_api_key)?;

        Ok(EvmChainClient::new(
            self.name.clone(),
            self.native_currency.clone(),
            Arc::new(rpc_client),
            Arc::new(etherscan),
            vec![],
        ))
    }

    /// Create a new [`EvmChainClient`] from this metadata, using a pre-initialized list of tokens.
    ///
    /// Uses the first available HTTP RPC URL that doesn't contain any template variables (e.g. `{API_KEY}`).
    ///
    /// # Arguments
    /// * `etherscan_api_key` - A valid Etherscan API key for the corresponding chain.
    /// * `tokens` - A list of `TokenMetadata` describing the tokens to track.
    pub fn create_rpc_client_with_tokens(
        &self,
        etherscan_api_key: &str,
        tokens: Vec<TokenMetadata>,
    ) -> Result<EvmChainClient> {
        let http_url = self
            .rpc
            .iter()
            .find(|url| url.starts_with("http") && !url.contains('{')) // skip urls that require templating
            .context(format!("No usable RPC HTTP URL for chain: {}", self.name))?;

        let url = Url::parse(http_url)?;
        let rpc_client: ReqwestClient = ClientBuilder::default().http(url);
        let etherscan = EtherscanClient::new(Chain::from_id(self.chain_id), etherscan_api_key)?;

        Ok(EvmChainClient::new(
            self.name.clone(),
            self.native_currency.clone(),
            Arc::new(rpc_client),
            Arc::new(etherscan),
            tokens,
        ))
    }
}
