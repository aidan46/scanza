#![warn(unused_crate_dependencies)]
use std::{collections::HashMap, fs, net::SocketAddr, path::Path};

use anyhow::Result;
use axum::{Router, routing::get};
use multichain_client::{ChainMetaData, EvmClientRegistry};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::routes::routes;

mod routes;
mod types;

async fn root() -> &'static str {
    "Welcome to Scanza"
}

fn init_app_state<P: AsRef<Path>>(chain_list: P) -> Result<AppState> {
    let etherscan_api_key = dotenvy::var("ETHERSCAN_API_KEY")?;
    let registry = create_registry(read_chains_from_json(chain_list)?, &etherscan_api_key)?;

    Ok(AppState { registry })
}

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

fn init_router(state: AppState) -> Result<Router> {
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
    etherscan_api_key: &str,
) -> Result<EvmClientRegistry> {
    let mut client_map = HashMap::new();
    for chain in chains.iter() {
        if let Ok(mut client) = chain.create_rpc_client(etherscan_api_key) {
            info!("✅ Created client for {}", chain.name);
            let path_str = format!("config/{}-tokens.json", chain.short_name);
            let path = Path::new(&path_str);
            if path.exists() {
                client.add_tokens_from_file(path)?;
                info!("✅ Added tokens for {}", chain.name);
            }
            client_map.insert(chain.short_name.clone(), client);
        }
    }

    Ok(EvmClientRegistry::new(client_map))
}

#[derive(Clone)]
pub struct AppState {
    pub registry: EvmClientRegistry,
}

#[tokio::main]
async fn main() -> Result<()> {
    // load .env file
    dotenvy::dotenv().ok();

    // initialize tracing
    init_tracing()?;

    // initialize app state
    let state = init_app_state("config/chains.json")?;

    // initialize router
    let app = init_router(state)?;

    // bind to localhost:3000
    let bind_address = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(bind_address).await?;
    info!("🚀 Server running at http://{bind_address}");

    // serve
    axum::serve(listener, app).await?;

    Ok(())
}
