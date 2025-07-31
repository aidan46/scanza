use std::{fmt::Display, sync::Arc};

use alloy::{
    rpc::client::{ClientBuilder, ReqwestClient},
    transports::http::reqwest::Url,
};
use alloy_chains::Chain as AlloyChain;
use anyhow::Result;
use foundry_block_explorers::Client as EtherscanClient;
use serde::{Deserialize, Serialize};

use crate::{loader::load_tokens_from_folder, types::TokenMetadata};

#[macro_export]
macro_rules! define_clients {
    ($($variant:ident),+ $(,)?) => {
        // Generate the enum
        #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
        #[serde(rename_all = "lowercase")]
        pub enum Chains {
            $($variant),+
        }

        impl Display for Chains {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        $(
            #[derive(Clone)]
            pub struct $variant {
                pub name: Chains,
                pub client: Arc<ReqwestClient>,
                pub etherscan: Arc<EtherscanClient>,
                pub tokens: Vec<TokenMetadata>,
            }

            impl $variant {
                pub fn new(
                    alloy_chain: AlloyChain,
                    rpc_url: &str,
                    etherscan_api_key: &str,
                    token_dir: &str,
                ) -> Result<Self> {
                    let client: ReqwestClient = ClientBuilder::default().http(Url::parse(rpc_url)?);
                    let etherscan = EtherscanClient::new(alloy_chain, etherscan_api_key)?;
                    let tokens = load_tokens_from_folder(token_dir)?;

                    Ok(Self {
                        name: Chains::$variant,
                        client: Arc::new(client),
                        etherscan: Arc::new(etherscan),
                        tokens,
                    })
                }
            }

            impl ChainClient for $variant {
                fn name(&self) -> &Chains {
                    &self.name
                }

                fn client(&self) -> &ReqwestClient {
                    &self.client
                }

                fn etherscan(&self) -> &EtherscanClient {
                    &self.etherscan
                }

                fn tokens(&self) -> &[TokenMetadata] {
                    &self.tokens
                }
            }
        )+
    };
}

define_clients!(Ethereum);

pub trait ChainClient: Send + Sync {
    fn name(&self) -> &Chains;
    fn client(&self) -> &ReqwestClient;
    fn etherscan(&self) -> &EtherscanClient;
    fn tokens(&self) -> &[TokenMetadata];
}
