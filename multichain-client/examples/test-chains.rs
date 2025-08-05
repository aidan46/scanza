use std::{collections::HashMap, fs, path::Path};

use alloy::primitives::{Address, address};
use anyhow::Result;
use foundry_block_explorers::account::GenesisOption;
use multichain_client::{ChainMetaData, EvmClientRegistry};
use tracing::{debug, info, level_filters::LevelFilter, warn};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn init_tracing() -> Result<()> {
    // set up logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env()?,
        )
        .init();
    Ok(())
}

fn read_chains(path: &str) -> Result<Vec<ChainMetaData>> {
    let data = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&data)?)
}

fn create_clients(
    chains: Vec<ChainMetaData>,
    etherscan_api_key: &str,
) -> Result<EvmClientRegistry> {
    let mut client_map = HashMap::new();
    for chain in chains.iter() {
        match chain.create_rpc_client(etherscan_api_key) {
            Ok(mut client) => {
                debug!("✅ Created client for {}", chain.name);
                let path_str = format!("{}-tokens.json", chain.short_name);
                let path = Path::new(&path_str);
                if path.exists() {
                    client.add_tokens_from_file(path)?;
                    debug!("✅ Added tokens for {}", chain.name);
                }
                client_map.insert(chain.short_name.clone(), client);
            }
            Err(e) => warn!("❌ Failed for {}: {}", chain.name, e),
        }
    }

    Ok(EvmClientRegistry::new(client_map))
}

#[allow(unused)]
async fn print_native_balances(chain_clients: &EvmClientRegistry, address: Address) -> Result<()> {
    let native_balances = chain_clients.get_native_balances(address).await?;

    for (chain, b) in native_balances.iter() {
        info!("Native balance for {address} on {chain}: {b}");
    }

    Ok(())
}

#[allow(unused)]
async fn print_token_balances(chain_clients: &EvmClientRegistry, address: Address) -> Result<()> {
    let token_balances = chain_clients.get_token_balances(address).await;

    for (chain, b) in token_balances.iter() {
        if !b.is_empty() {
            info!("Token balances for {address} on {chain}");
            b.iter().for_each(|b| {
                info!("Token {} = {}", b.token.name, b.balance);
            });
        }
    }

    Ok(())
}

#[allow(unused)]
async fn print_transactions(
    chain_clients: &EvmClientRegistry,
    address: Address,
    page: u64,
    offset: u64,
) -> Result<()> {
    let transactions = chain_clients
        .get_transactions(address, page, offset)
        .await?;
    transactions.iter().for_each(|(chain, transactions)| {
        info!("Transactions for {address} on {chain}:");
        transactions.iter().for_each(|tx| {
            if let GenesisOption::Some(hash) = tx.hash {
                info!("{hash:#?}")
            }
        });
    });
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing()?;
    dotenvy::dotenv().ok();
    let etherscan_api_key = dotenvy::var("ETHERSCAN_API_KEY")?;
    let path = "test-chains.json";
    let chains = read_chains(path)?;
    let chain_clients = create_clients(chains, &etherscan_api_key)?;

    let address = address!("0xCFFAd3200574698b78f32232aa9D63eABD290703");
    // print_native_balances(&chain_clients, address).await?;
    print_token_balances(&chain_clients, address).await?;
    // print_transactions(&chain_clients, address, 1, 1).await?;

    Ok(())
}
