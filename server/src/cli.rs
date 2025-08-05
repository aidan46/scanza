use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    #[clap(long, default_value = "config/")]
    pub token_folder: PathBuf,
    #[clap(long, default_value = "config/chains.json")]
    pub chains: PathBuf,
}
