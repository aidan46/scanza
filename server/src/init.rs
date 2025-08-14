use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use axum::{Router, routing::get};
use mongodb::Client as MongoClient;
use multichain_client::{ChainMetaData, EvmClientRegistry};
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{AppState, root, routes::routes};

pub async fn init_app_state<P: AsRef<Path>>(
    chain_list: P,
    token_folder: PathBuf,
) -> Result<AppState> {
    // setup registry
    let etherscan_api_key = dotenvy::var("ETHERSCAN_API_KEY")?;
    let chains = read_chains_from_json(chain_list)?;
    let registry = create_registry(chains, token_folder, &etherscan_api_key)?;

    // setup mongodb
    let uri = dotenvy::var("MONGODB_URI")?;
    let mongodb = MongoClient::with_uri_str(uri).await?;

    Ok(AppState { registry, mongodb })
}

pub fn init_tracing() -> Result<()> {
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

pub fn init_router(state: AppState) -> Result<Router> {
    // cors layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // build the router
    Ok(Router::new()
        .route("/", get(root))
        .merge(routes(state))
        .layer(cors))
}

fn read_chains_from_json<P: AsRef<Path>>(path: P) -> Result<Vec<ChainMetaData>> {
    let data = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&data)?)
}

fn create_registry(
    chains: Vec<ChainMetaData>,
    token_folder: PathBuf,
    etherscan_api_key: &str,
) -> Result<EvmClientRegistry> {
    let mut client_map = HashMap::new();
    for chain in chains.iter() {
        if let Ok(mut client) = chain.create_rpc_client(etherscan_api_key) {
            info!("✅ Created client for {}", chain.name);
            let path = token_folder.join(format!("{}-tokens.json", chain.short_name));
            if path.exists() {
                client.add_tokens_from_file(path)?;
                info!(
                    "✅ Added {} tokens for {}",
                    client.tokens().len(),
                    chain.name
                );
            }
            client_map.insert(chain.short_name.clone(), client);
        }
    }

    Ok(EvmClientRegistry::new(client_map))
}
