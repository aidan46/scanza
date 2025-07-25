use std::{fs, path::Path, str::FromStr};

use alloy::primitives::Address;
use tracing::{info, warn};

use crate::types::{RawTokenMetadata, TokenMetadata};

pub fn load_tokens_from_folder<P: AsRef<Path>>(folder: P) -> Vec<TokenMetadata> {
    let mut tokens = vec![];

    for entry in fs::read_dir(folder).expect("Failed to read token directory") {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                warn!("Skipping unreadable file: {e}");
                continue;
            }
        };
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        let data = match fs::read_to_string(&path) {
            Ok(d) => d,
            Err(e) => {
                warn!("Failed to read file {:?}: {e}", path);
                continue;
            }
        };

        let raw: RawTokenMetadata = match serde_json::from_str(&data) {
            Ok(t) => t,
            Err(e) => {
                warn!("Failed to parse JSON {:?}: {e}", path);
                continue;
            }
        };

        let address = match Address::from_str(&raw.address) {
            Ok(addr) => addr,
            Err(e) => {
                warn!("Invalid address in {:?}: {e}", path);
                continue;
            }
        };

        tokens.push(TokenMetadata {
            symbol: raw.symbol,
            address,
            decimals: raw.decimals,
        });
    }

    info!("Loaded {} tokens", tokens.len());
    tokens
}
