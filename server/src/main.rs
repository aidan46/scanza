#![warn(unused_crate_dependencies)]
use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use alloy_chains::Chain;
use anyhow::Result;
use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    chains::{ChainClient, Chains, Ethereum},
    routes::routes,
};

mod chains;
mod loader;
mod routes;
mod types;

async fn root() -> &'static str {
    "Welcome to Scanza"
}

#[derive(Clone)]
pub struct AppState {
    pub chains: HashMap<Chains, Arc<dyn ChainClient>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // load .env file
    dotenvy::dotenv().ok();
    // set up logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env()?,
        )
        .init();

    let mut chains: HashMap<Chains, Arc<dyn ChainClient>> = HashMap::new();

    let eth_chain = Ethereum::new(
        Chain::mainnet(),
        &dotenvy::var("ETHEREUM_RPC_URL")?,
        &dotenvy::var("ETHERSCAN_API_KEY")?,
        "tokens/eth.json",
    )?;
    chains.insert(Chains::Ethereum, Arc::new(eth_chain));

    // create state
    let state = AppState { chains };

    // cors layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // build the router
    let app = Router::new()
        .route("/", get(root))
        .merge(routes(state))
        .layer(cors);

    // bind to localhost:3000
    let bind_address = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(bind_address).await?;
    info!("🚀 Server running at http://{bind_address}");

    // serve
    axum::serve(listener, app).await?;

    Ok(())
}
