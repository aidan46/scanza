#![warn(unused_crate_dependencies)]
use std::net::SocketAddr;

use alloy::{
    rpc::client::{ClientBuilder, ReqwestClient},
    transports::http::reqwest::Url,
};
use anyhow::Result;
use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{loader::load_tokens_from_folder, routes::routes, types::TokenMetadata};

mod loader;
mod routes;
mod types;

async fn root() -> &'static str {
    "Welcome to Scanza"
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub client: ReqwestClient,
    pub tokens: Vec<TokenMetadata>,
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

    let tokens = load_tokens_from_folder("tokens/eth")?;

    // get rpc url from env
    let rpc_url = dotenvy::var("RPC_URL").expect("ETH_RPC_URL must be set in .env or environment");
    info!("RPC_URL: {rpc_url}");

    // create client
    let client: ReqwestClient = ClientBuilder::default().http(Url::parse(&rpc_url)?);

    // create state
    let state = AppState { client, tokens };

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
