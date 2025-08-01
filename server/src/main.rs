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

fn init_app_state() -> Result<AppState> {
    let mut chains: HashMap<Chains, Arc<dyn ChainClient>> = HashMap::new();

    // Ethereum
    let eth = Ethereum::new(
        Chain::mainnet(),
        &dotenvy::var("ETHEREUM_RPC_URL")?,
        &dotenvy::var("ETHERSCAN_API_KEY")?,
        "tokens/eth.json",
    )?;
    chains.insert(Chains::Ethereum, Arc::new(eth));

    Ok(AppState { chains })
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

#[derive(Clone)]
pub struct AppState {
    pub chains: HashMap<Chains, Arc<dyn ChainClient>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // load .env file
    dotenvy::dotenv().ok();

    // initialize tracing
    init_tracing()?;

    // initialize app state
    let state = init_app_state()?;

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
