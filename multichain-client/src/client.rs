//! Defines the `EvmChainClient` and `EvmChainClients` types for interacting with EVM-compatible blockchains.
//!
//! Includes methods for fetching native and token balances, transactions, and dynamically extending token metadata.

use std::{collections::HashMap, fs, path::Path, sync::Arc};

use alloy::{
    hex,
    primitives::{Address, U256},
    rpc::client::ReqwestClient,
    sol,
    sol_types::SolCall,
};
use anyhow::{Error, Result};
use foundry_block_explorers::{
    Client as EtherscanClient,
    account::{NormalTransaction, Sort, TxListParams},
};
use futures::future::{join_all, try_join_all};
use tracing::{info, warn};

use crate::{
    ChainMetaData,
    metadata::{NativeCurrency, TokenBalance, TokenMetadata},
};

sol! {
    function balanceOf(address) external view returns (uint256);
}

/// A client for a single EVM-compatible chain.
/// Holds RPC and Etherscan clients, native currency metadata, and token metadata.
#[derive(Clone)]
pub struct EvmChainClient {
    metadata: ChainMetaData,
    rpc_client: Arc<ReqwestClient>,
    etherscan: Arc<EtherscanClient>,
    tokens: Vec<TokenMetadata>,
}

impl EvmChainClient {
    /// Constructs a new `EvmChainClient`.
    pub fn new(
        metadata: ChainMetaData,
        rpc_client: Arc<ReqwestClient>,
        etherscan: Arc<EtherscanClient>,
        tokens: Vec<TokenMetadata>,
    ) -> Self {
        Self {
            metadata,
            rpc_client,
            etherscan,
            tokens,
        }
    }

    /// Returns the chain [`ChainMetaData`].
    pub fn metadata(&self) -> &ChainMetaData {
        &self.metadata
    }

    /// Returns the RPC client for interacting with the chain.
    pub fn rpc_client(&self) -> &Arc<ReqwestClient> {
        &self.rpc_client
    }

    /// Returns the Etherscan client for fetching transactions.
    pub fn etherscan(&self) -> &Arc<EtherscanClient> {
        &self.etherscan
    }

    /// Returns the list of tracked tokens for the chain.
    pub fn tokens(&self) -> &[TokenMetadata] {
        &self.tokens
    }

    /// Returns the native currency metadata (e.g., ETH or MATIC).
    pub fn native_currency(&self) -> &NativeCurrency {
        &self.metadata.native_currency
    }

    /// Appends tokens from a JSON file to the internal token list.
    ///
    /// The file should be formatted as an array of:
    /// `[{ "address": "0x...", "name": "Token", "symbol": "SYM", "decimals": 18 }]`
    pub fn add_tokens_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let data = fs::read_to_string(path)?;
        let tokens: Vec<TokenMetadata> = serde_json::from_str(&data)?;

        self.tokens.extend_from_slice(&tokens);
        Ok(())
    }

    /// Fetches the native balance (ETH/MATIC/etc) of an address.
    pub async fn get_native_balance(&self, address: Address) -> Result<U256> {
        match self
            .rpc_client
            .request::<_, U256>("eth_getBalance", (address, "latest"))
            .await
        {
            Ok(balance) => Ok(balance),
            Err(e) => Err(anyhow::anyhow!("Failed to fetch native balance: {}", e)),
        }
    }

    /// Fetches the balance for each tracked token for the given address.
    ///
    /// Only returns tokens with non-zero balances.
    pub async fn get_token_balances(&self, address: Address) -> Vec<TokenBalance> {
        let futures = self.tokens.iter().map(|token| {
            let token = token.clone();
            let client = self.rpc_client.clone();
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

    /// Fetches transactions for the given address using Etherscan.
    ///
    /// Over-fetches one extra to detect whether more pages exist.
    ///
    /// Returns `(Vec<NormalTransaction>, has_more)`.
    pub async fn get_transactions(
        &self,
        address: Address,
        page: u64,
        offset: u64,
    ) -> Result<(Vec<NormalTransaction>, bool)> {
        let overget_offset = offset + 1;

        let params = TxListParams {
            start_block: 0,
            end_block: u64::MAX,
            page,
            offset: overget_offset,
            sort: Sort::Desc,
        };

        let fetched = self
            .etherscan
            .get_transactions(&address, Some(params))
            .await?;
        let has_more = fetched.len() as u64 > offset;

        let trimmed = if has_more {
            fetched.into_iter().take(offset as usize).collect()
        } else {
            fetched
        };

        Ok((trimmed, has_more))
    }
}

/// Wrapper around a map of multiple `EvmChainClient`s, keyed by chain name.
#[derive(Clone)]
pub struct EvmClientRegistry(HashMap<String, EvmChainClient>);

impl EvmClientRegistry {
    /// Constructs a new `EvmChainClients` from a map of chain names to clients.
    pub fn new(map: HashMap<String, EvmChainClient>) -> Self {
        Self(map)
    }

    /// Fetches native balances for the given address across all chains.
    ///
    /// Returns a map from chain name to balance (U256).
    pub async fn get_native_balances(&self, address: Address) -> Result<HashMap<String, U256>> {
        let futures = self.0.iter().map(|(chain, client)| {
            let chain = chain.clone();
            async move {
                info!("Fetching native balance for {chain}");
                match client.get_native_balance(address).await {
                    Ok(balance) => Ok::<(String, U256), Error>((chain, balance)),
                    Err(e) => {
                        warn!("Failed to fetch native balance for {chain}: {e}");
                        Ok::<(String, U256), Error>((chain, U256::ZERO))
                    }
                }
            }
        });

        let results = try_join_all(futures).await?;
        Ok(results.into_iter().collect())
    }

    /// Fetches token balances for the given address across all chains that have tokens defined.
    ///
    /// Returns a map from chain name to list of `TokenBalance`.
    pub async fn get_token_balances(&self, address: Address) -> HashMap<String, Vec<TokenBalance>> {
        let futures = self.0.iter().filter_map(|(chain, client)| {
            let chain = chain.clone();
            if !client.tokens().is_empty() {
                Some(async move {
                    info!("Fetching token balances for {chain}");
                    (chain, client.get_token_balances(address).await)
                })
            } else {
                None
            }
        });

        let results = join_all(futures).await;
        results.into_iter().collect()
    }

    /// Fetches transactions for the given address across all chains.
    ///
    /// Returns a map from chain name to list of `NormalTransaction`s.
    pub async fn get_transactions(
        &self,
        address: Address,
        page: u64,
        offset: u64,
    ) -> Result<HashMap<String, Vec<NormalTransaction>>> {
        let futures = self.0.iter().map(|(chain, client)| {
            let chain = chain.clone();

            async move {
                info!("Fetching transactions for {chain}");
                match client.get_transactions(address, page, offset).await {
                    Ok((txs, _)) => Ok::<(String, Vec<NormalTransaction>), Error>((chain, txs)),
                    Err(e) => {
                        warn!("Failed to fetch transactions for {chain}: {e}");
                        Ok::<(String, Vec<NormalTransaction>), Error>((chain, vec![]))
                    }
                }
            }
        });

        let results = try_join_all(futures).await?;
        Ok(results.into_iter().collect())
    }

    #[inline]
    pub fn inner(&self) -> &HashMap<String, EvmChainClient> {
        &self.0
    }

    #[inline]
    pub fn get(&self, chain: &str) -> Option<&EvmChainClient> {
        self.0.get(chain)
    }
}
