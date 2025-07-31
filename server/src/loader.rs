use std::{fs, path::Path};

use anyhow::Result;
use tracing::info;

use crate::types::TokenMetadata;

pub fn load_tokens_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<TokenMetadata>> {
    let path_ref = path.as_ref();

    let data = fs::read_to_string(path_ref)
        .map_err(|e| anyhow::anyhow!("Failed to read file {:?}: {}", path_ref, e))?;

    let tokens: Vec<TokenMetadata> = serde_json::from_str(&data)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON in {:?}: {}", path_ref, e))?;

    info!("Loaded {} tokens from {:?}", tokens.len(), path_ref);
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use std::{io::Write, str::FromStr};

    use alloy::primitives::Address;
    use tempfile::NamedTempFile;

    use crate::loader::load_tokens_from_file;

    #[test]
    fn test_load_tokens_from_file() {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");

        let json = r#"
    [
        {
            "symbol": "USDC",
            "address": "0x1234567890abcdef1234567890abcdef12345678",
            "decimals": 6,
            "name": "USD Coin"
        },
        {
            "symbol": "DAI",
            "address": "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd",
            "decimals": 18,
            "name": "Dai Stablecoin"
        }
    ]
    "#;

        file.write_all(json.trim().as_bytes())
            .expect("Failed to write JSON");

        let tokens = load_tokens_from_file(file.path()).expect("Failed to load tokens");
        assert_eq!(tokens.len(), 2);

        let usdc = tokens
            .iter()
            .find(|t| t.symbol == "USDC")
            .expect("Missing USDC");
        assert_eq!(
            usdc.address,
            Address::from_str("0x1234567890abcdef1234567890abcdef12345678").unwrap()
        );
        assert_eq!(usdc.decimals, 6);
        assert_eq!(usdc.name, "USD Coin");

        let dai = tokens
            .iter()
            .find(|t| t.symbol == "DAI")
            .expect("Missing DAI");
        assert_eq!(
            dai.address,
            Address::from_str("0xabcdefabcdefabcdefabcdefabcdefabcdefabcd").unwrap()
        );
        assert_eq!(dai.decimals, 18);
        assert_eq!(dai.name, "Dai Stablecoin");
    }
}
