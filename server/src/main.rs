#![warn(unused_crate_dependencies)]
use std::net::SocketAddr;

use anyhow::Result;
use clap::Parser;
use mongodb::Client as MongoClient;
use multichain_client::EvmClientRegistry;
use tokio::net::TcpListener;
use tracing::info;

use crate::{
    cli::Cli,
    init::{init_app_state, init_router, init_tracing},
};

mod cli;
mod init;
mod routes;

async fn root() -> &'static str {
    "Welcome to Scanza"
}

#[derive(Clone)]
pub struct AppState {
    pub registry: EvmClientRegistry,
    pub mongodb: MongoClient,
}

#[tokio::main]
async fn main() -> Result<()> {
    // load .env file
    dotenvy::dotenv().ok();

    // initialize tracing
    init_tracing()?;

    let cli = Cli::parse();

    // initialize app state
    let state = init_app_state(cli.chains, cli.token_folder).await?;

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
