#![warn(unused_crate_dependencies)]
use std::net::SocketAddr;

use anyhow::Result;
use multichain_client::EvmClientRegistry;
use tokio::net::TcpListener;
use tracing::info;

use crate::init::{init_app_state, init_router, init_tracing};

mod init;
mod routes;
mod types;

async fn root() -> &'static str {
    "Welcome to Scanza"
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
